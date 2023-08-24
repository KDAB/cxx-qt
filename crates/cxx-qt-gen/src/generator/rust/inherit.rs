// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::rust::qobject::GeneratedRustQObject, parser::inherit::ParsedInheritedMethod,
    syntax::attribute::attribute_take_path,
};
use quote::quote;
use syn::{Item, Result};

pub fn generate(methods: &[ParsedInheritedMethod]) -> Result<GeneratedRustQObject> {
    let mut blocks = GeneratedRustQObject::default();

    let mut bridges = methods
        .iter()
        .map(|inherit_method| {
            let wrapper_ident_cpp_str = &inherit_method.wrapper_ident().to_string();

            // Remove any cxx_name attribute on the original method
            // As we need it to use the wrapper ident
            let original_method = {
                let mut original_method = inherit_method.method.clone();
                attribute_take_path(&mut original_method.attrs, &["cxx_name"]);
                original_method
            };

            let mut unsafe_block = None;
            let mut unsafe_call = Some(quote! { unsafe });
            if inherit_method.safe {
                std::mem::swap(&mut unsafe_call, &mut unsafe_block);
            }

            syn::parse2(quote! {
                #unsafe_block extern "C++" {
                    #[cxx_name = #wrapper_ident_cpp_str]
                    #original_method
                }
            })
        })
        .collect::<Result<Vec<Item>>>()?;

    blocks.cxx_mod_contents.append(&mut bridges);
    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{syntax::safety::Safety, tests::assert_tokens_eq};
    use syn::{parse_quote, ForeignItemFn};

    fn generate_from_foreign(
        method: ForeignItemFn,
        safety: Safety,
    ) -> Result<GeneratedRustQObject> {
        let inherited_methods = vec![ParsedInheritedMethod::parse(method, safety).unwrap()];
        generate(&inherited_methods)
    }

    #[test]
    fn test_mutable() {
        let generated = generate_from_foreign(
            parse_quote! {
                fn test(self: Pin<&mut MyObject>, a: B, b: C);
            },
            Safety::Safe,
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 1);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 0);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "testCxxQtInherit"]
                    fn test(self: Pin<&mut MyObject>, a: B, b: C);
                }
            },
        );
    }

    #[test]
    fn test_immutable() {
        let generated = generate_from_foreign(
            parse_quote! {
                fn test(self: &MyObject, a: B, b: C);
            },
            Safety::Safe,
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 1);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 0);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "testCxxQtInherit"]
                    fn test(self: &MyObject, a: B, b: C);
                }
            },
        );
    }

    #[test]
    fn test_unsafe() {
        let generated = generate_from_foreign(
            parse_quote! {
                unsafe fn test(self: &MyObject);
            },
            Safety::Unsafe,
        )
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 1);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 0);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                extern "C++" {
                    #[cxx_name = "testCxxQtInherit"]
                    unsafe fn test(self: &MyObject);
                }
            },
        );
    }
}
