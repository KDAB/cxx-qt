// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use quote::ToTokens;
use syn::{Ident, ItemEnum, Result, Variant};

use crate::syntax::path::path_compare_str;

pub struct ParsedQEnum {
    /// The ident of the QEnum
    pub ident: Ident,
    /// the values of the QEnum
    pub variants: Vec<Ident>,
    /// The original enum item
    pub item: ItemEnum,
}

impl ParsedQEnum {
    fn parse_variant(variant: &Variant) -> Result<Ident> {
        fn err(spanned: &impl ToTokens, message: &str) -> Result<Ident> {
            Err(syn::Error::new_spanned(spanned, message))
        }

        if !variant.fields.is_empty() {
            return err(variant, "QEnum variants cannot have fields");
        }
        if let Some(attr) = variant
            .attrs
            .iter()
            .find(|attr| !path_compare_str(attr.path(), &["doc"]))
        {
            return err(
                attr,
                "QEnum variants can only have #[doc=\"...\"] attributes",
            );
        }
        if let Some(discriminant) = variant.discriminant.as_ref() {
            return err(
                &discriminant.1,
                "QEnum variants with explicit values are not supported (yet)",
            );
        }

        Ok(variant.ident.clone())
    }

    pub fn parse(qenum: ItemEnum) -> Result<Self> {
        if qenum.variants.is_empty() {
            return Err(syn::Error::new_spanned(
                qenum,
                "QEnum must have at least one variant",
            ));
        }

        // TODO: Add support for `namespace`, `cxx_name` and `rust_name` attributes.
        if let Some(attr) = qenum
            .attrs
            .iter()
            .find(|attr| !path_compare_str(attr.path(), &["doc"]))
        {
            return Err(syn::Error::new_spanned(
                attr,
                "Additional attributes are not allowed on #[qenum] enums",
            ));
        }

        let variants = qenum
            .variants
            .iter()
            .map(Self::parse_variant)
            .collect::<Result<_>>()?;

        Ok(Self {
            ident: qenum.ident.clone(),
            variants,
            item: qenum,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn parse() {
        let qenum: ItemEnum = parse_quote! {
            /// My doc comment
            enum MyEnum {
                /// Variant1 doc comment
                Variant1,
                /// Variant2 doc comment
                Variant2,
            }
        };

        let parsed = ParsedQEnum::parse(qenum).unwrap();
        assert_eq!(parsed.ident, "MyEnum");

        assert_eq!(
            *parsed
                .variants
                .iter()
                .map(Ident::to_string)
                .collect::<Vec<_>>(),
            ["Variant1", "Variant2"],
        )
    }

    macro_rules! assert_parse_error {
        ($( $input:tt )*) => {
            let qenum: ItemEnum = parse_quote! { $($input)* };
            assert!(ParsedQEnum::parse(qenum).is_err());
        }
    }

    #[test]
    fn parse_errors() {
        assert_parse_error! {
            // No variants
            enum MyEnum {}
        }
        assert_parse_error! {
            // Unkown attributes
            #[any_attribute]
            enum MyEnum { A }
        }
        assert_parse_error! {
            // Repr is not allowed either
            #[repr(u32)]
            enum MyEnum { A }
        }
        assert_parse_error! {
            // Fields are not allowed
            enum MyEnum {
                A { field: i32 }
            }
        }
        assert_parse_error! {
            // Fields are not allowed
            enum MyEnum {
                A(i32)
            }
        }
        assert_parse_error! {
            // Attributes on variants are not allowed
            enum MyEnum {
                #[any_attribute]
                A
            }
        }

        // TODO: allow discriminants
        assert_parse_error! {
            enum MyEnum {
                A = 1
            }
        }
    }
}
