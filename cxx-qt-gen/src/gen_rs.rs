// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::ImplItemMethod;

use crate::extract::{QObject, QtTypes};
use crate::utils::{is_type_ident_ptr, type_to_namespace};

/// A trait which we implement on QtTypes allowing retrieval of attributes of the enum value.
trait RustType {
    /// Whether this type is a Pin<T>
    fn is_pin(&self) -> bool;
    /// Whether this type is a reference
    fn is_ref(&self) -> bool;
    /// Whether this type is a this (eg the T in Pin<T>)
    fn is_this(&self) -> bool;
    /// The ident of the type when used as a parameter on a function
    fn param_type_ident(&self) -> Ident;
    /// The full type for the parameter. Can be used Rust code outside cxx::bridge.
    fn full_param_type(&self) -> TokenStream;
}

impl RustType for QtTypes {
    /// Whether this type is a Pin<T>, this is used to find args to rename Pin to std::pin::Pin
    fn is_pin(&self) -> bool {
        match self {
            Self::Pin { .. } => true,
            _others => false,
        }
    }

    /// Whether this type should be a reference when used in Rust methods
    fn is_ref(&self) -> bool {
        match self {
            Self::Pin { .. } => unreachable!(),
            Self::Str | Self::String => true,
            _others => false,
        }
    }

    /// Whether this type is_this, this is used to determine if the type needs to be rewritten
    fn is_this(&self) -> bool {
        match self {
            Self::Pin { is_this, .. } => is_this == &true,
            _others => false,
        }
    }

    /// The ident of the type when used as a parameter on a function
    fn param_type_ident(&self) -> Ident {
        match self {
            Self::I32 => format_ident!("i32"),
            Self::Pin { .. } => unreachable!(),
            // Pointer types do not use this function (TODO: yet?)
            Self::Ptr { .. } => unreachable!(),
            Self::Str | Self::String | Self::QString => format_ident!("QString"),
            _others => unreachable!(),
        }
    }

