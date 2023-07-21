// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::rust::GeneratedRustBlocks;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

/// For a given GeneratedRustBlocks write this into a Rust TokenStream
pub fn write_rust(generated: &GeneratedRustBlocks) -> TokenStream {
    // Build the module idents
    let cxx_mod_ident = &generated.cxx_mod.ident;
    let cxx_qt_mod_ident = format_ident!("cxx_qt_{cxx_mod_ident}");

    // Retrieve the module contents and namespace
    let mut cxx_mod = generated.cxx_mod.clone();
    let mut cxx_mod_contents = generated.cxx_mod_contents.clone();
    let mut cxx_qt_mod_contents = generated.cxx_qt_mod_contents.clone();
    let namespace = &generated.namespace;

    // Add common includes for all objects
    cxx_mod_contents.insert(
        0,
        syn::parse2(quote! {
            unsafe extern "C++" {
                include ! (< QtCore / QObject >);

                include!("cxx-qt-lib/qt.h");
                #[doc(hidden)]
                #[namespace = "Qt"]
                #[rust_name = "CxxQtConnectionType"]
                type ConnectionType = cxx_qt_lib::ConnectionType;

                include!("cxx-qt-lib/qmetaobjectconnection.h");
                #[doc(hidden)]
                #[namespace = "rust::cxxqtlib1"]
                // Rename to CxxQtQMetaObjectConnection so the developer can define it
                // in their bridges without an invisible conflict
                #[rust_name = "CxxQtQMetaObjectConnection"]
                type QMetaObjectConnection = cxx_qt_lib::QMetaObjectConnection;
            }
        })
        .expect("Could not build CXX common block"),
    );

    let mut qobject_types = vec![];
    for qobject in &generated.qobjects {
        // Add the blocks from the QObject
        cxx_mod_contents.extend_from_slice(&qobject.blocks.cxx_mod_contents);
        cxx_qt_mod_contents.extend_from_slice(&qobject.blocks.cxx_qt_mod_contents);

        // Add the type alias to the C++ struct
        let cpp_struct_ident = &qobject.cpp_struct_ident;
        let cpp_struct_ident_str = cpp_struct_ident.to_string();
        qobject_types.push(quote! {
            #[doc = "The C++ type for the QObject "]
            #[doc = #cpp_struct_ident_str]
            #[doc = "\n"]
            #[doc = "Use this type when referring to the QObject as a pointer"]
            #[doc = "\n"]
            #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
            pub type #cpp_struct_ident = super::#cpp_struct_ident;
        })
    }

    // Inject the CXX blocks
    if let Some((_, items)) = &mut cxx_mod.content {
        items.extend(cxx_mod_contents.into_iter());
    } else {
        cxx_mod.content = Some((syn::token::Brace::default(), cxx_mod_contents));
    }

    // Copy the visiblity of the module so we re-export things in the same way
    let cxx_mod_visiblity = &generated.cxx_mod.vis;

    quote! {
        #[cxx::bridge(namespace = #namespace)]
        #cxx_mod

        #cxx_mod_visiblity use self::#cxx_qt_mod_ident::*;
        // TODO: for now mark as public
        // as we need to reach the generated getters and setters
        // but later we'll likely implement things outside the module
        //
        /// Internal CXX-Qt module, made public temporarily between API changes
        pub mod #cxx_qt_mod_ident {
            // Temporary hack so that qualified types still work, this will be removed in the next commit
            use super::*;
            use super::#cxx_mod_ident::*;
            use cxx_qt::CxxQtType;

            #[doc(hidden)]
            type UniquePtr<T> = cxx::UniquePtr<T>;

            #(#cxx_qt_mod_contents)*
        }
    }
    .into_token_stream()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::rust::qobject::{GeneratedRustQObject, GeneratedRustQObjectBlocks};
    use pretty_assertions::assert_str_eq;
    use quote::format_ident;
    use syn::parse_quote;

    /// Helper to create a GeneratedRustBlocks for testing
    pub fn create_generated_rust() -> GeneratedRustBlocks {
        GeneratedRustBlocks {
            cxx_mod: parse_quote! {
                mod ffi {}
            },
            cxx_mod_contents: vec![parse_quote! {
                unsafe extern "C++" {
                    include!("myobject.cxxqt.h");
                }
            }],
            cxx_qt_mod_contents: vec![parse_quote! {
                use module::Struct;
            }],
            namespace: "cxx_qt::my_object".to_owned(),
            qobjects: vec![GeneratedRustQObject {
                cpp_struct_ident: format_ident!("MyObject"),
                namespace_internals: "cxx_qt::my_object::cxx_qt_my_object".to_owned(),
                rust_struct_ident: format_ident!("MyObjectRust"),
                blocks: GeneratedRustQObjectBlocks {
                    cxx_mod_contents: vec![
                        parse_quote! {
                            unsafe extern "C++" {
                                type MyObject;
                            }
                        },
                        parse_quote! {
                            extern "Rust" {
                                type MyObjectRust;
                            }
                        },
                    ],
                    cxx_qt_mod_contents: vec![
                        parse_quote! {
                            #[derive(Default)]
                            pub struct MyObjectRust;
                        },
                        parse_quote! {
                            impl MyObjectRust {
                                fn rust_method(&self) {

                                }
                            }
                        },
                    ],
                },
            }],
        }
    }

    /// Helper to create a GeneratedRustBlocks for testing with multiple qobjects
    pub fn create_generated_rust_multi_qobjects() -> GeneratedRustBlocks {
        GeneratedRustBlocks {
            cxx_mod: parse_quote! {
                mod ffi {}
            },
            cxx_mod_contents: vec![parse_quote! {
                unsafe extern "C++" {
                    include!("multiobject.cxxqt.h");
                }
            }],
            cxx_qt_mod_contents: vec![parse_quote! {
                use module::Struct;
            }],
            namespace: "cxx_qt".to_owned(),
            qobjects: vec![
                GeneratedRustQObject {
                    cpp_struct_ident: format_ident!("FirstObject"),
                    namespace_internals: "cxx_qt::cxx_qt_first_object".to_owned(),
                    rust_struct_ident: format_ident!("FirstObjectRust"),
                    blocks: GeneratedRustQObjectBlocks {
                        cxx_mod_contents: vec![
                            parse_quote! {
                                unsafe extern "C++" {
                                    type FirstObject;
                                }
                            },
                            parse_quote! {
                                extern "Rust" {
                                    type FirstObjectRust;
                                }
                            },
                        ],
                        cxx_qt_mod_contents: vec![
                            parse_quote! {
                                #[derive(Default)]
                                pub struct FirstObjectRust;
                            },
                            parse_quote! {
                                impl FirstObjectRust {
                                    fn rust_method(&self) {

                                    }
                                }
                            },
                        ],
                    },
                },
                GeneratedRustQObject {
                    cpp_struct_ident: format_ident!("SecondObject"),
                    namespace_internals: "cxx_qt::cxx_qt_second_object".to_owned(),
                    rust_struct_ident: format_ident!("SecondObjectRust"),
                    blocks: GeneratedRustQObjectBlocks {
                        cxx_mod_contents: vec![
                            parse_quote! {
                                unsafe extern "C++" {
                                    type SecondObject;
                                }
                            },
                            parse_quote! {
                                extern "Rust" {
                                    type SecondObjectRust;
                                }
                            },
                        ],
                        cxx_qt_mod_contents: vec![
                            parse_quote! {
                                #[derive(Default)]
                                pub struct SecondObjectRust;
                            },
                            parse_quote! {
                                impl SecondObjectRust {
                                    fn rust_method(&self) {

                                    }
                                }
                            },
                        ],
                    },
                },
            ],
        }
    }

    /// Helper for the expected Rust
    pub fn expected_rust() -> String {
        quote! {
            #[cxx::bridge(namespace = "cxx_qt::my_object")]
            mod ffi {
                unsafe extern "C++" {
                    include ! (< QtCore / QObject >);

                    include!("cxx-qt-lib/qt.h");
                    #[doc(hidden)]
                    #[namespace = "Qt"]
                    #[rust_name = "CxxQtConnectionType"]
                    type ConnectionType = cxx_qt_lib::ConnectionType;

                    include!("cxx-qt-lib/qmetaobjectconnection.h");
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtlib1"]
                    #[rust_name = "CxxQtQMetaObjectConnection"]
                    type QMetaObjectConnection = cxx_qt_lib::QMetaObjectConnection;
                }

                unsafe extern "C++" {
                    include!("myobject.cxxqt.h");
                }

                unsafe extern "C++" {
                    type MyObject;
                }

                extern "Rust" {
                    type MyObjectRust;
                }
            }

            use self::cxx_qt_ffi::*;
            #[doc = r" Internal CXX-Qt module, made public temporarily between API changes"]
            pub mod cxx_qt_ffi {
                use super::*;
                use super::ffi::*;
                use cxx_qt::CxxQtType;

                #[doc(hidden)]
                type UniquePtr<T> = cxx::UniquePtr<T>;

                use module::Struct;

                #[derive(Default)]
                pub struct MyObjectRust;

                impl MyObjectRust {
                    fn rust_method(&self) {

                    }
                }
            }
        }
        .into_token_stream()
        .to_string()
    }

    /// Helper for the expected Rust with multiple qobjects
    pub fn expected_rust_multi_qobjects() -> String {
        quote! {
            #[cxx::bridge(namespace = "cxx_qt")]
            mod ffi {
                unsafe extern "C++" {
                    include ! (< QtCore / QObject >);

                    include!("cxx-qt-lib/qt.h");
                    #[doc(hidden)]
                    #[namespace = "Qt"]
                    #[rust_name = "CxxQtConnectionType"]
                    type ConnectionType = cxx_qt_lib::ConnectionType;

                    include!("cxx-qt-lib/qmetaobjectconnection.h");
                    #[doc(hidden)]
                    #[namespace = "rust::cxxqtlib1"]
                    #[rust_name = "CxxQtQMetaObjectConnection"]
                    type QMetaObjectConnection = cxx_qt_lib::QMetaObjectConnection;
                }

                unsafe extern "C++" {
                    include!("multiobject.cxxqt.h");
                }

                unsafe extern "C++" {
                    type FirstObject;
                }

                extern "Rust" {
                    type FirstObjectRust;
                }

                unsafe extern "C++" {
                    type SecondObject;
                }

                extern "Rust" {
                    type SecondObjectRust;
                }
            }

            use self::cxx_qt_ffi::*;
            #[doc = r" Internal CXX-Qt module, made public temporarily between API changes"]
            pub mod cxx_qt_ffi {
                use super::*;
                use super::ffi::*;
                use cxx_qt::CxxQtType;

                #[doc(hidden)]
                type UniquePtr<T> = cxx::UniquePtr<T>;

                use module::Struct;

                #[derive(Default)]
                pub struct FirstObjectRust;

                impl FirstObjectRust {
                    fn rust_method(&self) {

                    }
                }

                #[derive(Default)]
                pub struct SecondObjectRust;

                impl SecondObjectRust {
                    fn rust_method(&self) {

                    }
                }
            }
        }
        .into_token_stream()
        .to_string()
    }

    #[test]
    fn test_write_rust() {
        let generated = create_generated_rust();
        let result = write_rust(&generated);
        assert_str_eq!(result.to_string(), expected_rust());
    }

    #[test]
    fn test_write_rust_multi_qobjects() {
        let generated = create_generated_rust_multi_qobjects();
        let result = write_rust(&generated);
        assert_str_eq!(result.to_string(), expected_rust_multi_qobjects());
    }
}
