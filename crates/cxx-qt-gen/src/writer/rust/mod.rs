// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::rust::GeneratedRustBlocks;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// For a given GeneratedRustBlocks write this into a Rust TokenStream
pub fn write_rust(generated: &GeneratedRustBlocks) -> TokenStream {
    // Retrieve the module contents and namespace
    let mut cxx_mod = generated.cxx_mod.clone();
    let mut cxx_mod_contents = generated.cxx_mod_contents.clone();
    let mut cxx_qt_mod_contents = vec![];
    let namespace = &generated.namespace;

    // Add common includes for all objects
    cxx_mod_contents.insert(
        0,
        syn::parse2(quote! {
            unsafe extern "C++" {
                include ! (< QtCore / QObject >);

                include!("cxx-qt-common/cxxqt_connection.h");
                #[doc(hidden)]
                #[namespace = "Qt"]
                // Rename to CxxQtConnectionType so the developer can define it
                // in their bridges without an invisible conflict
                #[rust_name = "CxxQtConnectionType"]
                type ConnectionType = cxx_qt::ConnectionType;

                #[doc(hidden)]
                #[namespace = "rust::cxxqt1"]
                // Rename to CxxQtQMetaObjectConnection so the developer can define it
                // in their bridges without an invisible conflict
                #[rust_name = "CxxQtQMetaObjectConnection"]
                type QMetaObjectConnection = cxx_qt::QMetaObjectConnection;
            }
        })
        .expect("Could not build CXX common block"),
    );

    for fragment in &generated.fragments {
        // Add the blocks from the fragment
        cxx_mod_contents.extend_from_slice(&fragment.cxx_mod_contents);
        cxx_qt_mod_contents.extend_from_slice(&fragment.cxx_qt_mod_contents);
    }

    // Inject the CXX blocks
    if let Some((_, items)) = &mut cxx_mod.content {
        items.extend(cxx_mod_contents);
    } else {
        cxx_mod.content = Some((syn::token::Brace::default(), cxx_mod_contents));
    }

    quote! {
        #[cxx::bridge(namespace = #namespace)]
        #cxx_mod

        #(#cxx_qt_mod_contents)*
    }
    .into_token_stream()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::rust::fragment::GeneratedRustFragment;
    use pretty_assertions::assert_str_eq;
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
            namespace: "cxx_qt::my_object".to_owned(),
            fragments: vec![GeneratedRustFragment {
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
            namespace: "cxx_qt".to_owned(),
            fragments: vec![
                GeneratedRustFragment {
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
                GeneratedRustFragment {
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

                    include!("cxx-qt-common/cxxqt_connection.h");
                    #[doc(hidden)]
                    #[namespace = "Qt"]
                    #[rust_name = "CxxQtConnectionType"]
                    type ConnectionType = cxx_qt::ConnectionType;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqt1"]
                    #[rust_name = "CxxQtQMetaObjectConnection"]
                    type QMetaObjectConnection = cxx_qt::QMetaObjectConnection;
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

            #[derive(Default)]
            pub struct MyObjectRust;

            impl MyObjectRust {
                fn rust_method(&self) {

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

                    include!("cxx-qt-common/cxxqt_connection.h");
                    #[doc(hidden)]
                    #[namespace = "Qt"]
                    #[rust_name = "CxxQtConnectionType"]
                    type ConnectionType = cxx_qt::ConnectionType;

                    #[doc(hidden)]
                    #[namespace = "rust::cxxqt1"]
                    #[rust_name = "CxxQtQMetaObjectConnection"]
                    type QMetaObjectConnection = cxx_qt::QMetaObjectConnection;
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
