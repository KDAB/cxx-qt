// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{naming::qobject::QObjectNames, rust::fragment::GeneratedRustFragment},
    parser::inherit::ParsedInheritedMethod,
};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Item, Result};

pub fn generate(
    qobject_names: &QObjectNames,
    methods: &[&ParsedInheritedMethod],
) -> Result<GeneratedRustFragment> {
    let mut blocks = GeneratedRustFragment::default();
    let qobject_name = qobject_names.name.rust_unqualified();

    let mut bridges = methods
        .iter()
        .map(|method| {
            let parameters = method
                .parameters
                .iter()
                .map(|parameter| {
                    let ident = &parameter.ident;
                    let ty = &parameter.ty;
                    quote! { #ident: #ty }
                })
                .collect::<Vec<TokenStream>>();

            let span = method.method.span();

            let ident = &method.method_fields.name.rust_unqualified();
            let cxx_name_string = &method.wrapper_ident().to_string();
            let self_param = if method.mutable {
                quote_spanned! { span => self: Pin<&mut #qobject_name> }
            } else {
                quote_spanned! { span => self: &#qobject_name }
            };
            let return_type = &method.method.sig.output;

            // Needs to be unspanned or clippy breaks surrounding the safety comment
            let unsafe_call = if method.safe {
                None
            } else {
                Some(quote! { unsafe })
            };
            let doc_comments = &method.docs;
            let cfgs = &method.cfgs;
            let namespace = qobject_names.namespace_tokens();

            syn::parse2(quote_spanned! {
                span =>
                unsafe extern "C++" {
                    #[cxx_name = #cxx_name_string]
                    #namespace
                    #(#cfgs)*
                    #(#doc_comments)*
                    #unsafe_call fn #ident(#self_param, #(#parameters),*) #return_type;
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
    use crate::parser::CaseConversion;
    use crate::{generator::naming::qobject::tests::create_qobjectname, tests::assert_tokens_eq};
    use syn::{parse_quote, ForeignItemFn};

    fn generate_from_foreign(method: ForeignItemFn) -> Result<GeneratedRustFragment> {
        let method = ParsedInheritedMethod::parse(method, CaseConversion::none())?;
        let inherited_methods = vec![&method];
        generate(&create_qobjectname(), &inherited_methods)
    }

    #[test]
    fn test_mutable() {
        let generated = generate_from_foreign(parse_quote! {
            fn test(self: Pin<&mut MyObject>, a: B, b: C);
        })
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
        let generated = generate_from_foreign(parse_quote! {
            fn test(self: &MyObject, a: B, b: C);
        })
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
        let generated = generate_from_foreign(parse_quote! {
            unsafe fn test(self: &MyObject);
        })
        .unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 1);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 0);

        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            // TODO: Maybe remove the trailing comma after self?
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "testCxxQtInherit"]
                    unsafe fn test(self: &MyObject,);
                }
            },
        );
    }
}
