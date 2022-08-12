// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::rust::GeneratedRustBlocks;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

/// Return common blocks for CXX bridge which the C++ writer adds as well
fn cxx_common_blocks(
    cpp_struct_ident: &Ident,
    rust_struct_ident: &Ident,
    namespace_internals: &String,
) -> Vec<TokenStream> {
    vec![
        quote! {
            unsafe extern "C++" {
                include!("cxx-qt-lib/include/convert.h");

                #[cxx_name = "unsafeRust"]
                fn rust(self: &#cpp_struct_ident) -> &#rust_struct_ident;

                #[rust_name = "new_cpp_object"]
                #[namespace = #namespace_internals]
                fn newCppObject() -> UniquePtr<#cpp_struct_ident>;
            }
        },
        quote! {
            extern "C++" {
                #[cxx_name = "unsafeRustMut"]
                unsafe fn rust_mut(self: Pin<&mut #cpp_struct_ident>) -> Pin<&mut #rust_struct_ident>;
            }
        },
        quote! {
            extern "Rust" {
                #[cxx_name = "createRs"]
                #[namespace = #namespace_internals]
                fn create_rs() -> Box<#rust_struct_ident>;

                #[cxx_name = "initialiseCpp"]
                #[namespace = #namespace_internals]
                fn initialise_cpp(cpp: Pin<&mut #cpp_struct_ident>);
            }
        },
    ]
}

/// For a given GeneratedRustBlocks write this into a Rust TokenStream
pub fn write_rust(generated: &GeneratedRustBlocks) -> TokenStream {
    // Retrieve the struct idents
    let cpp_struct_ident = &generated.cpp_struct_ident;
    let rust_struct_ident = &generated.rust_struct_ident;

    // Build the module idents
    let cxx_mod_ident = &generated.cxx_mod.ident;
    let cxx_qt_mod_ident = format_ident!("cxx_qt_{}", cxx_mod_ident);

    // Retrieve the module contents and namespace
    let mut cxx_mod = generated.cxx_mod.clone();
    let cxx_qt_mod_contents = &generated.cxx_qt_mod_contents;
    let namespace = &generated.namespace;
    let namespace_internals = &generated.namespace_internals;

    // Inject the common blocks into the bridge which we need
    let cxx_mod_items = &mut cxx_mod.content.as_mut().expect("").1;
    for block in cxx_common_blocks(cpp_struct_ident, rust_struct_ident, namespace_internals) {
        cxx_mod_items.push(syn::parse2(block).expect("Could not build CXX common block"));
    }

    quote! {
        #[cxx::bridge(namespace = #namespace)]
        #cxx_mod

        pub use self::#cxx_qt_mod_ident::*;
        mod #cxx_qt_mod_ident {
            use super::#cxx_mod_ident::*;

            pub type FFICppObj = super::#cxx_mod_ident::#cpp_struct_ident;
            type UniquePtr<T> = cxx::UniquePtr<T>;

            #(#cxx_qt_mod_contents)*

            pub fn create_rs() -> std::boxed::Box<#rust_struct_ident> {
                std::default::Default::default()
            }

            pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
                let mut wrapper = CppObj::new(cpp);
                wrapper.grab_values_from_data(Data::default());
            }
        }
    }
    .into_token_stream()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use pretty_assertions::assert_str_eq;
    use quote::format_ident;

    /// Helper to create a GeneratedRustBlocks for testing
    pub fn create_generated_rust() -> GeneratedRustBlocks {
        GeneratedRustBlocks {
            cxx_mod: tokens_to_syn(quote! {
                mod ffi {
                    unsafe extern "C++" {
                        #[cxx_name = "MyObject"]
                        type MyObjectQt;
                    }

                    extern "Rust" {
                        #[cxx_name = "MyObjectRust"]
                        type MyObject;
                    }
                }
            }),
            cxx_qt_mod_contents: vec![
                tokens_to_syn(quote! {
                    #[derive(Default)]
                    pub struct MyObject;
                }),
                tokens_to_syn(quote! {
                    impl MyObject {
                        fn rust_method(&self) {

                        }
                    }
                }),
            ],
            cpp_struct_ident: format_ident!("MyObjectQt"),
            namespace: "cxx_qt::my_object".to_owned(),
            namespace_internals: "cxx_qt::my_object::cxx_qt_my_object".to_owned(),
            rust_struct_ident: format_ident!("MyObject"),
        }
    }

    /// Helper for the expected Rust
    pub fn expected_rust() -> String {
        quote! {
            #[cxx::bridge(namespace = "cxx_qt::my_object")]
            mod ffi {
                unsafe extern "C++" {
                    #[cxx_name = "MyObject"]
                    type MyObjectQt;
                }

                extern "Rust" {
                    #[cxx_name = "MyObjectRust"]
                    type MyObject;
                }

                unsafe extern "C++" {
                    include!("cxx-qt-lib/include/convert.h");

                    #[cxx_name = "unsafeRust"]
                    fn rust(self: &MyObjectQt) -> &MyObject;

                    #[rust_name = "new_cpp_object"]
                    #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
                    fn newCppObject() -> UniquePtr<MyObjectQt>;
                }

                extern "C++" {
                    #[cxx_name = "unsafeRustMut"]
                    unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
                }

                extern "Rust" {
                    #[cxx_name = "createRs"]
                    #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
                    fn create_rs() -> Box<MyObject>;

                    #[cxx_name = "initialiseCpp"]
                    #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
                    fn initialise_cpp(cpp: Pin<&mut MyObjectQt>);
                }
            }

            pub use self::cxx_qt_ffi::*;
            mod cxx_qt_ffi {
                use super::ffi::*;

                pub type FFICppObj = super::ffi::MyObjectQt;
                type UniquePtr<T> = cxx::UniquePtr<T>;

                #[derive(Default)]
                pub struct MyObject;

                impl MyObject {
                    fn rust_method(&self) {

                    }
                }

                pub fn create_rs() -> std::boxed::Box<MyObject> {
                    std::default::Default::default()
                }

                pub fn initialise_cpp(cpp: std::pin::Pin<&mut FFICppObj>) {
                    let mut wrapper = CppObj::new(cpp);
                    wrapper.grab_values_from_data(Data::default());
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
}
