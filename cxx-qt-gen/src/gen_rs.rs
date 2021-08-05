// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::extract::{QObject, QtTypes};
use crate::utils::is_type_ident_ptr;

/// A trait which we implement on QtTypes allowing retrieval of attributes of the enum value.
trait RustType {
    /// Whether this type is a reference
    fn is_ref(&self) -> bool;
    /// The ident of the type when used as a parameter on a function
    fn param_type_ident(&self) -> Ident;
}

impl RustType for QtTypes {
    /// Whether this type should be a reference when used in Rust methods
    fn is_ref(&self) -> bool {
        match self {
            Self::Str | Self::String => true,
            _others => false,
        }
    }
    /// The ident of the type when used as a parameter on a function
    fn param_type_ident(&self) -> Ident {
        match self {
            Self::I32 => format_ident!("i32"),
            Self::Str | Self::String => format_ident!("QString"),
            // Pointer types should not use this function
            _others => unreachable!(),
        }
    }
}

/// Generate Rust code that used CXX to interact with the C++ code generated for a QObject
pub fn generate_qobject_cxx(obj: &QObject) -> Result<TokenStream, TokenStream> {
    // Cache the original and rust class names, these are used multiple times later
    let class_name = &obj.ident;
    let rust_class_name = &obj.rust_struct_ident;

    // Build a snake version of the class name, this is used for rust method names
    //
    // TODO: Abstract this calculation to make it common to gen_rs and gen_cpp
    let ident_snake = class_name.to_string().to_case(Case::Snake);

    // Lists of functions we generate for the CXX bridge
    let mut cpp_functions = Vec::new();
    let mut rs_functions = Vec::new();

    // Invokables are only added to extern rust side
    //
    // TODO: later support a cxx_qt_name attribute on invokables to allow for renaming
    // to a custom name for C++ or Rust side?
    for i in &obj.invokables {
        // Cache the ident and parameters as they are used multiple times later
        let ident = &i.ident.rust_ident;
        let ident_cpp_str = &i.ident.cpp_ident.to_string();
        let parameters = &i.parameters;

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

            // TODO: add cpp functions for the invokable so that it can be called
            // consider how the different types for strings work here

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

            // Build the class name of the pointer, eg Object in crate::module::Object
            //
            // We can assume that unwrap will work here as we have checked that type_idents is not empty
            let ptr_class_name = type_idents.last().unwrap();

            // Swap the last type segment to be CppObj
            // so that crate::module::Object becomes crate::module::CppObj
            //
            // As we will generate a public type which points to the ffi type at the module level
            let mut type_idents_ffi = type_idents.clone();
            type_idents_ffi.pop();
            type_idents_ffi.push(format_ident!("CppObj"));

            // Add type definition for the class name we are a pointer for to the C++ bridge
            cpp_functions.push(quote! {
                type #ptr_class_name = #(#type_idents_ffi)::*;
            });

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

    // Build the methods to create the class
    let new_object_ident_cpp = format_ident!("new{}", class_name);
    let new_object_rust_str = format!("new_{}", class_name);
    let create_object_ident = format_ident!("create_{}_rs", ident_snake);
    let create_object_cpp_str = create_object_ident.to_string().to_case(Case::Camel);

    // Build the import path for the C++ header
    let import_path = format!("cxx-qt-gen/include/{}.h", ident_snake);

    // TODO: ideally we only want to add the "type QString = cxx_qt_lib::QString;"
    // if we actually generate some code that uses QString.

    // Build the CXX bridge
    let output = quote! {
        #[cxx::bridge]
        mod ffi {
            unsafe extern "C++" {
                include!(#import_path);

                type #class_name;
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
            }
        }

        pub type CppObj = ffi::#class_name;
    };

    Ok(output.into_token_stream())
}

/// Generate a Rust function that heap constructs the Rust object corresponding to the QObject
fn generate_rust_object_creator(obj: &QObject) -> Result<TokenStream, TokenStream> {
    // Cache the rust class name, this is used multiple times later
    let rust_class_name = &obj.rust_struct_ident;

    // Build the ident as snake case, then build the rust creator method
    //
    // TODO: can this code be shared with generate_qobject_cxx as they both create this fn name?
    let ident_snake = &obj.ident.to_string().to_case(Case::Snake);
    let fn_ident = format_ident!("create_{}_rs", ident_snake);

    // TODO: check if the original object had an explicit constructor and if so ensure that the create
    // function also takes the same parameters so that it can call this constructor. The C++ object will
    // also need to take the same parameters in its constructor. If the object is not default constructable
    // and does not provide a constructor then we need to throw an error.
    //
    // TODO: for now we assume that any object with properties implements Default. This likely means
    // for now it needs to derive from Default. As we don't (?) currently rename multiple impl
    // blocks - eg if there was a impl Default for Struct.

    // If an object has properties, we assume that it implements Default.
    let output = if obj.properties.is_empty() {
        quote! {
            fn #fn_ident() -> Box<#rust_class_name> {
                Box::new(#rust_class_name {})
            }
        }
    } else {
        quote! {
            fn #fn_ident() -> Box<#rust_class_name> {
                Box::new(#rust_class_name::default())
            }
        }
    };

    Ok(output.into_token_stream())
}

fn generate_property_methods_rs(obj: &QObject) -> Result<Vec<TokenStream>, TokenStream> {
    // Cache the rust class name, this is used multiple times later
    let rust_class_name = &obj.rust_struct_ident;

    // Build a list of property methods impls
    let mut property_methods = Vec::new();

    for property in &obj.properties {
        // Cache the property name and type
        let property_ident = &property.ident.rust_ident;
        let type_idents = &property.type_ident.idents;

        // Only add Rust getters and setters if we are not a special case of a pointer
        // If the type is a pointer then the getters are setters are on the C++ side
        if !is_type_ident_ptr(type_idents) {
            // TODO: later we might need consider if the struct has already implemented custom getters
            if let Some(getter) = &property.getter {
                // Generate a getter using the rust ident
                let getter_ident = &getter.rust_ident;

                property_methods.push(quote! {
                    fn #getter_ident(self: &#rust_class_name) -> &#(#type_idents)::* {
                        &self.#property_ident
                    }
                });
            }

            // TODO: later we might need consider if the struct has already implemented custom setters
            if let Some(setter) = &property.setter {
                // Generate a setter using the rust ident
                let setter_ident = &setter.rust_ident;

                property_methods.push(quote! {
                    fn #setter_ident(self: &mut #rust_class_name, value: #(#type_idents)::*) {
                        self.#property_ident = value;
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

fn rename_filter_struct(
    original_struct: &syn::ItemStruct,
    struct_ident: &syn::Ident,
) -> TokenStream {
    // Filter the fields of the struct to remove any pointer fields
    // as they are instead stored in the C++ object and not owned by the rust side
    let filtered_fields = original_struct
        .fields
        .iter()
        .filter(|field| is_field_ptr(field))
        .collect::<Vec<&syn::Field>>();

    // Capture the attributes, generics, visibility as local vars so they can be used by quote
    let original_attributes = &original_struct.attrs;
    let original_generics = &original_struct.generics;
    let original_visibility = &original_struct.vis;

    // Finally build the renamed struct
    //
    // If there are no fields then use semi-colon instead of brackets
    if filtered_fields.is_empty() {
        quote! {
            #(#original_attributes)*
            #original_visibility struct #struct_ident #original_generics;
        }
    } else {
        quote! {
            #(#original_attributes)*
            #original_visibility struct #struct_ident #original_generics {
                #(#filtered_fields),*
            }
        }
    }
}

/// Generate all the Rust code required to communicate with a QObject backed by generated C++ code
pub fn generate_qobject_rs(obj: &QObject) -> Result<TokenStream, TokenStream> {
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

    // Cache the original module ident and visiblity
    let mod_ident = &obj.original_mod.ident;
    let mod_vis = &obj.original_mod.vis;

    // Cache the rust class name
    let rust_class_name = &obj.rust_struct_ident;

    // Generate cxx block
    let cxx_block = generate_qobject_cxx(obj)?;

    // Create our renamed struct eg MyObject -> MyObjectRs with any filtering required
    let renamed_struct = rename_filter_struct(&obj.original_struct, rust_class_name);

    // Generate the data struct
    //
    // TODO: what happens with sub objects / pointers,
    // do we need to rewrite the field to their data struct?
    //
    // TODO: what happens if the original struct has no fields?
    // Do we then skip the data struct? Or do we always want a data struct
    // for simplicity?
    let data_struct_name = format_ident!("{}Data", obj.ident);
    let data_struct = rename_filter_struct(&obj.original_struct, &data_struct_name);

    // TODO: should we instead pass this around, as we do this filter multiple times
    let filtered_fields = obj
        .original_struct
        .fields
        .iter()
        .filter(|field| is_field_ptr(field))
        .collect::<Vec<&syn::Field>>();

    // Determine if we need a impl block on the data struct
    //
    // TODO: if there are original impl Default for struct then do we need to copy them?
    let data_struct_impl = if filtered_fields.is_empty() {
        quote! {}
    } else {
        let mut fields = vec![];
        let mut fields_clone = vec![];
        for field in filtered_fields {
            if let Some(field_ident) = &field.ident {
                let field_name = field_ident.clone();
                fields.push(quote! { #field_name: value.#field_name });
                // TODO: here we assume that all fields in the struct that are in the data struct
                // implement Clone.
                fields_clone.push(quote! { #field_name: value.#field_name.clone() });
            }
        }

        quote! {
            impl From<#data_struct_name> for #rust_class_name {
                fn from(value: #data_struct_name) -> Self {
                    Self {
                        #(#fields),*
                    }
                }
            }

            impl From<&#rust_class_name> for #data_struct_name {
                fn from(value: &#rust_class_name) -> Self {
                    Self {
                        #(#fields_clone),*
                    }
                }
            }
        }
    };

    // Generate property methods from the object
    let property_methods = generate_property_methods_rs(obj)?;

    // Capture original methods, trait impls, use decls so they can used by quote
    let original_methods = obj.invokables.iter().map(|m| &m.original_method);
    let original_trait_impls = &obj.original_trait_impls;
    let original_use_decls = &obj.original_use_decls;

    // Generate the rust creator function
    let creator_fn = generate_rust_object_creator(obj)?;

    // Determine if we need an impl block on the renamed struct
    let renamed_struct_impl = if original_methods.len() != 0 || !property_methods.is_empty() {
        quote! {
            impl #rust_class_name {
                #(#original_methods)*
                #(#property_methods)*
            }
        }
    } else {
        quote! {}
    };

    // Build our rewritten module that replaces the input from the macro
    let output = quote! {
        #(#mod_attrs)*
        #mod_vis mod #mod_ident {
            #(#original_use_decls)*

            #cxx_block

            #renamed_struct

            #renamed_struct_impl

            #data_struct

            #data_struct_impl

            #(#original_trait_impls)*

            #creator_fn
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
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_custom_default.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
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
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_ident_changes.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
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
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_invokable_and_properties.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
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
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_only_invokable.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
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
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_only_invokable_return.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
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
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_only_properties.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
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
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_mod_attrs_vis.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_mod_use() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_mod_use.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_mod_use.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
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
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/subobject_property.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    // TODO: add tests for more complex cases such as invokables with parameters
    // and for objects with properties
}
