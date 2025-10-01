// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::qenum::ParsedQEnum;
use quote::quote;
use syn::{parse_quote_spanned, spanned::Spanned};

use super::fragment::GeneratedRustFragment;

pub fn generate(qenums: &[ParsedQEnum]) -> Vec<GeneratedRustFragment> {
    qenums
        .iter()
        .map(|qenum| {
            if qenum.name.namespace().is_some() {
                generate_namespaced_qenum(qenum)
            } else {
                generate_member_qenum(qenum)
            }
        })
        .collect()
}

fn generate_member_qenum(qenum: &ParsedQEnum) -> GeneratedRustFragment {
    // TODO
    generate_namespaced_qenum(qenum)
}

fn generate_namespaced_qenum(qenum: &ParsedQEnum) -> GeneratedRustFragment {
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
    GeneratedRustFragment {
        cxx_mod_contents: vec![
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
        ],
        cxx_qt_mod_contents: vec![],
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::assert_tokens_eq;
    use quote::{format_ident, quote};
    use syn::parse_quote;

    use super::*;

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
        assert_eq!(generated.len(), 1);
        let generated = &generated[0];
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
