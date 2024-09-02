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
    qobject_ident: &QObjectNames,
    methods: &[&ParsedInheritedMethod],
) -> Result<GeneratedRustFragment> {
    let mut blocks = GeneratedRustFragment::default();
    let qobject_name = qobject_ident.name.rust_unqualified();

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

            let ident = &method.method.sig.ident;
            let cxx_name_string = &method.wrapper_ident().to_string();
            let self_param = if method.mutable {
                quote! { self: Pin<&mut #qobject_name> }
            } else {
                quote! { self: &#qobject_name }
            };
            let return_type = &method.method.sig.output;

            let mut unsafe_block = None;
            let mut unsafe_call = Some(quote! { unsafe });
            if method.safe {
                std::mem::swap(&mut unsafe_call, &mut unsafe_block);
            }

            let attrs = &method.method.attrs;
            let doc_comments = &method.docs;

            syn::parse2(quote_spanned! {
                method.method.span() =>
                #unsafe_block extern "C++" {
                    #(#attrs)*
                    #[cxx_name = #cxx_name_string]
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
    use crate::{
        generator::naming::qobject::tests::create_qobjectname, syntax::safety::Safety,
        tests::assert_tokens_eq,
    };
    use syn::{parse_quote, ForeignItemFn};

    fn generate_from_foreign(
        method: ForeignItemFn,
        safety: Safety,
    ) -> Result<GeneratedRustFragment> {
        let method = ParsedInheritedMethod::parse(method, safety).unwrap();
        let inherited_methods = vec![&method];
        generate(&create_qobjectname(), &inherited_methods)
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
            // TODO: Maybe remove the trailing comma after self?
            quote! {
                extern "C++" {
                    #[cxx_name = "testCxxQtInherit"]
                    unsafe fn test(self: &MyObject,);
                }
            },
        );
    }
}
