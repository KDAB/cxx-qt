// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::rust::{qobject::GeneratedRustQObject, GeneratedRustBlocks};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

/// Mangle an input name with an object name
///
/// For now we need to do this to avoid free Rust methods from colliding
/// Once static methods are possible in CXX this could be removed
/// https://github.com/dtolnay/cxx/issues/447
fn mangle(name: &str, object: &Ident) -> Ident {
    format_ident!("{}", format!("{name}_{object}").to_case(Case::Snake))
}

/// Return common blocks for CXX bridge which the C++ writer adds as well
fn cxx_bridge_common_blocks(qobject: &GeneratedRustQObject) -> Vec<TokenStream> {
    let cpp_struct_ident = &qobject.cpp_struct_ident;
    let rust_struct_ident = &qobject.rust_struct_ident;
    let namespace_internals = &qobject.namespace_internals;

    let create_rs_ident = mangle("create_rs", rust_struct_ident);

    vec![
        quote! {
            unsafe extern "C++" {
                #[cxx_name = "unsafeRust"]
                #[doc(hidden)]
                fn cxx_qt_ffi_rust(self: &#cpp_struct_ident) -> &#rust_struct_ident;
            }
        },
        quote! {
            unsafe extern "C++" {
                #[cxx_name = "unsafeRustMut"]
                #[doc(hidden)]
                fn cxx_qt_ffi_rust_mut(self: Pin<&mut #cpp_struct_ident>) -> Pin<&mut #rust_struct_ident>;
            }
        },
        quote! {
            extern "Rust" {
                #[cxx_name = "createRs"]
                #[namespace = #namespace_internals]
                fn #create_rs_ident() -> Box<#rust_struct_ident>;
            }
        },
    ]
}

/// Return common blocks for CXX-Qt implementation which the C++ writer adds as well
fn cxx_qt_common_blocks(qobject: &GeneratedRustQObject) -> Vec<TokenStream> {
    let cpp_struct_ident = &qobject.cpp_struct_ident;
    let rust_struct_ident = &qobject.rust_struct_ident;
    let create_rs_ident = mangle("create_rs", rust_struct_ident);

    vec![
        quote! {
            impl cxx_qt::CxxQtType for #cpp_struct_ident {
                type Rust = #rust_struct_ident;

                fn rust(&self) -> &Self::Rust {
                    self.cxx_qt_ffi_rust()
                }

                fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
                    self.cxx_qt_ffi_rust_mut()
                }
            }
        },
        quote! {
            /// Generated CXX-Qt method which creates a boxed rust struct of a QObject
            pub fn #create_rs_ident() -> std::boxed::Box<#rust_struct_ident> {
                std::default::Default::default()
            }
        },
    ]
}

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
        // Add the common blocks into the bridge which we need
        cxx_mod_contents.extend_from_slice(&qobject.blocks.cxx_mod_contents);
        cxx_mod_contents.append(
            &mut cxx_bridge_common_blocks(qobject)
                .into_iter()
                .map(|block| syn::parse2(block).expect("Could not build CXX common block"))
                .collect(),
        );

        // Inject the common blocks into the implementation we need
        cxx_qt_mod_contents.extend_from_slice(&qobject.blocks.cxx_qt_mod_contents);
        cxx_qt_mod_contents.append(
            &mut cxx_qt_common_blocks(qobject)
                .into_iter()
                .map(|block| syn::parse2(block).expect("Could not build CXX-Qt common block"))
                .collect(),
        );

        // Add the type alias to the C++ struct
        let cpp_struct_ident = &qobject.cpp_struct_ident;
        let rust_struct_ident = &qobject.rust_struct_ident;
        let rust_struct_ident_str = rust_struct_ident.to_string();
        qobject_types.push(quote! {
            #[doc = "The C++ type for the QObject "]
            #[doc = #rust_struct_ident_str]
            #[doc = "\n"]
            #[doc = "Use this type when referring to the QObject as a pointer"]
            #[doc = "\n"]
            #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
            pub type #rust_struct_ident = super::#cpp_struct_ident;
        })
    }

    // Create the qobject block for the type alias
    cxx_qt_mod_contents.push(
        syn::parse2(quote! {
            /// Generated CXX-Qt module containing type alias to the C++ types of the QObjects
            pub mod qobject {
                #(#qobject_types)*
            }
        })
        .expect("Could not build qobject block"),
    );

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
            use super::#cxx_mod_ident::*;
            use std::pin::Pin;
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
                cpp_struct_ident: format_ident!("MyObjectQt"),
                namespace_internals: "cxx_qt::my_object::cxx_qt_my_object".to_owned(),
                rust_struct_ident: format_ident!("MyObject"),
                blocks: GeneratedRustQObjectBlocks {
                    cxx_mod_contents: vec![
                        parse_quote! {
                            unsafe extern "C++" {
                                #[cxx_name = "MyObject"]
                                type MyObjectQt;
                            }
                        },
                        parse_quote! {
                            extern "Rust" {
                                #[cxx_name = "MyObjectRust"]
                                type MyObject;
                            }
                        },
                    ],
                    cxx_qt_mod_contents: vec![
                        parse_quote! {
                            #[derive(Default)]
                            pub struct MyObject;
                        },
                        parse_quote! {
                            impl MyObject {
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
                    cpp_struct_ident: format_ident!("FirstObjectQt"),
                    namespace_internals: "cxx_qt::cxx_qt_first_object".to_owned(),
                    rust_struct_ident: format_ident!("FirstObject"),
                    blocks: GeneratedRustQObjectBlocks {
                        cxx_mod_contents: vec![
                            parse_quote! {
                                unsafe extern "C++" {
                                    #[cxx_name = "FirstObject"]
                                    type FirstObjectQt;
                                }
                            },
                            parse_quote! {
                                extern "Rust" {
                                    #[cxx_name = "FirstObjectRust"]
                                    type FirstObject;
                                }
                            },
                        ],
                        cxx_qt_mod_contents: vec![
                            parse_quote! {
                                #[derive(Default)]
                                pub struct FirstObject;
                            },
                            parse_quote! {
                                impl FirstObject {
                                    fn rust_method(&self) {

                                    }
                                }
                            },
                        ],
                    },
                },
                GeneratedRustQObject {
                    cpp_struct_ident: format_ident!("SecondObjectQt"),
                    namespace_internals: "cxx_qt::cxx_qt_second_object".to_owned(),
                    rust_struct_ident: format_ident!("SecondObject"),
                    blocks: GeneratedRustQObjectBlocks {
                        cxx_mod_contents: vec![
                            parse_quote! {
                                unsafe extern "C++" {
                                    #[cxx_name = "SecondObject"]
                                    type SecondObjectQt;
                                }
                            },
                            parse_quote! {
                                extern "Rust" {
                                    #[cxx_name = "SecondObjectRust"]
                                    type SecondObject;
                                }
                            },
                        ],
                        cxx_qt_mod_contents: vec![
                            parse_quote! {
                                #[derive(Default)]
                                pub struct SecondObject;
                            },
                            parse_quote! {
                                impl SecondObject {
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
                    #[cxx_name = "MyObject"]
                    type MyObjectQt;
                }

                extern "Rust" {
                    #[cxx_name = "MyObjectRust"]
                    type MyObject;
                }

                unsafe extern "C++" {
                    #[cxx_name = "unsafeRust"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_rust(self: &MyObjectQt) -> &MyObject;
                }

                unsafe extern "C++" {
                    #[cxx_name = "unsafeRustMut"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
                }

                extern "Rust" {
                    #[cxx_name = "createRs"]
                    #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
                    fn create_rs_my_object() -> Box<MyObject>;
                }
            }

            use self::cxx_qt_ffi::*;
            #[doc = r" Internal CXX-Qt module, made public temporarily between API changes"]
            pub mod cxx_qt_ffi {
                use super::ffi::*;
                use std::pin::Pin;
                use cxx_qt::CxxQtType;

                #[doc(hidden)]
                type UniquePtr<T> = cxx::UniquePtr<T>;

                use module::Struct;

                #[derive(Default)]
                pub struct MyObject;

                impl MyObject {
                    fn rust_method(&self) {

                    }
                }

                impl cxx_qt::CxxQtType for MyObjectQt {
                    type Rust = MyObject;
                    fn rust(&self) -> &Self::Rust {
                        self.cxx_qt_ffi_rust()
                    }
                    fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
                        self.cxx_qt_ffi_rust_mut()
                    }
                }

                /// Generated CXX-Qt method which creates a boxed rust struct of a QObject
                pub fn create_rs_my_object() -> std::boxed::Box<MyObject> {
                    std::default::Default::default()
                }

                /// Generated CXX-Qt module containing type alias to the C++ types of the QObjects
                pub mod qobject {
                    #[doc = "The C++ type for the QObject "]
                    #[doc = "MyObject"]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
                    pub type MyObject = super::MyObjectQt;
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
                    #[cxx_name = "FirstObject"]
                    type FirstObjectQt;
                }

                extern "Rust" {
                    #[cxx_name = "FirstObjectRust"]
                    type FirstObject;
                }

                unsafe extern "C++" {
                    #[cxx_name = "unsafeRust"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_rust(self: &FirstObjectQt) -> &FirstObject;
                }

                unsafe extern "C++" {
                    #[cxx_name = "unsafeRustMut"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_rust_mut(self: Pin<&mut FirstObjectQt>) -> Pin<&mut FirstObject>;
                }

                extern "Rust" {
                    #[cxx_name = "createRs"]
                    #[namespace = "cxx_qt::cxx_qt_first_object"]
                    fn create_rs_first_object() -> Box<FirstObject>;
                }
                unsafe extern "C++" {
                    #[cxx_name = "SecondObject"]
                    type SecondObjectQt;
                }

                extern "Rust" {
                    #[cxx_name = "SecondObjectRust"]
                    type SecondObject;
                }

                unsafe extern "C++" {
                    #[cxx_name = "unsafeRust"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_rust(self: &SecondObjectQt) -> &SecondObject;
                }

                unsafe extern "C++" {
                    #[cxx_name = "unsafeRustMut"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_rust_mut(self: Pin<&mut SecondObjectQt>) -> Pin<&mut SecondObject>;
                }

                extern "Rust" {
                    #[cxx_name = "createRs"]
                    #[namespace = "cxx_qt::cxx_qt_second_object"]
                    fn create_rs_second_object() -> Box<SecondObject>;
                }
            }

            use self::cxx_qt_ffi::*;
            #[doc = r" Internal CXX-Qt module, made public temporarily between API changes"]
            pub mod cxx_qt_ffi {
                use super::ffi::*;
                use std::pin::Pin;
                use cxx_qt::CxxQtType;

                #[doc(hidden)]
                type UniquePtr<T> = cxx::UniquePtr<T>;

                use module::Struct;

                #[derive(Default)]
                pub struct FirstObject;

                impl FirstObject {
                    fn rust_method(&self) {

                    }
                }

                impl cxx_qt::CxxQtType for FirstObjectQt {
                    type Rust = FirstObject;
                    fn rust(&self) -> &Self::Rust {
                        self.cxx_qt_ffi_rust()
                    }
                    fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
                        self.cxx_qt_ffi_rust_mut()
                    }
                }

                /// Generated CXX-Qt method which creates a boxed rust struct of a QObject
                pub fn create_rs_first_object() -> std::boxed::Box<FirstObject> {
                    std::default::Default::default()
                }

                #[derive(Default)]
                pub struct SecondObject;

                impl SecondObject {
                    fn rust_method(&self) {

                    }
                }

                impl cxx_qt::CxxQtType for SecondObjectQt {
                    type Rust = SecondObject;
                    fn rust(&self) -> &Self::Rust {
                        self.cxx_qt_ffi_rust()
                    }
                    fn rust_mut(self: core::pin::Pin<&mut Self>) -> Pin<&mut Self::Rust> {
                        self.cxx_qt_ffi_rust_mut()
                    }
                }

                /// Generated CXX-Qt method which creates a boxed rust struct of a QObject
                pub fn create_rs_second_object() -> std::boxed::Box<SecondObject> {
                    std::default::Default::default()
                }

                /// Generated CXX-Qt module containing type alias to the C++ types of the QObjects
                pub mod qobject {
                    #[doc = "The C++ type for the QObject "]
                    #[doc = "FirstObject"]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
                    pub type FirstObject = super::FirstObjectQt;
                    #[doc = "The C++ type for the QObject "]
                    #[doc = "SecondObject"]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
                    pub type SecondObject = super::SecondObjectQt;
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
