// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse::{ParseStream, Parser},
    Attribute, ForeignItem, ForeignItemType, Ident, ItemForeignMod, Result, Token, Visibility,
};

/// For a given [syn::ForeignItem] return the [syn::ForeignItemType] if there is one
///
/// And ignore any extra syntax after the = in type A = ...
fn foreign_item_to_type(foreign_item: &ForeignItem) -> Result<Option<ForeignItemType>> {
    match foreign_item {
        // type A;
        ForeignItem::Type(foreign_type) => Ok(Some(foreign_type.clone())),
        // Could be Verbatim when there is a = Y after the type, could be a normal type otherwise
        ForeignItem::Verbatim(tokens) => verbatim_to_foreign_type(tokens),
        _others => Ok(None),
    }
}

/// For a given [syn::ItemForeignMod] return a vector of the [syn::ForeignItemType] if there are any
///
/// And ignore any extra syntax after the = in type A = ...
pub(crate) fn foreign_mod_to_foreign_item_types(
    foreign_mod: &ItemForeignMod,
) -> Result<Vec<ForeignItemType>> {
    foreign_mod
        .items
        .iter()
        .filter_map(|item| match foreign_item_to_type(item) {
            Ok(Some(value)) => Some(Ok(value)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<Vec<ForeignItemType>>>()
}

/// For a given verbatim [proc_macro2::TokenStream] return a [syn::ItemForegnMod] if there is one
///
/// And ignore any unsafe token before the extern block
pub(crate) fn verbatim_to_foreign_mod(tokens: &TokenStream) -> Result<Option<ItemForeignMod>> {
    |input: ParseStream| -> Result<Option<ItemForeignMod>> {
        // Parse any namespace attributes on the outside of the unsafe extern block
        let mut attrs = input.call(Attribute::parse_outer)?;

        // If we are an unsafe then extern block try to parse it
        if input.peek(Token![unsafe]) && input.peek2(Token![extern]) {
            input.parse::<Token![unsafe]>()?;
            let mut foreign_mod = input.parse::<ItemForeignMod>()?;
            // Inject the attributes from the outside of the unsafe block into the foreign mod
            attrs.append(&mut foreign_mod.attrs);
            foreign_mod.attrs = attrs;
            Ok(Some(foreign_mod))
        } else {
            // Move the cursor past all remaining tokens, otherwise parse2 fails
            input.step(|cursor| {
                let mut rest = *cursor;
                while let Some((_, next)) = rest.token_tree() {
                    rest = next;
                }
                Ok(((), rest))
            })?;

            Ok(None)
        }
    }
    .parse2(tokens.clone())
}

/// For a given verbatim [proc_macro2::TokenStream] return the [syn::ForeignItemType] if there is one
///
/// And ignore any extra syntax after the = in type A = ...
fn verbatim_to_foreign_type(tokens: &TokenStream) -> Result<Option<ForeignItemType>> {
    |input: ParseStream| -> Result<Option<ForeignItemType>> {
        let attrs = input.call(Attribute::parse_outer)?;
        let visibility: Visibility = input.parse()?;
        if input.peek(Token![type]) {
            let type_token: Token![type] = input.parse()?;
            let ident: Ident = input.parse()?;

            // Read until the next semi colon
            input.step(|cursor| {
                let mut rest = *cursor;
                while let Some((tt, next)) = rest.token_tree() {
                    match &tt {
                        TokenTree::Punct(punct) if punct.as_char() == ';' => {
                            return Ok(((), next));
                        }
                        _ => rest = next,
                    }
                }
                Err(cursor.error("no `;` was found after this point"))
            })?;

            Ok(Some(syn::parse2(
                quote! {
                    #(#attrs)*
                    #visibility #type_token #ident;
                }
                .into_token_stream(),
            )?))
        } else {
            Ok(None)
        }
    }
    .parse2(tokens.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;

    #[test]
    fn test_foreign_mod_to_foreign_item_types() {
        let item: ItemForeignMod = tokens_to_syn(quote! {
            extern "C++" {
                #[namespace = "a"]
                type A;

                #[cxx_name = "D"]
                type B = C;
            }
        });
        let result = foreign_mod_to_foreign_item_types(&item).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].attrs.len(), 1);
        assert_eq!(result[0].ident, "A");

        assert_eq!(result[1].attrs.len(), 1);
        assert_eq!(result[1].ident, "B");
    }

    #[test]
    fn test_verbatim_to_foreign_mod() {
        let tokens = quote! {
            #[namespace = "a"]
            unsafe extern "C++" {
                type A;
            }
        };
        let result = verbatim_to_foreign_mod(&tokens).unwrap();
        let result = result.unwrap();
        assert_eq!(result.attrs.len(), 1);
        assert_eq!(result.items.len(), 1);
    }
}
