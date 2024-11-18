// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::qenum::ParsedQEnum;
use quote::quote;
use syn::{parse_quote_spanned, spanned::Spanned, Item};

pub fn generate_cxx_mod_contents(qenums: &[ParsedQEnum]) -> Vec<Item> {
    qenums
        .iter()
        .flat_map(|qenum| {
            let qenum_ident = &qenum.name.rust_unqualified();
            let namespace = &qenum.name.namespace();
            let item = &qenum.item;
            let vis = &item.vis;
            let variants = &item.variants;
            let docs = &qenum.docs;
            let cfgs = &qenum.cfgs;

            let cxx_namespace = if namespace.is_none() {
                quote! {}
            } else {
                quote! { #[namespace = #namespace ] }
            };
            vec![
                parse_quote_spanned! {
                    item.span() =>
                    #[repr(i32)]
                    #(#cfgs)*
                    #(#docs)*
                    #cxx_namespace
                    #vis enum #qenum_ident {
                        #variants
                    }
                },
                parse_quote_spanned! {
                    item.span() =>
                    extern "C++" {
                        #(#cfgs)*
                        #cxx_namespace
                        type #qenum_ident;
                    }
                },
            ]
            .into_iter()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{generator::rust::fragment::GeneratedRustFragment, tests::assert_tokens_eq};
    use quote::{format_ident, quote};
    use syn::parse_quote;

    use super::*;

    fn generate(qenums: &[ParsedQEnum]) -> GeneratedRustFragment {
        GeneratedRustFragment {
            cxx_mod_contents: generate_cxx_mod_contents(qenums),
            ..Default::default()
        }
    }

    #[test]
    fn generates() {
        let qenums = vec![ParsedQEnum::parse(
            parse_quote! {
                /// Doc comment
                enum MyEnum {
                    /// Document Variant1
                    Variant1,
                    /// Document Variant2
                    Variant2,
                }
            },
            // The Ident of the QObject shouldn't matter
            Some(format_ident!("MyObject")),
            None,
            &format_ident!("qobject"),
        )
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
