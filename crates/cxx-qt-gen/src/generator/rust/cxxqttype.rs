// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{naming::qobject::QObjectNames, rust::fragment::GeneratedRustFragment},
    naming::TypeNames,
};
use quote::quote;
use syn::{Ident, Result};

use super::fragment::RustFragmentPair;

pub fn generate(
    qobject_ident: &QObjectNames,
    type_names: &TypeNames,
    module_ident: &Ident,
) -> Result<GeneratedRustFragment> {
    let mut blocks = GeneratedRustFragment::default();

    let cpp_struct_ident = &qobject_ident.name.rust_unqualified();
    let rust_struct_ident = &qobject_ident.rust_struct.rust_unqualified();
    let rust_fn = qobject_ident.cxx_qt_ffi_method("rust");
    let rust_mut_fn = qobject_ident.cxx_qt_ffi_method("rust_mut");
    let qualified_impl = type_names.rust_qualified(cpp_struct_ident)?;

    let fragment = RustFragmentPair {
        cxx_bridge: vec![
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "unsafeRust"]
                    #[namespace = "rust::cxxqt1"]
                    #[doc(hidden)]
                    fn #rust_fn(outer: &#cpp_struct_ident) -> &#rust_struct_ident;
                }
            },
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "unsafeRustMut"]
                    #[namespace = "rust::cxxqt1"]
                    #[doc(hidden)]
                    fn #rust_mut_fn(outer: Pin<&mut #cpp_struct_ident>) -> Pin<&mut #rust_struct_ident>;
                }
            },
        ],
        implementation: vec![
            quote! {
                impl ::core::ops::Deref for #qualified_impl {
                    type Target = #rust_struct_ident;

                    fn deref(&self) -> &Self::Target {
                        #module_ident::#rust_fn(self)
                    }
                }
            },
            quote! {
                impl ::cxx_qt::CxxQtType for #qualified_impl {
                    type Rust = #rust_struct_ident;

                    fn rust(&self) -> &Self::Rust {
                        #module_ident::#rust_fn(self)
                    }

                    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
                        #module_ident::#rust_mut_fn(self)
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

    use quote::format_ident;

    #[test]
    fn test_generate_rust_cxxqttype() {
        let qobject = create_parsed_qobject();
        let qobject_idents = QObjectNames::from_qobject(&qobject, &TypeNames::mock()).unwrap();

        let generated = generate(
            &qobject_idents,
            &TypeNames::mock(),
            &format_ident!("qobject"),
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 2);

        // CXX bridges

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "unsafeRust"]
                    #[namespace = "rust::cxxqt1"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_my_object_rust(outer: &MyObject) -> &MyObjectRust;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "unsafeRustMut"]
                    #[namespace = "rust::cxxqt1"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_my_object_rust_mut(outer: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
                }
            },
        );

        // CXX-Qt generated contents
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                impl ::core::ops::Deref for qobject::MyObject {
                    type Target = MyObjectRust;

                    fn deref(&self) -> &Self::Target {
                        qobject::cxx_qt_ffi_my_object_rust(self)
                    }
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[1],
            quote! {
                impl ::cxx_qt::CxxQtType for qobject::MyObject {
                    type Rust = MyObjectRust;

                    fn rust(&self) -> &Self::Rust {
                        qobject::cxx_qt_ffi_my_object_rust(self)
                    }

                    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
                        qobject::cxx_qt_ffi_my_object_rust_mut(self)
                    }
                }
            },
        );
    }
}
