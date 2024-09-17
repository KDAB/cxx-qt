// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{naming::qobject::QObjectNames, rust::fragment::GeneratedRustFragment},
    naming::TypeNames,
};
use quote::quote;
use syn::Result;

use super::fragment::RustFragmentPair;

pub fn generate(
    qobject_names: &QObjectNames,
    type_names: &TypeNames,
) -> Result<GeneratedRustFragment> {
    let mut blocks = GeneratedRustFragment::default();

    let cpp_struct_ident = &qobject_names.name.rust_unqualified();
    let rust_struct_ident = &qobject_names.rust_struct.rust_unqualified();
    let (rust_fn_name, rust_fn_attrs, rust_fn_qualified) = qobject_names
        .cxx_qt_ffi_method("unsafeRust")
        .into_cxx_parts();

    let (rust_mut_fn_name, rust_mut_fn_attrs, rust_mut_fn_qualified) = qobject_names
        .cxx_qt_ffi_method("unsafeRustMut")
        .into_cxx_parts();

    let qualified_impl = type_names.rust_qualified(cpp_struct_ident)?;

    let fragment = RustFragmentPair {
        cxx_bridge: vec![
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #(#rust_fn_attrs)*
                    fn #rust_fn_name(outer: &#cpp_struct_ident) -> &#rust_struct_ident;
                }
            },
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #(#rust_mut_fn_attrs)*
                    fn #rust_mut_fn_name(outer: Pin<&mut #cpp_struct_ident>) -> Pin<&mut #rust_struct_ident>;
                }
            },
        ],
        implementation: vec![
            quote! {
                impl ::core::ops::Deref for #qualified_impl {
                    type Target = #rust_struct_ident;

                    fn deref(&self) -> &Self::Target {
                        #rust_fn_qualified(self)
                    }
                }
            },
            quote! {
                impl ::cxx_qt::CxxQtType for #qualified_impl {
                    type Rust = #rust_struct_ident;

                    fn rust(&self) -> &Self::Rust {
                        #rust_fn_qualified(self)
                    }

                    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
                        #rust_mut_fn_qualified(self)
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
        let qobject_names = QObjectNames::from_qobject(&qobject, &TypeNames::mock()).unwrap();

        let generated = generate(&qobject_names, &TypeNames::mock()).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 2);

        // CXX bridges

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[cxx_name = "unsafeRust"]
                    #[namespace = "rust::cxxqt1"]
                    fn cxx_qt_ffi_MyObject_unsafeRust(outer: &MyObject) -> &MyObjectRust;
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[cxx_name = "unsafeRustMut"]
                    #[namespace = "rust::cxxqt1"]
                    fn cxx_qt_ffi_MyObject_unsafeRustMut(outer: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
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
                        qobject::cxx_qt_ffi_MyObject_unsafeRust(self)
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
                        qobject::cxx_qt_ffi_MyObject_unsafeRust(self)
                    }

                    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
                        qobject::cxx_qt_ffi_MyObject_unsafeRustMut(self)
                    }
                }
            },
        );
    }
}
