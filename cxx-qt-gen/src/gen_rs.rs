// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::HashSet;

use crate::extract::{Invokable, QObject, QtTypes};
use crate::syntax::path::path_compare_str;
use crate::utils::type_to_namespace;

/// A trait which we implement on QtTypes allowing retrieval of attributes of the enum value.
trait RustType {
    /// Whether this type is a reference
    fn is_ref(&self) -> bool;
    /// The name of the type when defined in the CXX bridge, eg the A in type A = B;
    fn cxx_bridge_type_ident(&self) -> Ident;
    /// The full type for the parameter. Can be used Rust code outside cxx::bridge.
    fn cxx_qt_lib_type(&self) -> TokenStream;
}

impl RustType for QtTypes {
    /// Whether this type should be a reference when used in Rust methods
    fn is_ref(&self) -> bool {
        match self {
            Self::QColor => true,
            Self::QDate => true,
            Self::QDateTime => true,
            Self::QPoint => true,
            Self::QPointF => true,
            Self::QRect => true,
            Self::QRectF => true,
            Self::QSize => true,
            Self::QSizeF => true,
            Self::QTime => true,
            Self::QString => true,
            Self::QUrl => true,
            Self::QVariant => true,
            Self::UniquePtr { .. } => true,
            _others => false,
        }
    }

    /// The name of the type when defined in the CXX bridge
    ///
    /// eg the A in type A = B;
    /// And then the A in fn method(A) -> A;
    fn cxx_bridge_type_ident(&self) -> Ident {
        match self {
            Self::Bool => format_ident!("bool"),
            Self::F32 => format_ident!("f32"),
            Self::F64 => format_ident!("f64"),
            Self::I8 => format_ident!("i8"),
            Self::I16 => format_ident!("i16"),
            Self::I32 => format_ident!("i32"),
            Self::QColor => format_ident!("QColor"),
            Self::QDate => format_ident!("QDate"),
            Self::QDateTime => format_ident!("QDateTime"),
            Self::QPoint => format_ident!("QPoint"),
            Self::QPointF => format_ident!("QPointF"),
            Self::QRect => format_ident!("QRect"),
            Self::QRectF => format_ident!("QRectF"),
            Self::QSize => format_ident!("QSize"),
            Self::QSizeF => format_ident!("QSizeF"),
            Self::QString => format_ident!("QString"),
            Self::QTime => format_ident!("QTime"),
            Self::QUrl => format_ident!("QUrl"),
            Self::QVariant => format_ident!("QVariant"),
            Self::U8 => format_ident!("u8"),
            Self::U16 => format_ident!("u16"),
            Self::U32 => format_ident!("u32"),
            Self::UniquePtr { inner } => format_ident!("{}", inner.cxx_bridge_type_ident()),
            _others => unreachable!(),
        }
    }

