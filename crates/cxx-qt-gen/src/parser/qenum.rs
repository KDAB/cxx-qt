// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{naming::Name, parser::require_attributes, syntax::path::path_compare_str};
use quote::ToTokens;
use syn::{Ident, ItemEnum, Result, Variant};

pub struct ParsedQEnum {
    /// The name of the QObject
    pub name: Name,
    /// the values of the QEnum
    pub variants: Vec<Ident>,
    /// The QObject to which this QEnum belongs.
    pub qobject: Option<Ident>,
    /// The original enum item
    pub item: ItemEnum,
}

impl ParsedQEnum {
    const ALLOWED_ATTRS: [&'static str; 5] = ["doc", "cxx_name", "rust_name", "namespace", "qenum"];
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

    pub fn parse(
        qenum: ItemEnum,
        qobject: Option<Ident>,
        parent_namespace: Option<&str>,
        module: &Ident,
    ) -> Result<Self> {
        require_attributes(&qenum.attrs, &Self::ALLOWED_ATTRS)?;
        if qenum.variants.is_empty() {
            return Err(syn::Error::new_spanned(
                qenum,
                "QEnum must have at least one variant!",
            ));
        }

        let name =
            Name::from_ident_and_attrs(&qenum.ident, &qenum.attrs, parent_namespace, Some(module))?;

        if name.namespace().is_none() && qobject.is_none() {
            return Err(syn::Error::new_spanned(
                qenum.ident,
                "A QEnum must either be namespaced or associated to a QObject!",
            ));
        }

        let variants = qenum
            .variants
            .iter()
            .map(Self::parse_variant)
            .collect::<Result<_>>()?;

        Ok(Self {
            name,
            qobject,
            variants,
            item: qenum,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::assert_tokens_eq;

    use super::*;
    use quote::format_ident;
    use syn::parse_quote;

    fn variants_to_strings(qenum: &ParsedQEnum) -> Vec<String> {
        qenum
            .variants
            .iter()
            .map(Ident::to_string)
            .collect::<Vec<_>>()
    }

    fn mock_module() -> Ident {
        format_ident!("qobject")
    }

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
        let qobject = Some(format_ident!("MyObject"));

        let parsed =
            ParsedQEnum::parse(qenum.clone(), qobject.clone(), None, &mock_module()).unwrap();
        assert_eq!(parsed.name.rust_unqualified(), "MyEnum");
        assert_eq!(parsed.name.namespace(), None);
        assert_eq!(parsed.qobject, qobject);

        assert_eq!(*variants_to_strings(&parsed), ["Variant1", "Variant2"],);
        assert_tokens_eq(&parsed.item, qenum.to_token_stream());
    }

    #[test]
    fn parse_namespaced() {
        let qenum: ItemEnum = parse_quote! {
            #[namespace="my_namespace"]
            enum MyEnum {
                A,
                B,
            }
        };

        let parsed = ParsedQEnum::parse(qenum.clone(), None, None, &mock_module()).unwrap();
        assert_eq!(parsed.name.rust_unqualified(), "MyEnum");
        assert_eq!(parsed.name.namespace().unwrap(), "my_namespace");
        assert!(parsed.qobject.is_none());

        assert_eq!(*variants_to_strings(&parsed), ["A", "B"],);
        assert_tokens_eq(&parsed.item, qenum.to_token_stream());
    }

    use crate::tests::assert_parse_errors;

    #[test]
    fn parse_errors() {
        assert_parse_errors! {
            |qenum| ParsedQEnum::parse(qenum, Some(format_ident!("QObject")), None, &mock_module()) =>
            // No variants
            { enum MyEnum {} }
            {
                // Unknown attributes
                #[any_attribute]
                enum MyEnum { A }
            }
            {
                // Repr is not allowed either
                #[repr(u32)]
                enum MyEnum { A }
            }
            {
                // Fields are not allowed
                enum MyEnum {
                    A { field: i32 }
                }
            }
            {
                // Fields are not allowed
                enum MyEnum {
                    A(i32)
                }
            }
            {
                // Attributes on variants are not allowed
                enum MyEnum {
                    #[any_attribute]
                    A
                }
            }
            {
                // TODO: Allow discriminants
                enum MyEnum {
                    A = 1
                }
            }
        }
    }

    #[test]
    fn parse_missing_namespace() {
        let qenum: ItemEnum = parse_quote! {
            enum MyEnum {
                A,
            }
        };
        assert!(ParsedQEnum::parse(qenum, None, None, &mock_module()).is_err());
    }

    #[test]
    fn parse_qobject_and_qenum_namespace_are_independent() {
        let qenum: ItemEnum = parse_quote! {
            enum MyEnum {
                A,
            }
        };
        let parent_namespace = Some("my_namespace");
        let qobject = Some(format_ident!("MyObject"));
        let parsed =
            ParsedQEnum::parse(qenum, qobject.clone(), parent_namespace, &mock_module()).unwrap();
        assert_eq!(parsed.name.namespace(), Some("my_namespace"));
        assert_eq!(parsed.qobject, qobject);
    }
}
