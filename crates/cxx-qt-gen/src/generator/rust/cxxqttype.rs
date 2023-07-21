// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{naming::qobject::QObjectName, rust::qobject::GeneratedRustQObjectBlocks};
use quote::quote;
use syn::Result;

use super::fragment::RustFragmentPair;

pub fn generate(qobject_ident: &QObjectName) -> Result<GeneratedRustQObjectBlocks> {
    let mut blocks = GeneratedRustQObjectBlocks::default();

    let cpp_struct_ident = &qobject_ident.cpp_class.rust;
    let rust_struct_ident = &qobject_ident.rust_struct.rust;

    let fragment = RustFragmentPair {
        cxx_bridge: vec![
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
        ],
        implementation: vec![
            quote! {
                impl core::ops::Deref for #cpp_struct_ident {
                    type Target = #rust_struct_ident;

                    fn deref(&self) -> &Self::Target {
                        self.cxx_qt_ffi_rust()
                    }
                }
            },
            quote! {
                impl cxx_qt::CxxQtType for #cpp_struct_ident {
                    type Rust = #rust_struct_ident;

                    fn rust(&self) -> &Self::Rust {
                        self.cxx_qt_ffi_rust()
                    }

                    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
                        self.cxx_qt_ffi_rust_mut()
                    }
                }
            },
        ],
    };

    blocks
        .cxx_mod_contents
        .append(&mut fragment.cxx_bridge_as_items()?);
    blocks
        .cxx_qt_mod_contents
        .append(&mut fragment.implementation_as_items()?);

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::assert_tokens_eq;

    use crate::parser::qobject::tests::create_parsed_qobject;

    #[test]
    fn test_generate_rust_cxxqttype() {
        let qobject = create_parsed_qobject();
        let qobject_idents = QObjectName::from(&qobject);

        let generated = generate(&qobject_idents).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 2);

        // CXX bridges

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "unsafeRust"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_rust(self: &MyObject) -> &MyObjectRust;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "unsafeRustMut"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_rust_mut(self: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
                }
            },
        );

        // CXX-Qt generated contents
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl core::ops::Deref for MyObject {
                    type Target = MyObjectRust;

                    fn deref(&self) -> &Self::Target {
                        self.cxx_qt_ffi_rust()
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl cxx_qt::CxxQtType for MyObject {
                    type Rust = MyObjectRust;

                    fn rust(&self) -> &Self::Rust {
                        self.cxx_qt_ffi_rust()
                    }

                    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
                        self.cxx_qt_ffi_rust_mut()
                    }
                }
            },
        );
    }
}
