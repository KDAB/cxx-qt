// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream, Parser},
    parse_quote,
    spanned::Spanned,
    Attribute, Error, FnArg, ForeignItem, ForeignItemType, Ident, ItemForeignMod, Path, Receiver,
    Result, Signature, Token, Visibility,
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

            // Read until the end of the cursor
            input.step(|cursor| {
                let mut rest = *cursor;
                while let Some((_, next)) = rest.token_tree() {
                    rest = next;
                }
                Ok(((), rest))
            })?;

            Ok(Some(parse_quote! {
                #(#attrs)*
                #visibility #type_token #ident;
            }))
        } else {
            // Error as we have parsed the attributes and visiblity but have an unknown stream
            //
            // To return None here we should instead peek
            Err(Error::new(
                tokens.span(),
                "Unsupported verbatim input in ForeignItem!",
            ))
        }
    }
    .parse2(tokens.clone())
}

/// Representation of a specific type alias for CXX-Qt where we map between two idents with a single super.
///
/// `type A = super::B`
#[derive(Clone)]
pub struct ForeignTypeIdentAlias {
    /// Attributes on the alias
    pub attrs: Vec<Attribute>,
    /// The left side of the alias
    pub ident_left: Ident,
    /// The right side of the alias
    pub ident_right: Ident,
}

impl Parse for ForeignTypeIdentAlias {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        // Note that visibility is ignored for now
        let _: Visibility = input.parse()?;

        if input.peek(Token![type]) {
            let _type_token: Token![type] = input.parse()?;
            let ident_left: Ident = input.parse()?;
            let _equals_token: Token![=] = input.parse()?;
            let path: Path = input.parse()?;
            let _semi_colon: Token![;] = input.parse()?;

            // Convert the path (super::T) to an ident (T)
            let ident_right = {
                // We only support super::T for now due to CXX only supporting type T with no alias
                // as an extern "Rust" type.
                //
                // TODO: once CXX does have support for type aliases in extern "Rust" blocks
                // we can instead pass through the full path of an ident_right.
                // https://github.com/dtolnay/cxx/issues/1187
                // https://github.com/dtolnay/cxx/pull/1181
                //
                // Note that we would need to still use the last segment as the Rust name internally
                if path.segments.len() != 2 {
                    return Err(Error::new(
                        path.span(),
                        "Type alias path must have at exactly two segments, super::T!",
                    ));
                }

                if path.segments[0].ident != "super" {
                    return Err(Error::new(
                        path.span(),
                        "Type alias path must have super as the first segment, super::T!",
                    ));
                }

                path.segments[1].ident.clone()
            };

            if ident_left == ident_right {
                return Err(Error::new(
                    path.span(),
                    "Type alias path must have differing idents, type A = super::B. A and B cannot be the same!",
                ));
            }

            Ok(Self {
                attrs,
                ident_left,
                ident_right,
            })
        } else {
            // Error as we have parsed the attributes and visiblity but have an unknown stream
            //
            // To return None here we should instead peek
            Err(Error::new(
                input.span(),
                "Unsupported verbatim input in ForeignItem!",
            ))
        }
    }
}

