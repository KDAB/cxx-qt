// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{spanned::Spanned, Error, Field, Fields, FieldsNamed, Ident, Result, Type};

/// In a group of [syn::Fields] extract the [syn::Ident] and [syn::Type] from
/// any [syn::FieldsNamed] fields
///
/// If there are [syn::FieldsUnnamed] then an error occurs
pub fn fields_named_to_ident_type(fields: &Fields) -> Result<Vec<(Ident, Type)>> {
    match fields {
        Fields::Named(FieldsNamed { named, .. }) => {
            let mut fields = vec![];
            for name in named {
                if let Field {
                    ident: Some(ident),
                    ty,
                    ..
                } = name
                {
                    fields.push((ident.clone(), ty.clone()));
                }
            }

            Ok(fields)
        }
        Fields::Unnamed(_) => Err(Error::new(fields.span(), "Fields cannot be unnamed")),
        // Unit is an empty struct or enum etc
        Fields::Unit => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::{ItemStruct, Type, Variant};

    /// Helper which returns a f64 as a [syn::Type]
    fn f64_type() -> Type {
        tokens_to_syn(quote! { f64 })
    }

    #[test]
    fn test_fields_named_to_ident_type_enum_variant_named() {
        let v: Variant = tokens_to_syn(quote! {
            PointChanged { x: f64, y: f64 }
        });
        let result = fields_named_to_ident_type(&v.fields).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "x");
        assert_eq!(result[0].1, f64_type());
        assert_eq!(result[1].0, "y");
        assert_eq!(result[1].1, f64_type());
    }

    #[test]
    fn test_fields_named_to_ident_type_enum_variant_unamed() {
        let v: Variant = tokens_to_syn(quote! {
            PointChanged(f64, f64)
        });
        let result = fields_named_to_ident_type(&v.fields);
        assert!(result.is_err());
    }

    #[test]
    fn test_fields_named_to_ident_type_enum_variant_empty() {
        let v: Variant = tokens_to_syn(quote! {
            PointChanged
        });
        let result = fields_named_to_ident_type(&v.fields).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_fields_named_to_ident_type_struct_named() {
        let s: ItemStruct = tokens_to_syn(quote! {
            struct Point {
                x: f64,
                y: f64
            }
        });
        let result = fields_named_to_ident_type(&s.fields).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "x");
        assert_eq!(result[0].1, f64_type());
        assert_eq!(result[1].0, "y");
        assert_eq!(result[1].1, f64_type());
    }

    #[test]
    fn test_fields_named_to_ident_type_struct_unamed() {
        let s: ItemStruct = tokens_to_syn(quote! {
            struct Point(f64, f64);
        });
        let result = fields_named_to_ident_type(&s.fields);
        assert!(result.is_err());
    }

    #[test]
    fn test_fields_named_to_ident_type_struct_empty() {
        let s: ItemStruct = tokens_to_syn(quote! {
            struct Point;
        });
        let result = fields_named_to_ident_type(&s.fields).unwrap();
        assert_eq!(result.len(), 0);
    }
}
