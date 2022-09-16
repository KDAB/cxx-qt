// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Item, ItemMod};

use crate::extract::{Invokable, Property, QObject, QtTypes};
use crate::generator::{
    naming,
    naming::{property::QPropertyName, qobject::QObjectName},
    rust::{
        qobject::{GeneratedRustQObject, GeneratedRustQObjectBlocks},
        GeneratedRustBlocks,
    },
};
use crate::writer::rust::write_rust;

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

fn generate_invokable_cxx_declaration(obj: &QObject, i: &Invokable) -> TokenStream {
    let qobject_idents = naming::qobject::QObjectName::from(&obj.original_rust_struct.ident);
    let cpp_class_name_rust = qobject_idents.cpp_class.rust;
    let rust_struct_name_rust = qobject_idents.rust_struct.rust;

    // Cache the ident and parameters as they are used multiple times later
    let (ident, ident_cpp_str) = (&i.ident_wrapper.rust, i.ident_wrapper.cpp.to_string());
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
    // As a first step we could maybe just add a `cpp: Pin<&mut MyObjectQt>`
    // argument to invokables so that users can manually wrap it.

    // Determine if the invokable has any parameter
    let mut parameters_quotes = Vec::new();

    let cpp_type = if i.mutable {
        quote! { Pin<&mut #cpp_class_name_rust> }
    } else {
        quote! { &#cpp_class_name_rust }
    };

    parameters_quotes.push(quote! { cpp: #cpp_type });

    for p in parameters {
        // Cache the name and type
        let ident = &p.ident;

        // If the type is Pin<T> then we need to change extract differently
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
    // Determine if there is a return type
    let return_expr = if let Some(return_type) = &i.return_type {
        // Cache and build the return type
        let type_ident = &return_type.qt_type.cxx_bridge_type_ident();
        if return_type.qt_type.is_opaque() {
            Some(quote! { -> UniquePtr<#type_ident> })
        } else if return_type.is_ref {
            Some(quote! { -> &#type_ident })
        } else {
            Some(quote! { -> #type_ident })
        }
    } else {
        None
    };

    quote! {
        #[cxx_name = #ident_cpp_str]
        fn #ident(self: &#mutablility #rust_struct_name_rust, #(#parameters_quotes),*) #return_expr;
    }
}

/// Generate Rust code that used CXX to interact with the C++ code generated for a QObject
pub fn generate_qobject_cxx(obj: &QObject) -> Result<Vec<Item>, TokenStream> {
    // Cache the original and rust class names, these are used multiple times later
    let qobject_idents = naming::qobject::QObjectName::from(&obj.original_rust_struct.ident);
    let cpp_class_name_cpp = qobject_idents.cpp_class.cpp.to_string();
    let cpp_class_name_rust = qobject_idents.cpp_class.rust;
    let rust_struct_name_cpp = qobject_idents.rust_struct.cpp.to_string();
    let rust_struct_name_rust = qobject_idents.rust_struct.rust;

    // Lists of functions we generate for the CXX bridge
    let mut cpp_functions = Vec::new();
    let mut rs_functions = Vec::new();

    // Invokables are only added to extern rust side
    //
    // TODO: later support a cxx_qt_name attribute on invokables to allow for renaming
    // to a custom name for C++ or Rust side?
    for i in &obj.invokables {
        rs_functions.push(generate_invokable_cxx_declaration(obj, i));
    }

    // Add getters/setters/notify from properties
    for property in &obj.properties {
        let property_ident = QPropertyName::from(&property.ident);
        let getter_cpp = &property_ident.getter.cpp.to_string();
        let getter_rust = property_ident.getter.rust;

        let setter_cpp = &property_ident.setter.cpp.to_string();
        let setter_rust = property_ident.setter.rust;

        let emit_cpp = &property_ident.emit.cpp;
        let emit_rust = property_ident.emit.rust.to_string();

        // Add the emit changed
        cpp_functions.push(quote! {
            #[rust_name = #emit_rust]
            fn #emit_cpp(self: Pin<&mut #cpp_class_name_rust>);
        });

        let qt_type = &property.type_ident.qt_type;
        let qt_type_ident = qt_type.cxx_bridge_type_ident();
        let ty = if qt_type.is_opaque() {
            quote! { UniquePtr<#qt_type_ident> }
        } else {
            quote! { #qt_type_ident }
        };

        // Add the getter and setter to C++ bridge
        rs_functions.push(quote! {
            #[cxx_name = #getter_cpp]
            unsafe fn #getter_rust<'a>(self: &'a #rust_struct_name_rust, cpp: &'a #cpp_class_name_rust) -> &'a #ty;
            #[cxx_name = #setter_cpp]
            fn #setter_rust(self: &mut #rust_struct_name_rust, cpp: Pin<&mut #cpp_class_name_rust>, value: #ty);
        });
    }

    // Add signals emitters
    for signal in &obj.signals {
        let queued_ident_cpp = &signal.emit_ident.cpp;
        let queued_ident_rust_str = &signal.emit_ident.rust.to_string();

        if signal.parameters.is_empty() {
            cpp_functions.push(quote! {
                #[rust_name = #queued_ident_rust_str]
                fn #queued_ident_cpp(self: Pin<&mut #cpp_class_name_rust>);
            });
        } else {
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
                #[rust_name = #queued_ident_rust_str]
                fn #queued_ident_cpp(self: Pin<&mut #cpp_class_name_rust>, #(#parameters_queued),*);
            });
        }
    }

    // Build the CXX bridge
    Ok(vec![
        syn::parse2::<Item>(
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = #cpp_class_name_cpp]
                    type #cpp_class_name_rust;

                    #(#cpp_functions)*
                }
            }
            .into_token_stream(),
        )
        .map_err(|err| err.to_compile_error())?,
        syn::parse2::<Item>(
            quote! {
                extern "Rust" {
                    #[cxx_name = #rust_struct_name_cpp]
                    type #rust_struct_name_rust;

                    #(#rs_functions)*
                }
            }
            .into_token_stream(),
        )
        .map_err(|err| err.to_compile_error())?,
    ])
}

fn generate_signal_methods_rs(obj: &QObject) -> Result<Vec<TokenStream>, TokenStream> {
    let mut signal_methods = Vec::new();
    let mut queued_cases = Vec::new();
    let ident = &obj.signal_ident;

    for signal in &obj.signals {
        let emit_ident = &signal.emit_ident.rust;
        let enum_ident = &signal.enum_ident;
        let parameters = signal
            .parameters
            .iter()
            .map(|parameter| &parameter.ident)
            .collect::<Vec<&Ident>>();
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

        queued_cases.push(quote! {
            #ident::#enum_ident { #(#parameters),* } => self.#emit_ident(#(#parameters_to_value_queued),*),
        });
    }

    if !queued_cases.is_empty() {
        signal_methods.push(quote! {
            pub fn emit_queued(self: Pin<&mut Self>, signal: #ident) {
                match signal {
                    #(#queued_cases)*
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
    let ident = &invokable.ident.rust;
    let mutablility = if invokable.mutable {
        Some(quote! { mut })
    } else {
        None
    };

    let mut input_parameters = vec![];
    let mut output_parameters = vec![];
    let mut wrappers = vec![];

    let cpp_struct_ident = QObjectName::from(&invokable.qt_ident).cpp_class.rust;
    let cpp_type = if invokable.mutable {
        quote! { std::pin::Pin<&mut #cpp_struct_ident> }
    } else {
        quote! { &#cpp_struct_ident }
    };
    input_parameters.push(quote! { cpp: #cpp_type });

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

    // If we are an opaque return type then we need to convert into the C++ type
    if let Some(return_type) = &invokable.return_type {
        let return_type_ident = return_type.qt_type.cxx_qt_lib_type();

        Ok(quote! {
            pub fn #ident_wrapper(&#mutablility self, #(#input_parameters),*) -> #return_type_ident {
                #(#wrappers)*
                return cpp.#ident(#(#output_parameters),*);
            }
        })
    } else {
        Ok(quote! {
            pub fn #ident_wrapper(&#mutablility self, #(#input_parameters),*) {
                #(#wrappers)*
                cpp.#ident(#(#output_parameters),*);
            }
        })
    }
}

/// Generate the wrapper method for a given property
fn property_generate_wrapper(
    property: &Property,
    cpp_class_name: &Ident,
) -> Result<TokenStream, TokenStream> {
    let property_ident = QPropertyName::from(&property.ident);
    let getter_ident = property_ident.getter.rust;
    let setter_ident = property_ident.setter.rust;

    let qt_type = &property.type_ident.qt_type;
    let qt_type_ident = qt_type.cxx_bridge_type_ident();
    let ty = if qt_type.is_opaque() {
        quote! { UniquePtr<#qt_type_ident> }
    } else {
        quote! { #qt_type_ident }
    };

    Ok(quote! {
        pub fn #getter_ident<'a>(&'a self, cpp: &'a #cpp_class_name) -> &'a #ty {
            cpp.#getter_ident()
        }

        pub fn #setter_ident(&mut self, cpp: Pin<&mut #cpp_class_name>, value: #ty) {
            cpp.#setter_ident(value);
        }
    })
}

/// Generate the method for a given property
fn property_generate(property: &Property) -> Result<TokenStream, TokenStream> {
    let property_ident = QPropertyName::from(&property.ident);
    let getter_ident = property_ident.getter.rust;
    let setter_ident = property_ident.setter.rust;
    let emit_ident = property_ident.emit.rust;
    let ident = property_ident.name.rust;

    let qt_type = &property.type_ident.qt_type;
    let qt_type_ident = qt_type.cxx_bridge_type_ident();
    let ty = if qt_type.is_opaque() {
        quote! { UniquePtr<#qt_type_ident> }
    } else {
        quote! { #qt_type_ident }
    };

    Ok(quote! {
        pub fn #getter_ident(&self) -> &#ty {
            &self.rust().#ident
        }

        pub fn #setter_ident(mut self: Pin<&mut Self>, value: #ty) {
            unsafe {
                self.as_mut().rust_mut().#ident = value;
            }
            self.as_mut().#emit_ident();
        }
    })
}

/// Generate all the Rust code required to communicate with a QObject backed by generated C++ code
pub fn generate_qobject_rs(obj: &QObject) -> Result<TokenStream, TokenStream> {
    // Cache the rust class name
    let qobject_idents = naming::qobject::QObjectName::from(&obj.original_rust_struct.ident);
    let cpp_class_name_rust = qobject_idents.cpp_class.rust;
    let rust_struct_name_rust = qobject_idents.rust_struct.rust;
    let cxx_qt_thread_ident = qobject_idents.cxx_qt_thread_class;

    // Generate cxx block
    let cxx_mod_contents = generate_qobject_cxx(obj)?;

    // Generate property methods from the object
    let signal_methods = generate_signal_methods_rs(obj)?;
    let signal_enum = obj.original_signal_enum.as_ref();

    // Capture methods, trait impls, use decls so they can used by quote
    let invokable_method_wrappers = obj
        .invokables
        .iter()
        .map(|i| invokable_generate_wrapper(i, &i.ident_wrapper.rust))
        .collect::<Result<Vec<TokenStream>, TokenStream>>()?;
    let invokable_methods = obj
        .invokables
        .iter()
        .map(|m| m.original_method.clone())
        .collect::<Vec<syn::ImplItemMethod>>();

    let property_method_wrappers = obj
        .properties
        .iter()
        .map(|p| property_generate_wrapper(p, &cpp_class_name_rust))
        .collect::<Result<Vec<TokenStream>, TokenStream>>()?;
    let property_methods = obj
        .properties
        .iter()
        .map(property_generate)
        .collect::<Result<Vec<TokenStream>, TokenStream>>()?;

    let methods = &obj.methods;

    // Build our filtered rust struct
    let rust_struct = build_struct_with_fields(
        &obj.original_rust_struct,
        &obj.original_rust_struct
            .fields
            .iter()
            .collect::<Vec<&syn::Field>>(),
    );

    let rust_struct_impl = quote! {
        impl #rust_struct_name_rust {
            #(#property_method_wrappers)*
            #(#invokable_method_wrappers)*
        }
    };

    let qobject_impl = quote! {
        impl #cpp_class_name_rust {
            #(#property_methods)*
            #(#invokable_methods)*
            #(#methods)*

            #(#signal_methods)*
        }
    };

    // Create the namespace for internal use
    let namespace_internals =
        naming::namespace::NamespaceName::from_pair_str(&obj.namespace, &obj.ident).internal;

    // TODO: For now we proxy the gen_cpp code into what the writer phase expects
    // later this code will be moved into a generator phase
    let cxx_qt_mod_fake: ItemMod = syn::parse2::<ItemMod>(quote! {
        mod fake {
            #signal_enum

            #rust_struct

            #rust_struct_impl

            #qobject_impl
        }
    })
    .map_err(|err| err.to_compile_error())?;

    // Build the include path for the C++ header
    let cxx_stem = naming::module::cxx_stem_from_ident(&obj.original_rust_struct.ident);
    let import_path = format!("cxx-qt-gen/include/{}.cxxqt.h", cxx_stem);
    let include_line = syn::parse2(quote! {
        unsafe extern "C++" {
            include!(#import_path);
        }
    })
    .map_err(|err| err.to_compile_error())?;

    let qobjects = vec![GeneratedRustQObject {
        cpp_struct_ident: cpp_class_name_rust,
        cxx_qt_thread_ident,
        namespace_internals,
        rust_struct_ident: obj.ident.clone(),
        blocks: GeneratedRustQObjectBlocks {
            cxx_mod_contents,
            cxx_qt_mod_contents: cxx_qt_mod_fake
                .content
                .unwrap_or((syn::token::Brace::default(), vec![]))
                .1,
        },
    }];

    let generated = GeneratedRustBlocks {
        cxx_mod: obj.original_mod.clone(),
        cxx_mod_contents: vec![include_line],
        cxx_qt_mod_contents: obj.original_passthrough_decls.clone(),
        namespace: obj.namespace.to_owned(),
        qobjects,
    };
    Ok(write_rust(&generated))
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
        let qobject = extract_qobject(&module).unwrap();

        let expected_output = include_str!("../test_outputs/custom_default.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_naming() {
        let source = include_str!("../test_inputs/naming.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_output = include_str!("../test_outputs/naming.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_passthrough() {
        let source = include_str!("../test_inputs/passthrough.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_output = include_str!("../test_outputs/passthrough.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_types_primitive_property() {
        let source = include_str!("../test_inputs/types_primitive_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_output = include_str!("../test_outputs/types_primitive_property.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_types_qt_property() {
        let source = include_str!("../test_inputs/types_qt_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_output = include_str!("../test_outputs/types_qt_property.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_types_qt_invokable() {
        let source = include_str!("../test_inputs/types_qt_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_output = include_str!("../test_outputs/types_qt_invokable.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_str_eq!(generated_rs, expected_output);
    }
}
