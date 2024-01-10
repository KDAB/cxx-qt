// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{generator::rust::fragment::GeneratedRustFragment, parser::qenum::ParsedQEnum};
use quote::quote;
use syn::{parse_quote_spanned, spanned::Spanned, Item};

pub fn generate_cxx_mod_contents(qenums: &[ParsedQEnum]) -> Vec<Item> {
    qenums
        .iter()
        .flat_map(|qenum| {
            let qenum_item = &qenum.item;
            let qenum_ident = &qenum.ident;
            let namespace = &qenum.namespace;
            let namespace = if namespace.is_empty() {
                quote! {}
            } else {
                quote! { #[namespace = #namespace ] }
            };
            vec![
                parse_quote_spanned! {
                    qenum.item.span() =>
                    #[repr(i32)]
                    #qenum_item
                },
                parse_quote_spanned! {
                    qenum.item.span() =>
                    extern "C++" {
                        #namespace
                        type #qenum_ident;
                    }
                },
            ]
            .into_iter()
        })
        .collect()
}

pub fn generate(qenums: &[ParsedQEnum]) -> GeneratedRustFragment {
    GeneratedRustFragment {
        cxx_mod_contents: generate_cxx_mod_contents(qenums),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::assert_tokens_eq;
    use quote::quote;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn generates() {
        let qenums = vec![ParsedQEnum::parse(parse_quote! {
            /// Doc comment
            enum MyEnum {
                /// Document Variant1
                Variant1,
                /// Document Variant2
                Variant2,
            }
        })
        .unwrap()];

        let generated = generate(&qenums);
        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                #[repr(i32)]
                #[doc = r" Doc comment"]
                enum MyEnum {
                    #[doc = r" Document Variant1"]
                    Variant1,
                    #[doc = r" Document Variant2"]
                    Variant2,
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                extern "C++" {
                    type MyEnum;
                }
            },
        )
    }
}