    /// The full type for the parameter. Can be used Rust code outside cxx::bridge.
    fn cxx_qt_lib_type(&self) -> TokenStream {
        match self {
            Self::Bool => quote! {bool},
            Self::CppObj {
                external,
                combined_name,
                ..
            } if external == &true => quote! {cxx::UniquePtr<#combined_name>},
            Self::F32 => quote! {f32},
            Self::F64 => quote! {f64},
            Self::I8 => quote! {i8},
            Self::I16 => quote! {i16},
            Self::I32 => quote! {i32},
            Self::QColor => quote! {cxx_qt_lib::QColor},
            Self::QDate => quote! {cxx_qt_lib::QDate},
            Self::QDateTime => quote! {cxx_qt_lib::QDateTime},
            Self::QPoint => quote! {cxx_qt_lib::QPoint},
            Self::QPointF => quote! {cxx_qt_lib::QPointF},
            Self::QRect => quote! {cxx_qt_lib::QRect},
            Self::QRectF => quote! {cxx_qt_lib::QRectF},
            Self::QSize => quote! {cxx_qt_lib::QSize},
            Self::QSizeF => quote! {cxx_qt_lib::QSizeF},
            Self::QString => quote! {cxx_qt_lib::QString},
            Self::QTime => quote! {cxx_qt_lib::QTime},
            Self::QUrl => quote! {cxx_qt_lib::QUrl},
            Self::QVariant => quote! {cxx_qt_lib::QVariant},
            Self::U8 => quote! {u8},
            Self::U16 => quote! {u16},
            Self::U32 => quote! {u32},
            Self::UniquePtr { inner } => {
                let inner = inner.cxx_qt_lib_type();
                quote! {UniquePtr<#inner>}
            }
            _other => unreachable!(),
        }
    }
}

/// Generate Rust code that used CXX to interact with the C++ code generated for a QObject
pub fn generate_qobject_cxx(
    obj: &QObject,
    cpp_namespace_prefix: &[&str],
) -> Result<TokenStream, TokenStream> {
    // Cache the original and rust class names, these are used multiple times later
    let class_name = &obj.ident;
    let rust_class_name_cpp = format_ident!("{}Qt", class_name);
    let cxx_class_name_rust = format_ident!("{}Rust", class_name);
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
                .iter()
                // If the ident starts with cxx_qt_ for now this means we should remove it
                // as we are trying to access the CppObj
                //
                // TODO: this hack will be removed in the future when we move to UniquePtr
                .map(|ident| {
                    if ident.starts_with("cxx_qt_") {
                        ident.get(7..).unwrap().to_string()
                    } else {
                        ident.to_owned()
                    }
                })
                .collect::<Vec<String>>()
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
    // Closure which retrieves Object from crate::module::Object and swaps to crate::module::FFICppObj
    let type_idents_to_ptr_class_name_and_ffi_type = |type_idents: &Vec<Ident>| {
        // Build the class name of the pointer, eg Object in crate::module::Object
        //
        // We can assume that unwrap will work here as we have checked that type_idents is not empty
        let ptr_class_name = type_idents.last().unwrap().clone();

        // Swap the last type segment to be FFICppObj
        // so that crate::module::Object becomes crate::module::FFICppObj
        //
        // As we will generate a public type which points to the ffi type at the module level
        let mut type_idents_ffi = type_idents.clone();
        type_idents_ffi.pop();
        type_idents_ffi.push(format_ident!("FFICppObj"));

        (ptr_class_name, type_idents_ffi)
    };

    // Invokables are only added to extern rust side
    //
    // TODO: later support a cxx_qt_name attribute on invokables to allow for renaming
    // to a custom name for C++ or Rust side?
    for i in &obj.invokables {
        // Cache the ident and parameters as they are used multiple times later
        let (ident, ident_cpp_str) = if let Some(ident_wrapper) = &i.ident_wrapper {
            (
                &ident_wrapper.rust_ident,
                ident_wrapper.cpp_ident.to_string(),
            )
        } else {
            (&i.ident.rust_ident, i.ident.cpp_ident.to_string())
        };
        let parameters = &i.parameters;
        let mutablility = if i.mutable {
            Some(quote! { mut })
        } else {
            None
        };

        // TODO: invokables need to also become freestanding functions that
        // take as input a reference to both the Rs class and the CppObject
        // inside a wrapper. The functions that are impl'ed on the Rs class
        // will then simply create the wrapper and call the free functions.
        //
        // As a first step we could maybe just add a `cpp: Pin<&mut FFICppObj>`
        // argument to invokables so that users can manually wrap it.

        // Determine if the invokable has any parameter
        if parameters.is_empty() {
            // Determine if there is a return type
            if let Some(return_type) = &i.return_type {
                // Cache and build the return type
                let type_ident = &return_type.qt_type.cxx_bridge_type_ident();
                let type_ident = if return_type.qt_type.is_opaque() {
                    quote! { UniquePtr<#type_ident> }
                } else if return_type.is_ref {
                    quote! { &#type_ident }
                } else {
                    quote! { #type_ident }
                };

                rs_functions.push(quote! {
                    #[cxx_name = #ident_cpp_str]
                    fn #ident(self: &#mutablility #rust_class_name) -> #type_ident;
                });
            } else {
                rs_functions.push(quote! {
                    #[cxx_name = #ident_cpp_str]
                    fn #ident(self: &#mutablility #rust_class_name);
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
                    QtTypes::CppObj {
                        external,
                        rust_type_idents,
                        combined_name,
                        ..
                    } => {
                        if *external {
                            // TODO: these two will likely replace type_idents_to_ptr_class_name_and_ffi_type
                            // once we remove Ptr as a type
                            let rust_type_idents = rust_type_idents
                                .iter()
                                .take(rust_type_idents.len() - 1)
                                .cloned()
                                .chain(vec![format_ident!("FFICppObj")])
                                .collect::<Vec<Ident>>();

                            parameters_quotes.push(quote! {
                                #ident: Pin<&mut #combined_name>
                            });

                            // Add the type of the external object to the C++ bridge
                            cpp_types_push_unique(
                                &mut cpp_functions,
                                &mut cpp_types,
                                combined_name,
                                rust_type_idents,
                            )?;
                        } else {
                            parameters_quotes.push(quote! {
                                #ident: Pin<&mut #(#rust_type_idents)::*>
                            });
                        }
                    }
                    _others => {
                        let type_ident = &p.type_ident.qt_type.cxx_bridge_type_ident();
                        let is_ref = if p.type_ident.is_ref {
                            quote! {&}
                        } else {
                            quote! {}
                        };
                        let is_mut = if p.type_ident.is_mut {
                            quote! {mut}
                        } else {
                            quote! {}
                        };
                        parameters_quotes.push(quote! {
                            #ident: #is_ref #is_mut #type_ident
                        });
                    }
                };
            }

            // Determine if there is a return type and if it's a reference
            if let Some(return_type) = &i.return_type {
                // Cache and build the return type
                let type_ident = &return_type.qt_type.cxx_bridge_type_ident();
                let type_ident = if return_type.qt_type.is_opaque() {
                    quote! { UniquePtr<#type_ident> }
                } else if return_type.is_ref {
                    quote! { &#type_ident }
                } else {
                    quote! { #type_ident }
                };

                rs_functions.push(quote! {
                    #[cxx_name = #ident_cpp_str]
                    fn #ident(self: &#mutablility #rust_class_name, #(#parameters_quotes),*) -> #type_ident;
                });
            } else {
                rs_functions.push(quote! {
                    #[cxx_name = #ident_cpp_str]
                    fn #ident(self: &#mutablility #rust_class_name, #(#parameters_quotes),*);
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

        // This type is a pointer (CppObj), so special case the C++ functions and no Rust functions
        if let QtTypes::CppObj { combined_name, .. } = &property.type_ident.qt_type {
            // Check that type_idents is not empty
            if type_idents.is_empty() {
                return Err(syn::Error::new(
                    property.ident.rust_ident.span(),
                    "Property type needs at least one type ident.",
                )
                .to_compile_error());
            }

            // Retrieve Object from crate::module::Object and swap to crate::module::FFICppObj
            let (_, type_idents_ffi) = type_idents_to_ptr_class_name_and_ffi_type(type_idents);

            // Add type definition for the class name we are a pointer for to the C++ bridge
            //
            // Ensure that we only do this once
            cpp_types_push_unique(
                &mut cpp_functions,
                &mut cpp_types,
                combined_name,
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
                fn #getter_cpp(self: Pin<&mut #rust_class_name_cpp>) -> UniquePtr<#combined_name>;
                #[rust_name = #setter_str]
                fn #setter_cpp(self: Pin<&mut #rust_class_name_cpp>, value: UniquePtr<#combined_name>);
            });
        // This is a normal primitive type so add Rust getters and setters
        } else {
            // Build the C++ method declarations names
            let getter_str = &property_ident_snake;
            let getter_cpp = format_ident!("get{}", property_ident_pascal);
            let setter_str = format!("set_{}", property_ident_snake);
            let setter_cpp = format_ident!("set{}", property_ident_pascal);

            let qt_type = &property.type_ident.qt_type;
            let param_type = qt_type.cxx_bridge_type_ident();
            let param_type = if qt_type.is_ref() {
                quote! {&#param_type}
            } else {
                quote! {#param_type}
            };

            // Add the getter and setter to C++ bridge
            cpp_functions.push(quote! {
                #[rust_name = #getter_str]
                fn #getter_cpp(self: &#rust_class_name_cpp) -> #param_type;
                #[rust_name = #setter_str]
                fn #setter_cpp(self: Pin<&mut #rust_class_name_cpp>, value: #param_type);
            });
        }
    }

    // Add signals emitters
    for signal in &obj.signals {
        let signal_ident_cpp = &signal.signal_ident.cpp_ident;
        let signal_ident_rust_str = &signal.signal_ident.rust_ident.to_string();

        let queued_ident_cpp = &signal.emit_ident.cpp_ident;
        let queued_ident_rust_str = &signal.emit_ident.rust_ident.to_string();

        if signal.parameters.is_empty() {
            cpp_functions.push(quote! {
                #[rust_name = #signal_ident_rust_str]
                fn #signal_ident_cpp(self: Pin<&mut #rust_class_name_cpp>);
                #[rust_name = #queued_ident_rust_str]
                fn #queued_ident_cpp(self: Pin<&mut #rust_class_name_cpp>);
            });
        } else {
            // For immediate parameters we want by-ref or primitive by-value
            let parameters = signal
                .parameters
                .iter()
                .map(|parameter| {
                    let ident = &parameter.ident;
                    let param_type = parameter.type_ident.qt_type.cxx_bridge_type_ident();
                    if parameter.type_ident.qt_type.is_ref() {
                        quote! {
                            #ident: &#param_type
                        }
                    } else {
                        quote! {
                            #ident: #param_type
                        }
                    }
                })
                .collect::<Vec<TokenStream>>();
            // For queued parameters we want by-value (or UniquePtr<T>)
            let parameters_queued = signal
                .parameters
                .iter()
                .map(|parameter| {
                    let ident = &parameter.ident;
                    let param_type = parameter.type_ident.qt_type.cxx_bridge_type_ident();
                    if parameter.type_ident.qt_type.is_opaque() {
                        quote! {
                            #ident: UniquePtr<#param_type>
                        }
                    } else {
                        quote! {
                            #ident: #param_type
                        }
                    }
                })
                .collect::<Vec<TokenStream>>();
            cpp_functions.push(quote! {
                #[rust_name = #signal_ident_rust_str]
                fn #signal_ident_cpp(self: Pin<&mut #rust_class_name_cpp>, #(#parameters),*);
                #[rust_name = #queued_ident_rust_str]
                fn #queued_ident_cpp(self: Pin<&mut #rust_class_name_cpp>, #(#parameters_queued),*);
            });
        }
    }

    // Define a function to handle update requests if we have one
    let handle_update_request = if obj.handle_updates_impl.is_some() {
        quote! {
            #[cxx_name = "handleUpdateRequest"]
            fn call_handle_update_request(self: &mut #rust_class_name, cpp: Pin<&mut #rust_class_name_cpp>);
        }
    } else {
        quote! {}
    };

    let update_requester_type = if obj.handle_updates_impl.is_some() {
        quote! {
            #[namespace = "rust::cxxqtlib1"]
            type UpdateRequester = cxx_qt_lib::UpdateRequesterCpp;
        }
    } else {
        quote! {}
    };
    let request_updater_method = if obj.handle_updates_impl.is_some() {
        quote! {
            #[rust_name = "update_requester"]
            fn updateRequester(self: Pin<&mut #rust_class_name_cpp>) -> UniquePtr<UpdateRequester>;
        }
    } else {
        quote! {}
    };

    // Build the import path for the C++ header
    let import_path = format!("cxx-qt-gen/include/{}.cxxqt.h", ident_snake);

    // TODO: ideally we only want to add the "type QString = cxx_qt_lib::QString;"
    // if we actually generate some code that uses QString.

    // Build the namespace string, rust::module
    let namespace = obj.namespace.join("::");

    // Build the module ident
    let mod_ident = &obj.original_mod.ident;
    let mod_vis = &obj.original_mod.vis;

    // Retrieve the passthrough items to CXX
    let cxx_items = &obj.cxx_items;

    // Build the CXX bridge
    let class_name_str = class_name.to_string();
    let cxx_class_name_rust_str = cxx_class_name_rust.to_string();
    let output = quote! {
        #[cxx::bridge(namespace = #namespace)]
        #mod_vis mod #mod_ident {
            unsafe extern "C++" {
                include!(#import_path);

                #[cxx_name = #class_name_str]
                type #rust_class_name_cpp;

                #update_requester_type

                #(#cpp_functions)*

                #[cxx_name = "unsafe_rust"]
                fn rust(self: &#rust_class_name_cpp) -> &#rust_class_name;
                #[rust_name = "new_cpp_object"]
                fn newCppObject() -> UniquePtr<#rust_class_name_cpp>;

                #request_updater_method
            }

            extern "C++" {
                #[cxx_name = "unsafe_rust_mut"]
                unsafe fn rust_mut(self: Pin<&mut #rust_class_name_cpp>) -> Pin<&mut #rust_class_name>;
            }

            extern "Rust" {
                #[cxx_name = #cxx_class_name_rust_str]
                type #rust_class_name;

                #(#rs_functions)*

                #[cxx_name = "createRs"]
                fn create_rs() -> Box<#rust_class_name>;

                #[cxx_name = "initialiseCpp"]
                fn initialise_cpp(cpp: Pin<&mut #rust_class_name_cpp>);

                #handle_update_request
            }

            #(#cxx_items)*
        }
    };

    Ok(output.into_token_stream())
}

/// Generate a Rust function that initialises a QObject with the values from Data::default()
fn generate_cpp_object_initialiser(obj: &QObject) -> TokenStream {
    let data_class_name = &obj.original_data_struct.ident;

    // We assume that all Data classes implement default
    let output = quote! {
        pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
            let mut wrapper = CppObj::new(cpp);
            wrapper.grab_values_from_data(#data_class_name::default());
        }
    };

    output.into_token_stream()
}

fn generate_property_methods_rs(obj: &QObject) -> Result<Vec<TokenStream>, TokenStream> {
    // Build a list of property methods impls
    let mut property_methods = Vec::new();

    for property in &obj.properties {
        let qt_type = if let QtTypes::UniquePtr { inner } = &property.type_ident.qt_type {
            &**inner
        } else {
            &property.type_ident.qt_type
        };
        let rust_param_type = qt_type.cxx_qt_lib_type();
        // When the output type is opaque we pass by value rather than ref
        // even though it's a non trivial type
        let rust_param_type = if !qt_type.is_opaque() && qt_type.is_ref() {
            quote! {&#rust_param_type}
        } else {
            quote! {#rust_param_type}
        };

        let cpp_getter_ident = &property.getter.as_ref().unwrap().rust_ident;
        let cpp_setter_ident = &property.setter.as_ref().unwrap().rust_ident;

        if let QtTypes::CppObj { .. } = property.type_ident.qt_type {
            let ident = &property.ident.rust_ident;
            let take_ident = format_ident!("take_{}", ident);
            let give_ident = format_ident!("give_{}", ident);

            property_methods.push(quote! {
                pub fn #take_ident(&mut self) -> #rust_param_type {
                    self.cpp.as_mut().#take_ident()
                }
            });

            property_methods.push(quote! {
                pub fn #give_ident(&mut self, value: #rust_param_type) {
                    self.cpp.as_mut().#give_ident(value);
                }
            });
        } else {
            if let Some(getter) = &property.getter {
                // Generate a getter using the rust ident
                let getter_ident = &getter.rust_ident;

                property_methods.push(quote! {
                    pub fn #getter_ident(&self) -> #rust_param_type {
                        self.cpp.#cpp_getter_ident()
                    }
                });
            }

            if let Some(setter) = &property.setter {
                // Generate a setter using the rust ident
                let setter_ident = &setter.rust_ident;
                if qt_type.is_opaque() {
                    property_methods.push(quote! {
                        pub fn #setter_ident(&mut self, value: #rust_param_type) {
                            self.cpp.as_mut().#cpp_setter_ident(&value);
                        }
                    });
                } else {
                    property_methods.push(quote! {
                        pub fn #setter_ident(&mut self, value: #rust_param_type) {
                            self.cpp.as_mut().#cpp_setter_ident(value);
                        }
                    });
                };
            }
        }
    }

    Ok(property_methods)
}

fn generate_signal_methods_rs(obj: &QObject) -> Result<Vec<TokenStream>, TokenStream> {
    let mut signal_methods = Vec::new();
    let mut queued_cases = Vec::new();
    let mut immediate_cases = Vec::new();
    let ident = &obj.signal_ident;

    for signal in &obj.signals {
        let emit_ident = &signal.emit_ident.rust_ident;
        let enum_ident = &signal.enum_ident;
        let parameters = signal
            .parameters
            .iter()
            .map(|parameter| &parameter.ident)
            .collect::<Vec<&Ident>>();
        let parameters_to_value_immediate = signal
            .parameters
            .iter()
            .map(|parameter| {
                let ident = &parameter.ident;
                if parameter.type_ident.qt_type.is_opaque() || parameter.type_ident.qt_type.is_ref()
                {
                    quote! { &#ident }
                } else {
                    ident.into_token_stream()
                }
            })
            .collect::<Vec<TokenStream>>();
        let parameters_to_value_queued = signal
            .parameters
            .iter()
            .map(|parameter| {
                let ident = &parameter.ident;
                if parameter.type_ident.qt_type.is_opaque() {
                    quote! { #ident }
                } else {
                    ident.into_token_stream()
                }
            })
            .collect::<Vec<TokenStream>>();
        let signal_ident = &signal.signal_ident.rust_ident;

        queued_cases.push(quote! {
            #ident::#enum_ident { #(#parameters),* } => self.cpp.as_mut().#emit_ident(#(#parameters_to_value_queued),*),
        });

        immediate_cases.push(quote! {
            #ident::#enum_ident { #(#parameters),* } => self.cpp.as_mut().#signal_ident(#(#parameters_to_value_immediate),*),
        });
    }

    if !queued_cases.is_empty() {
        signal_methods.push(quote! {
            pub fn emit_queued(&mut self, signal: #ident) {
                match signal {
                    #(#queued_cases)*
                }
            }
        });
    }

    if !immediate_cases.is_empty() {
        signal_methods.push(quote! {
            pub unsafe fn emit_immediate(&mut self, signal: #ident) {
                match signal {
                    #(#immediate_cases)*
                }
            }
        });
    }

    Ok(signal_methods)
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

/// Generate the wrapper method for a given invokable
fn invokable_generate_wrapper(
    invokable: &Invokable,
    ident_wrapper: &Ident,
) -> Result<TokenStream, TokenStream> {
    let ident = &invokable.ident.rust_ident;
    let mutablility = if invokable.mutable {
        Some(quote! { mut })
    } else {
        None
    };

    let mut input_parameters = vec![];
    let mut output_parameters = vec![];
    let mut wrappers = vec![];

    for param in &invokable.parameters {
        let param_ident = &param.ident;
        let is_mut = if param.type_ident.is_mut {
            quote! { mut }
        } else {
            quote! {}
        };
        let is_ref = if param.type_ident.is_ref {
            quote! { & }
        } else {
            quote! {}
        };

        if let QtTypes::CppObj {
            rust_type_idents, ..
        } = &param.type_ident.qt_type
        {
            // Create Rust idents with CppObj and FFICppObj at the end
            let rust_idents_module = rust_type_idents
                .iter()
                .take(rust_type_idents.len() - 1)
                .cloned()
                .collect::<Vec<Ident>>();
            let rust_idents_ffi = rust_idents_module
                .iter()
                .cloned()
                .chain(vec![format_ident!("FFICppObj")]);
            let rust_idents_cpp_obj = rust_idents_module
                .iter()
                .cloned()
                .chain(vec![format_ident!("CppObj")]);

            input_parameters
                .push(quote! { #param_ident: std::pin::Pin<&mut #(#rust_idents_ffi)::*> });

            wrappers.push(quote! {
                let mut #param_ident = #(#rust_idents_cpp_obj)::*::new(#param_ident);
            });
            output_parameters.push(quote! { #is_ref #is_mut #param_ident });
        } else {
            // If we are an opaque input type we need to convert to the Rust type
            //
            // And then keep the ref and mut state of the parameter
            if param.type_ident.qt_type.is_opaque() {
                wrappers.push(quote! {
                    let #is_mut #param_ident = #param_ident.to_rust();
                });

                output_parameters.push(quote! { #is_ref #is_mut #param_ident });
            } else {
                output_parameters.push(quote! { #param_ident });
            }

            let param_type = param.type_ident.qt_type.cxx_qt_lib_type();
            input_parameters.push(quote! { #param_ident: #is_ref #is_mut #param_type });
        }
    }

    // If we are an opaque return type then we need to convert into the C++ type
    if let Some(return_type) = &invokable.return_type {
        let return_type_ident = return_type.qt_type.cxx_qt_lib_type();

        Ok(quote! {
            pub fn #ident_wrapper(&#mutablility self, #(#input_parameters),*) -> #return_type_ident {
                #(#wrappers)*
                return self.#ident(#(#output_parameters),*);
            }
        })
    } else {
        Ok(quote! {
            pub fn #ident_wrapper(&#mutablility self, #(#input_parameters),*) {
                #(#wrappers)*
                self.#ident(#(#output_parameters),*);
            }
        })
    }
}

/// Generate all the Rust code required to communicate with a QObject backed by generated C++ code
pub fn generate_qobject_rs(
    obj: &QObject,
    cpp_namespace_prefix: &[&str],
) -> Result<TokenStream, TokenStream> {
    // Load macro attributes that were on the module, excluding #[cxx_qt::bridge]
    let mod_attrs = obj
        .original_mod
        .attrs
        .iter()
        .filter_map(|attr| {
            // Filter out any attributes that are #[cxx_qt::bridge] as that is ourselves
            //
            // TODO: what happens if there are multiple macros to start from?
            // Will generate_qobject_rs only ever come from cxx_qt::bridge?
            // Otherwise we might need to pass the originating macro from the
            // calling proc_macro_attribute method.
            if path_compare_str(&attr.path, &["cxx_qt", "bridge"]) {
                None
            } else {
                Some(attr.to_owned())
            }
        })
        .collect::<Vec<syn::Attribute>>();

    // Cache the original module ident and visibility
    let mod_ident = &obj.original_mod.ident;
    let cxx_qt_mod_ident = format_ident!("cxx_qt_{}", mod_ident);
    let mod_vis = &obj.original_mod.vis;
    let class_name = &obj.ident;

    // Cache the rust class name
    let rust_class_name = format_ident!("RustObj");
    let rust_wrapper_name = format_ident!("CppObj");

    // Generate cxx block
    let cxx_block = generate_qobject_cxx(obj, cpp_namespace_prefix)?;

    // Generate the data struct
    //
    // TODO: what happens with sub objects / pointers,
    // do we need to rewrite the field to their data struct?
    let data_struct_name = &obj.original_data_struct.ident;
    // Build a list of the fields that aren't pointers as they are stored on C++ side
    let data_fields_no_ptr = obj
        .properties
        .iter()
        .zip(&obj.original_data_struct.fields)
        .filter_map(|(prop, field)| {
            if let QtTypes::CppObj { .. } = prop.type_ident.qt_type {
                None
            } else {
                Some((&prop.type_ident.qt_type, field))
            }
        })
        .collect::<Vec<(&QtTypes, &syn::Field)>>();
    // TODO: we need to update this to only store fields defined as "private" once we have an API for that
    let data_struct = build_struct_with_fields(
        &obj.original_data_struct,
        &data_fields_no_ptr
            .iter()
            .map(|(_, field)| *field)
            .collect::<Vec<&syn::Field>>(),
    );

    // Build a converter for Data -> CppObj
    let data_struct_impl = {
        let mut fields_into = vec![];
        // If there are no filtered fields then use _value
        let value_ident = if data_fields_no_ptr.is_empty() {
            format_ident!("_value")
        } else {
            format_ident!("value")
        };

        for (_, field) in &data_fields_no_ptr {
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

            impl<'a> From<&mut #rust_wrapper_name<'a>> for #data_struct_name {
                fn from(#value_ident: &mut #rust_wrapper_name<'a>) -> Self {
                    Self::from(&*#value_ident)
                }
            }
        }
    };

    // Generate property methods from the object
    let property_methods = generate_property_methods_rs(obj)?;
    let signal_methods = generate_signal_methods_rs(obj)?;
    let signal_enum = obj.original_signal_enum.clone().map(|mut signal_enum| {
        signal_enum.attrs = signal_enum
            .attrs
            .iter()
            .filter_map(|attr| {
                // Filter out any attributes that are #[cxx_qt::signals] as that is ourselves
                if path_compare_str(&attr.path, &["cxx_qt", "signals"]) {
                    None
                } else {
                    Some(attr.to_owned())
                }
            })
            .collect::<Vec<syn::Attribute>>();
        Some(signal_enum)
    });

    // Capture methods, trait impls, use decls so they can used by quote
    let invokable_method_wrappers = obj
        .invokables
        .iter()
        .filter_map(|i| {
            i.ident_wrapper
                .as_ref()
                .map(|ident_wrapper| invokable_generate_wrapper(i, &ident_wrapper.rust_ident))
        })
        .collect::<Result<Vec<TokenStream>, TokenStream>>()?;
    let invokable_methods = obj
        .invokables
        .iter()
        .map(|m| m.original_method.clone())
        .collect::<Vec<syn::ImplItemMethod>>();

    let methods = &obj.methods;
    let original_passthrough_decls = &obj.original_passthrough_decls;

    // Generate the cpp initialiser function
    let initialiser_fn = generate_cpp_object_initialiser(obj);

    // Build our filtered rust struct
    let rust_struct = build_struct_with_fields(
        &obj.original_rust_struct,
        &obj.original_rust_struct
            .fields
            .iter()
            .collect::<Vec<&syn::Field>>(),
    );

    // Define a function to handle update requests if we have one
    let handle_update_request = if obj.handle_updates_impl.is_some() {
        quote! {
            pub fn call_handle_update_request(&mut self, cpp: std::pin::Pin<&mut FFICppObj>) {
                let mut cpp = CppObj::new(cpp);
                self.handle_update_request(&mut cpp);
            }
        }
    } else {
        quote! {}
    };

    let rust_struct_impl = quote! {
        impl #rust_class_name {
            #(#invokable_method_wrappers)*
            #(#invokable_methods)*
            #(#methods)*

            #handle_update_request
        }
    };

    // Create a struct that wraps the CppObject with a nicer interface
    let wrapper_struct = quote! {
        pub struct #rust_wrapper_name<'a> {
            cpp: std::pin::Pin<&'a mut FFICppObj>,
        }
    };

    // TODO: eventually we want so support grabbing values from sub objects too
    let mut grab_values = vec![];
    for (qt_type, field) in &data_fields_no_ptr {
        if let Some(field_ident) = &field.ident {
            let field_name = field_ident.clone();
            let setter_name = format_ident!("set_{}", field_name);

            if qt_type.is_opaque() {
                grab_values.push(quote! {
                    self.#setter_name(data.#field_name.as_ref().unwrap());
                });
            } else {
                let is_ref = if qt_type.is_ref() {
                    quote! {&}
                } else {
                    quote! {}
                };

                grab_values.push(quote! {
                    self.#setter_name(#is_ref data.#field_name);
                });
            }
        }
    }

    let update_requester = if obj.handle_updates_impl.is_some() {
        quote! {
            pub fn update_requester(&mut self) -> cxx_qt_lib::UpdateRequester {
                cxx_qt_lib::UpdateRequester::from_unique_ptr(self.cpp.as_mut().update_requester())
            }
        }
    } else {
        quote! {}
    };

    let wrapper_struct_impl = quote! {
        impl<'a> #rust_wrapper_name<'a> {
            pub fn new(cpp: std::pin::Pin<&'a mut FFICppObj>) -> Self {
                Self { cpp }
            }

            #(#property_methods)*
            #(#signal_methods)*

            #update_requester

            pub fn grab_values_from_data(&mut self, mut data: #data_struct_name) {
                #(#grab_values)*
            }
        }
    };

    let mut use_traits = Vec::new();
    if obj.handle_updates_impl.is_some() {
        use_traits.push(quote! { use cxx_qt_lib::UpdateRequestHandler; });
    }

    let handle_updates_impl = &obj.handle_updates_impl;

    // Build our rewritten module that replaces the input from the macro
    //
    // TODO: where do the mod_attrs go ?
    let class_name_cpp = format_ident!("{}Qt", class_name);
    let output = quote! {
        #cxx_block

        pub use self::#cxx_qt_mod_ident::*;
        #(#mod_attrs)*
        #mod_vis mod #cxx_qt_mod_ident {
            use super::#mod_ident::*;

            #(#use_traits)*

            pub type FFICppObj = super::#mod_ident::#class_name_cpp;
            type UniquePtr<T> = cxx::UniquePtr<T>;

            #signal_enum

            #rust_struct

            #rust_struct_impl

            #wrapper_struct

            #wrapper_struct_impl

            #data_struct

            #data_struct_impl

            #handle_updates_impl

            #(#original_passthrough_decls)*

            pub fn create_rs() -> std::boxed::Box<RustObj> {
                std::default::Default::default()
            }

            #initialiser_fn
        }
    };

    Ok(output.into_token_stream())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract_qobject;

    use pretty_assertions::assert_str_eq;
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
    fn generates_custom_default() {
        let source = include_str!("../test_inputs/custom_default.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/custom_default.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_handlers() {
        let source = include_str!("../test_inputs/handlers.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/handlers.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_invokables() {
        let source = include_str!("../test_inputs/invokables.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/invokables.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_naming() {
        let source = include_str!("../test_inputs/naming.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/naming.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_passthrough() {
        let source = include_str!("../test_inputs/passthrough.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/passthrough.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_properties() {
        let source = include_str!("../test_inputs/properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/properties.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_signals() {
        let source = include_str!("../test_inputs/signals.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/signals.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_types_primitive_property() {
        let source = include_str!("../test_inputs/types_primitive_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/types_primitive_property.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_types_qt_property() {
        let source = include_str!("../test_inputs/types_qt_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/types_qt_property.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_types_qt_invokable() {
        let source = include_str!("../test_inputs/types_qt_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_output = include_str!("../test_outputs/types_qt_invokable.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject, &cpp_namespace_prefix)
            .unwrap()
            .to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }
}