pub fn self_type_from_foreign_fn(signature: &Signature) -> Result<Receiver> {
    if let Some(FnArg::Receiver(receiver)) = signature.inputs.iter().next() {
        if !receiver.attrs.is_empty() {
            return Err(Error::new(
                receiver.span(),
                "Attributes on the `self:` receiver are not supported!",
            ));
        }

        if receiver.mutability.is_some() {
            return Err(Error::new(
                receiver.span(),
                "mut on self (i.e. `&mut self`) are not supported! Use `self: Pin<&mut T>` instead",
            ));
        }

        if receiver.reference.is_some() {
            return Err(Error::new(
                receiver.span(),
                "Reference on self (i.e. `&self`) are not supported! Use `self: &T` instead",
            ));
        }

        if receiver.colon_token.is_none() {
            return Err(Error::new(
                receiver.span(),
                "`self` is not supported as receiver! Use `self: T` to indicate a type.",
            ));
        }

        Ok(receiver.clone())
    } else {
        Err(Error::new_spanned(
            signature,
            "Expected first argument to be a `self:` receiver!",
        ))
    }
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, ForeignItemFn};

    use super::*;

    use quote::{quote, ToTokens};

    #[test]
    fn test_foreign_mod_to_foreign_item_types() {
        let item: ItemForeignMod = parse_quote! {
            extern "C++" {
                #[namespace = "a"]
                type A;

                #[cxx_name = "D"]
                type B = C;
            }
        };
        let result = foreign_mod_to_foreign_item_types(&item).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].attrs.len(), 1);
        assert_eq!(result[0].ident, "A");

        assert_eq!(result[1].attrs.len(), 1);
        assert_eq!(result[1].ident, "B");
    }

    #[test]
    fn test_foreign_mod_to_foreign_item_types_invalid() {
        let item: ItemForeignMod = parse_quote! {
            extern "RustQt" {
                fn my_function() {}
            }
        };
        let result = foreign_mod_to_foreign_item_types(&item);
        assert!(result.is_err())
    }

    #[test]
    fn test_foreign_fn_self() {
        let foreign_fn: ForeignItemFn = parse_quote! {
            fn foo(self: &T, a: A) -> B;
        };
        let result = self_type_from_foreign_fn(&foreign_fn.sig).unwrap();
        assert_eq!(result.ty.to_token_stream().to_string(), "& T");
    }

    #[test]
    fn test_foreign_fn_invalid_self() {
        macro_rules! test {
            ($($tt:tt)*) => {
                let foreign_fn: ForeignItemFn = parse_quote! {
                    $($tt)*
                };
                assert!(self_type_from_foreign_fn(&foreign_fn.sig).is_err());
            }
        }
        // Missing self
        test! { fn foo(a: A) -> B; }
        // self without type
        test! { fn foo(self); }
        // self with mut
        test! { fn foo(mut self: T); }
        // self reference
        test! { fn foo(&self); }
        // self reference with mut
        test! { fn foo(&mut self); }
        // attribute on self type
        test! { fn foo(#[attr] self: T); }
    }

    #[test]
    fn test_foreign_type_ident_alias_invalid() {
        let alias = syn::parse2::<ForeignTypeIdentAlias>(quote! {
            struct MyStruct;
        });
        assert!(alias.is_err()); // Unsupported verbatim input from trying to parse a struct as a foreign Type alias
    }

    #[test]
    fn test_foreign_type_ident_alias() {
        let alias: ForeignTypeIdentAlias = parse_quote! {
            #[attr]
            type A = super::B;
        };

        assert_eq!(alias.attrs.len(), 1);
        assert_eq!(alias.ident_left, "A");
        assert_eq!(alias.ident_right, "B");
    }

    #[test]
    fn test_foreign_type_ident_alias_segments_one() {
        let parse = syn::parse2::<ForeignTypeIdentAlias>(quote! {
            type A = B;
        });
        assert!(parse.is_err());
    }

    #[test]
    fn test_foreign_type_ident_alias_segments_three() {
        let parse = syn::parse2::<ForeignTypeIdentAlias>(quote! {
            type A = super::module::B;
        });
        assert!(parse.is_err());
    }

    #[test]
    fn test_foreign_type_ident_alias_no_super() {
        let parse = syn::parse2::<ForeignTypeIdentAlias>(quote! {
            type A = crate::B;
        });
        assert!(parse.is_err());
    }

    #[test]
    fn test_foreign_type_ident_alias_left_is_right() {
        let parse = syn::parse2::<ForeignTypeIdentAlias>(quote! {
            type A = super::A;
        });
        assert!(parse.is_err());
    }

    #[test]
    fn test_foreign_type_ident_visibility() {
        // Ensure that visibility does not error, later it might be stored
        let alias: ForeignTypeIdentAlias = parse_quote! {
            #[attr]
            pub type A = super::B;
        };
        assert_eq!(alias.attrs.len(), 1);
        assert_eq!(alias.ident_left, "A");
        assert_eq!(alias.ident_right, "B");
    }
}