    /// The full type for the parameter. Can be used Rust code outside cxx::bridge.
    fn full_param_type(&self) -> TokenStream {
        match self {
            Self::I32 => quote! {i32},
            Self::Pin { .. } => unreachable!(),
            Self::Ptr { ident_str, .. } => {
                let ident = format_ident!("{}", ident_str);
                quote! {cxx::UniquePtr<ffi::#ident>}
            }
            Self::Str | Self::String | Self::QString => quote! {cxx_qt_lib::QString},
            _other => unreachable!(),
        }
    }
}

/// Generate Rust code that used CXX to interact with the C++ code generated for a QObject
pub fn generate_qobject_cxx(
    obj: &QObject,
    cpp_namespace_prefix: &[String],
) -> Result<TokenStream, TokenStream> {
    // Cache the original and rust class names, these are used multiple times later
    let class_name = &obj.ident;
    let rust_class_name = format_ident!("RustObj");

    // Build a snake version of the class name, this is used for rust method names
    //
    // TODO: Abstract this calculation to make it common to gen_rs and gen_cpp
    let ident_snake = class_name.to_string().to_case(Case::Snake);

    // Lists of functions we generate for the CXX bridge
    let mut cpp_functions = Vec::new();
    let mut cpp_types = HashSet::new();
    let mut rs_functions = Vec::new();

    // Closure which allows for adding a type to cpp functions but ensures no duplicates
    //
    // This is useful when the same external C++ type is used in multiple properties or invokables
    let cpp_types_push_unique = |cpp_functions: &mut Vec<TokenStream>,
                                 cpp_types: &mut HashSet<String>,
                                 ptr_class_name: &Ident,
                                 type_idents_ffi: Vec<Ident>|
     -> Result<_, TokenStream> {
        // Ensure that this type doesn't exist in our set already
        //
        // TODO: when we skip adding a type for ptr_class_name's that are the same but with
        // different type_idents, other parts of the system will likely fail.
        // This is likely a good place to catch the error.
        // Eg if we had moduleA::Object and moduleB::Object?
        if !cpp_types.contains(&ptr_class_name.to_string()) {
            // Build the namespace for our type
            let namespace = type_to_namespace(cpp_namespace_prefix, &type_idents_ffi)
                .map_err(|msg| {
                    syn::Error::new(
                        ptr_class_name.span(),
                        format!(
                            "Could not generate namespace with type idents {:#?}: {}",
                            type_idents_ffi, msg
                        ),
                    )
                    .to_compile_error()
                })?
                .join("::");
            // Add the type definition to the C++ part of the cxx bridge
            cpp_functions.push(quote! {
                #[namespace = #namespace]
                type #ptr_class_name = #(#type_idents_ffi)::*;
            });
            // Track that we have added this type
            cpp_types.insert(ptr_class_name.to_string());
        }

        Ok(())
    };
    // Closure which retrieves Object from crate::module::Object and swaps to crate::module::CppObj
    let type_idents_to_ptr_class_name_and_ffi_type = |type_idents: &Vec<Ident>| {
        // Build the class name of the pointer, eg Object in crate::module::Object
        //
        // We can assume that unwrap will work here as we have checked that type_idents is not empty
        let ptr_class_name = type_idents.last().unwrap().clone();

        // Swap the last type segment to be CppObj
        // so that crate::module::Object becomes crate::module::CppObj
        //
        // As we will generate a public type which points to the ffi type at the module level
        let mut type_idents_ffi = type_idents.clone();
        type_idents_ffi.pop();
        type_idents_ffi.push(format_ident!("CppObj"));

        (ptr_class_name, type_idents_ffi)
    };

    // Invokables are only added to extern rust side
    //
    // TODO: later support a cxx_qt_name attribute on invokables to allow for renaming
    // to a custom name for C++ or Rust side?
    for i in &obj.invokables {
        // Cache the ident and parameters as they are used multiple times later
        let ident = &i.ident.rust_ident;
        let ident_cpp_str = &i.ident.cpp_ident.to_string();
        let parameters = &i.parameters;

        // TODO: invokables need to also become freestanding functions that
        // take as input a reference to both the Rs class and the CppObject
        // inside a wrapper. The functions that are impl'ed on the Rs class
        // will then simply create the wrapper and call the free functions.
        //
        // As a first step we could maybe just add a `cpp: Pin<&mut CppObj>`
        // argument to invokables so that users can manually wrap it.

        // Determine if the invokable has any parameter
        if parameters.is_empty() {
            // Determine if there is a return type and if it's a reference
            if let Some(return_type) = &i.return_type {
                // Cache the return type
                let type_idents = &return_type.idents;

                // Determine if the return type is a ref
                if return_type.is_ref {
                    rs_functions.push(quote! {
                        #[cxx_name = #ident_cpp_str]
                        fn #ident(self: &#rust_class_name) -> &#(#type_idents)::*;
                    });
                } else {
                    rs_functions.push(quote! {
                        #[cxx_name = #ident_cpp_str]
                        fn #ident(self: &#rust_class_name) -> #(#type_idents)::*;
                    });
                }
            } else {
                rs_functions.push(quote! {
                    #[cxx_name = #ident_cpp_str]
                    fn #ident(self: &#rust_class_name);
                });
            }
        } else {
            // Build a list of quotes of the parameter name and type
            let mut parameters_quotes = Vec::new();
            for p in parameters {
                // Cache the name and type
                let ident = &p.ident;

                // If the type is Pin<T> then we need to change extract differently
                match &p.type_ident.qt_type {
                    QtTypes::Pin {
                        is_mut,
                        is_this,
                        type_idents,
                        ..
                    } => {
                        // If the Pin<T> is to our own type, then we refer to the C++ class name
                        // not the external crate (when it is a sub object)
                        if *is_this {
                            if *is_mut {
                                parameters_quotes.push(quote! { #ident: Pin<&mut #class_name> });
                            } else {
                                parameters_quotes.push(quote! { #ident: Pin<&#class_name> });
                            }
                        } else {
                            // Retrieve Object from crate::module::Object and swap to crate::module::CppObj
                            let (ptr_class_name, type_idents_ffi) =
                                type_idents_to_ptr_class_name_and_ffi_type(type_idents);

                            // Add type definition for the struct name we are a Pin for to the Rust bridge
                            //
                            // Ensure that we only do this once
                            cpp_types_push_unique(
                                &mut cpp_functions,
                                &mut cpp_types,
                                &ptr_class_name,
                                type_idents_ffi,
                            )?;

                            if *is_mut {
                                parameters_quotes
                                    .push(quote! { #ident: Pin<&mut #ptr_class_name> });
                            } else {
                                parameters_quotes.push(quote! { #ident: Pin<&#ptr_class_name> });
                            }
                        }
                    }
                    _others => {
                        let type_idents = &p.type_ident.idents;

                        // Determine if the type is a ref
                        if p.type_ident.is_ref {
                            parameters_quotes.push(quote! {
                                #ident: &#(#type_idents)::*
                            });
                        } else {
                            parameters_quotes.push(quote! {
                                #ident: #(#type_idents)::*
                            });
                        }
                    }
                };
            }

            // Determine if there is a return type and if it's a reference
            if let Some(return_type) = &i.return_type {
                // Cache the return type
                let type_idents = &return_type.idents;

                // Determine if the return type is a ref
                if return_type.is_ref {
                    rs_functions.push(quote! {
                        #[cxx_name = #ident_cpp_str]
                        fn #ident(self: &#rust_class_name, #(#parameters_quotes),*) -> &#(#type_idents)::*;
                    });
                } else {
                    rs_functions.push(quote! {
                        #[cxx_name = #ident_cpp_str]
                        fn #ident(self: &#rust_class_name, #(#parameters_quotes),*) -> #(#type_idents)::*;
                    });
                }
            } else {
                rs_functions.push(quote! {
                    #[cxx_name = #ident_cpp_str]
                    fn #ident(self: &#rust_class_name, #(#parameters_quotes),*);
                });
            }
        }
    }

    // Add getters/setters/notify from properties
    for property in &obj.properties {
        // Cache the type of the property
        let type_idents = &property.type_ident.idents;

        // cache the snake and pascal case
        let property_ident_snake = property.ident.rust_ident.to_string().to_case(Case::Snake);
        let property_ident_pascal = property.ident.rust_ident.to_string().to_case(Case::Pascal);

        // This type is a pointer, so special case the C++ functions and no Rust functions
        if is_type_ident_ptr(type_idents) {
            // Check that type_idents is not empty
            if type_idents.is_empty() {
                return Err(syn::Error::new(
                    property.ident.rust_ident.span(),
                    "Property type needs at least one type ident.",
                )
                .to_compile_error());
            }

            // Retrieve Object from crate::module::Object and swap to crate::module::CppObj
            let (ptr_class_name, type_idents_ffi) =
                type_idents_to_ptr_class_name_and_ffi_type(type_idents);

            // Add type definition for the class name we are a pointer for to the C++ bridge
            //
            // Ensure that we only do this once
            cpp_types_push_unique(
                &mut cpp_functions,
                &mut cpp_types,
                &ptr_class_name,
                type_idents_ffi,
            )?;

            // Build the C++ method declarations names
            let getter_str = format!("take_{}", property_ident_snake);
            let getter_cpp = format_ident!("take{}", property_ident_pascal);
            let setter_str = format!("give_{}", property_ident_snake);
            let setter_cpp = format_ident!("give{}", property_ident_pascal);

            // Add the getter and setter to C++ bridge
            cpp_functions.push(quote! {
                #[rust_name = #getter_str]
                fn #getter_cpp(self: Pin<&mut #class_name>) -> UniquePtr<#ptr_class_name>;
                #[rust_name = #setter_str]
                fn #setter_cpp(self: Pin<&mut #class_name>, value: UniquePtr<#ptr_class_name>);
            });
        // This is a normal primitive type so add Rust getters and setters
        } else {
            // Build the C++ method declarations names
            let getter_str = &property_ident_snake;
            let getter_cpp = format_ident!("get{}", property_ident_pascal);
            let setter_str = format!("set_{}", property_ident_snake);
            let setter_cpp = format_ident!("set{}", property_ident_pascal);

            let qt_type = &property.type_ident.qt_type;
            let param_type = qt_type.param_type_ident();
            let param_type = if qt_type.is_ref() {
                quote! {&#param_type}
            } else {
                quote! {#param_type}
            };

            // Add the getter and setter to C++ bridge
            cpp_functions.push(quote! {
                #[rust_name = #getter_str]
                fn #getter_cpp(self: &#class_name) -> #param_type;
                #[rust_name = #setter_str]
                fn #setter_cpp(self: Pin<&mut #class_name>, value: #param_type);
            });
        }
    }

    // Define a function to handle update requests if we have one
    let handle_update_request = if obj.handle_updates_impl.is_some() {
        quote! {
            #[cxx_name = "handleUpdateRequest"]
            fn call_handle_update_request(self: &mut #rust_class_name, cpp: Pin<&mut #class_name>);
        }
    } else {
        quote! {}
    };

    // Define a function to handle property changes if we have one
    let handle_property_change = if obj.handle_property_change_impl.is_some() {
        quote! {
            #[cxx_name = "handlePropertyChange"]
            fn call_handle_property_change(self: &mut #rust_class_name, cpp: Pin<&mut #class_name>, property: Property);
        }
    } else {
        quote! {}
    };

    // Build the methods to create the class
    let new_object_ident_cpp = format_ident!("new{}", class_name);
    let new_object_rust_str = format!("new_{}", class_name);
    let create_object_ident = format_ident!("create_{}_rs", ident_snake);
    let create_object_cpp_str = create_object_ident.to_string().to_case(Case::Camel);
    let initialise_object_ident = format_ident!("initialise_{}_cpp", ident_snake);
    let initialise_object_cpp_str = initialise_object_ident.to_string().to_case(Case::Camel);

    // Build the import path for the C++ header
    let import_path = format!("cxx-qt-gen/include/{}.h", ident_snake);

    // Generate an enum representing all the properties that the object has
    let property_enum = generate_property_enum(obj);

    // TODO: ideally we only want to add the "type QString = cxx_qt_lib::QString;"
    // if we actually generate some code that uses QString.

    // Build the namespace string, rust::module
    let namespace = obj.namespace.join("::");

    // Build the CXX bridge
    let output = quote! {
        #[cxx::bridge(namespace = #namespace)]
        mod ffi {
            #property_enum

            unsafe extern "C++" {
                include!(#import_path);

                type #class_name;

                #[namespace = ""]
                type QString = cxx_qt_lib::QString;

                #(#cpp_functions)*

                #[rust_name = #new_object_rust_str]
                fn #new_object_ident_cpp() -> UniquePtr<#class_name>;
            }

            extern "Rust" {
                type #rust_class_name;

                #(#rs_functions)*

                #[cxx_name = #create_object_cpp_str]
                fn #create_object_ident() -> Box<#rust_class_name>;

                #[cxx_name = #initialise_object_cpp_str]
                fn #initialise_object_ident(cpp: Pin<&mut #class_name>);

                #handle_update_request
                #handle_property_change
            }
        }

        pub type CppObj = ffi::#class_name;
        pub type Property = ffi::Property;
    };

    Ok(output.into_token_stream())
}

/// Generate a Rust function that heap constructs the Rust object corresponding to the QObject
fn generate_rust_object_creator(obj: &QObject) -> Result<TokenStream, TokenStream> {
    // Cache the rust class name, this is used multiple times later
    let rust_class_name = format_ident!("RustObj");

    // Build the ident as snake case, then build the rust creator method
    //
    // TODO: simplify this function name now that we have namespaces
    let ident_snake = &obj.ident.to_string().to_case(Case::Snake);
    let fn_ident = format_ident!("create_{}_rs", ident_snake);

    // We assume that RustObj implements Default otherwise it can't be constructed from QML
    // TODO: throw an error if we can determine this to not be the case
    let output = quote! {
        fn #fn_ident() -> std::boxed::Box<#rust_class_name> {
            std::default::Default::default()
        }
    };

    Ok(output.into_token_stream())
}

/// Generate a Rust function that initialises a QObject with the values from Data::default()
fn generate_cpp_object_initialiser(obj: &QObject) -> TokenStream {
    let data_class_name = &obj.original_data_struct.ident;

    // Build the ident as snake case, then build the rust creator method
    //
    // TODO: Simplify things to  initialiseCpp, createRs, newCppObject etc. now that we have namespaces
    let ident_snake = &obj.ident.to_string().to_case(Case::Snake);
    let fn_ident = format_ident!("initialise_{}_cpp", ident_snake);

    // We assume that all Data classes implement default
    let output = quote! {
        fn #fn_ident(cpp: std::pin::Pin<&mut CppObj>) {
            let mut wrapper = CppObjWrapper::new(cpp);
            wrapper.grab_values_from_data(&#data_class_name::default());
        }
    };

    output.into_token_stream()
}

/// Generate an enum representing all the properties that a QObject has
fn generate_property_enum(obj: &QObject) -> TokenStream {
    let properties = obj.properties.iter().map(|property| {
        let ident_str = &property.ident.rust_ident.to_string();
        let ident = format_ident!("{}", ident_str.to_case(Case::Pascal));
        quote! { #ident }
    });

    quote! {
        enum Property {
            #(#properties),*
        }
    }
}

fn generate_property_methods_rs(obj: &QObject) -> Result<Vec<TokenStream>, TokenStream> {
    // Build a list of property methods impls
    let mut property_methods = Vec::new();

    for property in &obj.properties {
        let qt_type = &property.type_ident.qt_type;
        let param_type = qt_type.full_param_type();
        let param_type = if qt_type.is_ref() {
            quote! {&#param_type}
        } else {
            quote! {#param_type}
        };

        let cpp_getter_ident = &property.getter.as_ref().unwrap().rust_ident;
        let cpp_setter_ident = &property.setter.as_ref().unwrap().rust_ident;

        if is_type_ident_ptr(&property.type_ident.idents) {
            let ident = &property.ident.rust_ident;
            let take_ident = format_ident!("take_{}", ident);
            let give_ident = format_ident!("give_{}", ident);

            property_methods.push(quote! {
                pub fn #take_ident(&mut self) -> #param_type {
                    self.cpp.as_mut().#take_ident()
                }
            });

            property_methods.push(quote! {
                pub fn #give_ident(&mut self, value: #param_type) {
                    self.cpp.as_mut().#give_ident(value);
                }
            });
        } else {
            if let Some(getter) = &property.getter {
                // Generate a getter using the rust ident
                let getter_ident = &getter.rust_ident;

                property_methods.push(quote! {
                    pub fn #getter_ident(&self) -> #param_type {
                        self.cpp.#cpp_getter_ident()
                    }
                });
            }

            if let Some(setter) = &property.setter {
                // Generate a setter using the rust ident
                let setter_ident = &setter.rust_ident;

                property_methods.push(quote! {
                    pub fn #setter_ident(&mut self, value: #param_type) {
                        self.cpp.as_mut().#cpp_setter_ident(value);
                    }
                });
            }
        }
    }

    Ok(property_methods)
}

fn is_field_ptr(field: &syn::Field) -> bool {
    // Determine the type of the field
    let ty_path;
    match &field.ty {
        syn::Type::Path(path) => {
            ty_path = path;
        }
        syn::Type::Reference(syn::TypeReference { elem, .. }) => {
            if let syn::Type::Path(path) = &**elem {
                ty_path = path;
            } else {
                // Unknown type, just ignore so we pass through
                return true;
            }
        }
        _others => {
            // Unknown type, just ignore so we pass through
            return true;
        }
    }

    // Filter any fields that have a type which is a pointer
    !is_type_ident_ptr(
        &ty_path
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_owned())
            .collect::<Vec<syn::Ident>>(),
    )
}

/// Return the fields of the struct filtered by the given closure
fn filter_fields<F: Fn(&syn::Field) -> bool>(
    original_struct: &syn::ItemStruct,
    filter_closure: F,
) -> Vec<&syn::Field> {
    // Filter the fields of the struct to remove any pointer fields
    // as they are instead stored in the C++ object and not owned by the rust side
    original_struct
        .fields
        .iter()
        .filter(|field| filter_closure(field))
        .collect::<Vec<&syn::Field>>()
}

/// Builds a struct with th given new fields
fn build_struct_with_fields(
    original_struct: &syn::ItemStruct,
    new_fields: &[&syn::Field],
) -> TokenStream {
    // Capture the attributes, generics, visibility as local vars so they can be used by quote
    let original_attributes = &original_struct.attrs;
    let original_generics = &original_struct.generics;
    let original_visibility = &original_struct.vis;
    let struct_ident = &original_struct.ident;

    // Finally build the renamed struct
    //
    // If there are no fields then use semi-colon instead of brackets
    if new_fields.is_empty() {
        quote! {
            #(#original_attributes)*
            #original_visibility struct #struct_ident #original_generics;
        }
    } else {
        quote! {
            #(#original_attributes)*
            #original_visibility struct #struct_ident #original_generics {
                #(#new_fields),*
            }
        }
    }
}

/// Add std::pin to Pin arguments in a method
fn fix_pin_path(method: &mut ImplItemMethod, pin_args: &[usize]) -> Result<(), TokenStream> {
    // Rewrite args
    method
        .sig
        .inputs
        .iter_mut()
        // We can skip self as that's not in our parameters above
        //
        // TODO: note this assumes that the first argument is self
        .skip(1)
        // We only want to rewrite pinned args
        .enumerate()
        .filter(|(index, _)| pin_args.contains(index))
        // Rewrite the type
        //
        // We add std::pin to Pin, and swap the last type inside a Pin<&a::T> from T to CppObj
        .map(|(_, item)| {
            if let syn::FnArg::Typed(syn::PatType { ref mut ty, .. }) = item {
                let ty_path;

                match ty.as_mut() {
                    syn::Type::Path(ref mut path) => {
                        ty_path = path;
                    }
                    // TODO: do we support TypeReference for &Pin<T>?
                    // Type::Reference(TypeReference { elem, .. }) => {
                    //     // If the type is a path then extract it and mark is_ref
                    //     if let Type::Path(path) = &**elem {
                    //         ty_path = path;
                    //     }
                    // }
                    //
                    _others => {
                        return Err(syn::Error::new(
                            item.span(),
                            "Pin<T> argument must be a path, we do not support reference yet",
                        )
                        .to_compile_error())
                    }
                }

                // Add std::pin to Pin
                ty_path.path.segments.insert(0, format_ident!("std").into());
                ty_path.path.segments.insert(1, format_ident!("pin").into());

                // From a::Pin<&b::T> we want Pin<&b::T> (we ultimately want to get to T)
                if let Some(segment) = ty_path.path.segments.last_mut() {
                    // From Pin<&b::T> we want &b::T
                    if let syn::PathArguments::AngleBracketed(arguments) = &mut segment.arguments {
                        // From &b::T we want b::T
                        //
                        // TODO: do we need to support non reference? or will it always be reference?
                        if let Some(syn::GenericArgument::Type(syn::Type::Reference(
                            syn::TypeReference { elem, .. },
                        ))) = arguments.args.first_mut()
                        {
                            if let syn::Type::Path(arg_path) = &mut **elem {
                                // From b::T we want T
                                if let Some(segment) = arg_path.path.segments.last_mut() {
                                    // Always set last type in Pin<T> to CppObj
                                    segment.ident = format_ident!("CppObj");
                                }
                            }
                        }
                    }
                }
            }
            Ok(item)
        })
        .collect::<Result<Vec<_>, TokenStream>>()?;

    Ok(())
}

/// Add std::pin to Pin and cxx_qt_lib::qstring to QString arguments for all impl methods
fn fix_impl_methods(impl_: &syn::ItemImpl) -> Result<TokenStream, TokenStream> {
    let mut cloned = impl_.to_owned();

    for mut item in &mut cloned.items {
        if let syn::ImplItem::Method(m) = &mut item {
            *m = fix_method_params(m)?;
        } else {
            return Err(
                syn::Error::new(item.span(), "Only methods are supported.").to_compile_error()
            );
        }
    }

    Ok(quote! { #cloned })
}

/// Add std::pin to Pin and cxx_qt_lib::qstring to QString arguments
fn fix_method_params(method: &ImplItemMethod) -> Result<ImplItemMethod, TokenStream> {
    let mut method = method.clone();

    // TODO: update this once extract_invokables is split in two
    let invokable = crate::extract::extract_invokable(&method, &["".to_owned()])?;

    // Find which arguments are using Pin<T>
    let pin_args = invokable
        .parameters
        .iter()
        .enumerate()
        .filter(|(_, parameter)| parameter.type_ident.qt_type.is_pin())
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

    let qstring_args = invokable
        .parameters
        .iter()
        .enumerate()
        .filter(|(_, parameter)| parameter.type_ident.qt_type == QtTypes::QString)
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

    if !pin_args.is_empty() {
        fix_pin_path(&mut method, &pin_args)?;
    }

    if !qstring_args.is_empty() {
        // Rewrite args
        method
            .sig
            .inputs
            .iter_mut()
            // We can skip self as that's not in our parameters above
            //
            // TODO: note this assumes that the first argument is self
            .skip(1)
            .enumerate()
            .filter(|(index, _)| qstring_args.contains(index))
            // Rewrite the type
            //
            // We add cxx_qt_lib to QString
            .map(|(_, item)| {
                if let syn::FnArg::Typed(syn::PatType { ref mut ty, .. }) = item {
                    let ty_path;

                    match ty.as_mut() {
                        syn::Type::Reference(syn::TypeReference { elem, .. }) => {
                            // If the type is a path then extract it and mark is_ref
                            if let syn::Type::Path(ref mut path) = &mut **elem {
                                ty_path = path;
                            } else {
                                return Err(syn::Error::new(
                                    item.span(),
                                    "QString args must be valid references.",
                                )
                                .to_compile_error());
                            }
                        }
                        _others => {
                            return Err(syn::Error::new(
                                item.span(),
                                "QString args must be references.",
                            )
                            .to_compile_error())
                        }
                    }

                    ty_path
                        .path
                        .segments
                        .insert(0, format_ident!("cxx_qt_lib").into());
                }
                Ok(item)
            })
            .collect::<Result<Vec<_>, TokenStream>>()?;
    }

    Ok(method)
}

/// Generate all the Rust code required to communicate with a QObject backed by generated C++ code
pub fn generate_qobject_rs(
    obj: &QObject,
    cpp_namespace_prefix: &[String],
) -> Result<TokenStream, TokenStream> {
    // Load macro attributes that were on the module, excluding #[make_qobject]
    let mod_attrs = obj
        .original_mod
        .attrs
        .iter()
        .filter_map(|attr| {
            // Filter out any attributes that are #[make_qobject] as that is ourselves
            //
            // TODO: what happens if there are multiple macros to start from?
            // Will generate_qobject_rs only ever come from make_qobject?
            // Otherwise we might need to pass the originating macro from the
            // calling proc_macro_attribute method.
            if let Some(first_segment) = attr.path.segments.first() {
                if first_segment.ident.to_string().as_str() != "make_qobject" {
                    Some(attr.to_owned())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<syn::Attribute>>();

    // Cache the original module ident and visibility
    let mod_ident = &obj.original_mod.ident;
    let mod_vis = &obj.original_mod.vis;

    // Cache the rust class name
    let rust_class_name = format_ident!("RustObj");
    let rust_wrapper_name = format_ident!("CppObjWrapper");

    // Generate cxx block
    let cxx_block = generate_qobject_cxx(obj, cpp_namespace_prefix)?;

    // Generate the data struct
    //
    // TODO: what happens with sub objects / pointers,
    // do we need to rewrite the field to their data struct?
    let data_struct_name = &obj.original_data_struct.ident;
    // Build a list of the fields that aren't pointers as they are stored on C++ side
    let data_fields_no_ptr = filter_fields(&obj.original_data_struct, is_field_ptr);
    // TODO: we need to update this to only store fields defined as "private" once we have an API for that
    let data_struct = build_struct_with_fields(&obj.original_data_struct, &data_fields_no_ptr);

    // Build a converter for Data -> CppObjWrapper
    let data_struct_impl = {
        let mut fields_into = vec![];
        // If there are no filtered fields then use _value
        let value_ident = if data_fields_no_ptr.is_empty() {
            format_ident!("_value")
        } else {
            format_ident!("value")
        };

        for field in &data_fields_no_ptr {
            if let Some(field_ident) = &field.ident {
                let field_name = field_ident.clone();

                // The Data struct should only contain "Qt-compatible" fields defined by
                // us so we will insure that From is implemented where necessary.
                fields_into.push(quote! { #field_name: #value_ident.#field_name().into() });
            }
        }

        quote! {
            impl<'a> From<&#rust_wrapper_name<'a>> for #data_struct_name {
                fn from(#value_ident: &#rust_wrapper_name<'a>) -> Self {
                    Self {
                        #(#fields_into),*
                    }
                }
            }
        }
    };

    // Generate property methods from the object
    let property_methods = generate_property_methods_rs(obj)?;

    // Capture methods, trait impls, use decls so they can used by quote
    let invokable_methods = obj
        .invokables
        .iter()
        .map(|m| fix_method_params(&m.original_method))
        .collect::<Result<Vec<syn::ImplItemMethod>, TokenStream>>()?;

    let normal_methods = obj
        .normal_methods
        .iter()
        .map(|m| fix_method_params(m))
        .collect::<Result<Vec<syn::ImplItemMethod>, TokenStream>>()?;

    let original_trait_impls = &obj.original_trait_impls;
    let original_passthrough_decls = &obj.original_passthrough_decls;

    // Generate the rust creator function
    let creator_fn = generate_rust_object_creator(obj)?;

    // Generate the cpp initialiser function
    let initialiser_fn = generate_cpp_object_initialiser(obj);

    // Build our filtered rust struct
    //
    // TODO: once fields are stored on this C++ side, this can change
    let rust_struct = build_struct_with_fields(
        &obj.original_rust_struct,
        &filter_fields(&obj.original_rust_struct, is_field_ptr),
    );

    // Define a function to handle update requests if we have one
    let handle_update_request = if obj.handle_updates_impl.is_some() {
        quote! {
            fn call_handle_update_request(&mut self, cpp: std::pin::Pin<&mut CppObj>) {
                self.handle_update_request(cpp);
            }
        }
    } else {
        quote! {}
    };

    // Define a function to handle property changes if we have one
    let handle_property_change = if obj.handle_property_change_impl.is_some() {
        quote! {
            fn call_handle_property_change(&mut self, cpp: std::pin::Pin<&mut CppObj>, property: Property) {
                self.handle_property_change(cpp, property);
            }
        }
    } else {
        quote! {}
    };

    let rust_struct_impl = quote! {
        impl #rust_class_name {
            #(#invokable_methods)*
            #(#normal_methods)*

            #handle_update_request
            #handle_property_change
        }
    };

    // Create a struct that wraps the CppObject with a nicer interface
    let wrapper_struct = quote! {
        pub struct #rust_wrapper_name<'a> {
            cpp: std::pin::Pin<&'a mut CppObj>,
        }
    };

    // TODO: eventually we want so support grabbing values from sub objects too
    let mut grab_values = vec![];
    for field in &data_fields_no_ptr {
        if let Some(field_ident) = &field.ident {
            let field_name = field_ident.clone();
            let setter_name = format_ident!("set_{}", field_name);

            grab_values.push(quote! {
                data.#field_name.
                    map_qt_value(|context, converted| context.#setter_name(converted), self);
            });
        }
    }

    // # Safety
    //
    // We only generate wrapper code for objects that derive from CxxQObject
    // so casting self.cpp to a CxxObject pointer is valid. We cast from *const
    // to *mut so that update_requester() can be called on const wrappers. This
    // is okay because only C++ code ever uses the pointer so the "const_cast" cannot
    // result in Rust UB. Since the wrapper was constructed from a non-const reference
    // in the first place, the underlying pointer does not refer to a const object
    // as far as C++ is concerned so we are not invoking C++ UB either.

    let wrapper_struct_impl = quote! {
        impl<'a> #rust_wrapper_name<'a> {
            fn new(cpp: std::pin::Pin<&'a mut CppObj>) -> Self {
                Self { cpp }
            }

            #(#property_methods)*

            pub fn update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester {
                use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};

                let ptr: *const CppObj = unsafe { &*self.cpp.as_ref() };
                unsafe { UpdateRequester::new(ptr as *mut CxxQObject) }
            }

            pub fn grab_values_from_data(&mut self, data: &#data_struct_name) {
                use cxx_qt_lib::MapQtValue;

                #(#grab_values)*
            }
        }
    };

    let mut use_traits = Vec::new();
    if obj.handle_updates_impl.is_some() {
        use_traits.push(quote! { use cxx_qt_lib::UpdateRequestHandler; });
    }
    if obj.handle_property_change_impl.is_some() {
        use_traits.push(quote! { use cxx_qt_lib::PropertyChangeHandler; });
    }

    let handle_updates_impl = if let Some(impl_) = &obj.handle_updates_impl {
        fix_impl_methods(impl_)?
    } else {
        quote! {}
    };

    let handle_property_change_impl = if let Some(impl_) = &obj.handle_property_change_impl {
        fix_impl_methods(impl_)?
    } else {
        quote! {}
    };

    // Build our rewritten module that replaces the input from the macro
    let output = quote! {
        #(#mod_attrs)*
        #mod_vis mod #mod_ident {
            #(#use_traits)*

            #(#original_passthrough_decls)*

            #cxx_block

            #rust_struct

            #rust_struct_impl

            #wrapper_struct

            #wrapper_struct_impl

            #data_struct

            #data_struct_impl

            #(#original_trait_impls)*

            #handle_updates_impl

            #handle_property_change_impl

            #creator_fn

            #initialiser_fn
        }
    };

    Ok(output.into_token_stream())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract_qobject;

    use pretty_assertions::assert_eq;
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    use syn::ItemMod;

    fn format_rs_source(rs_code: &str) -> String {
        // NOTE: this error handling is pretty rough so should only used for tests
        let mut command = Command::new("rustfmt");
        let mut child = command
            .args(&["--emit", "stdout"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        // Scope stdin to force an automatic flush
        {
            let mut stdin = child.stdin.take().unwrap();
            write!(stdin, "{}", rs_code).unwrap();
        }

        let output = child.wait_with_output().unwrap();
        let output = String::from_utf8(output.stdout).unwrap();

        // Quote does not retain empty lines so we throw them away in the case of the
        // reference string as to not cause clashes
        output.replace("\n\n", "\n")
    }

    #[test]
    fn generates_basic_custom_default() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_custom_default.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_custom_default.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_ident_changes() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_ident_changes.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_ident_changes.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_invokable_and_properties() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_invokable_and_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_invokable_and_properties.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_only_invokables() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_only_invokable.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_only_invokables_with_return() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_invokable_return.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_only_invokable_return.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_only_properties() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_only_properties.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_mod_attrs_vis() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_mod_attrs_vis.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_mod_attrs_vis.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_mod_passthrough() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_mod_passthrough.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_mod_passthrough.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_pin_invokable() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_pin_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_pin_invokable.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_subobject_property() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/subobject_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/subobject_property.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_subobject_pin_invokable() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/subobject_pin_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/subobject_pin_invokable.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_update_requester() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_update_requester.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_update_requester.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_change_handler() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_change_handler.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt".to_owned()];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/basic_change_handler.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    // TODO: add tests for more complex cases such as invokables with parameters
    // and for objects with properties
}
