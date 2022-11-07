// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{spanned::Spanned, Error, Field, Fields, FieldsNamed, Result};

/// In a group of [syn::Fields] extract any [syn::FieldNamed] fields and allow for mutation
///
/// If there are [syn::FieldsUnnamed] then an error occurs
pub fn fields_to_named_fields_mut(fields: &mut Fields) -> Result<Vec<&mut Field>> {
    match fields {
        Fields::Named(FieldsNamed { named, .. }) => Ok(named.iter_mut().collect()),
        Fields::Unnamed(_) => Err(Error::new(fields.span(), "Fields cannot be unnamed")),
        // Unit is an empty struct or enum etc
        Fields::Unit => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::utils::tokens_to_syn;
    use quote::quote;
    use syn::{ItemStruct, Type, Variant};

    /// Helper which returns a f64 as a [syn::Type]
    fn f64_type() -> Type {
        tokens_to_syn(quote! { f64 })
    }

    #[test]
    fn test_fields_to_named_fields_enum_variant_named() {
        let mut v: Variant = tokens_to_syn(quote! {
            PointChanged { x: f64, y: f64 }
        });
        let result = fields_to_named_fields_mut(&mut v.fields).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].ident.as_ref().unwrap(), "x");
        assert_eq!(result[0].ty, f64_type());
        assert_eq!(result[1].ident.as_ref().unwrap(), "y");
        assert_eq!(result[1].ty, f64_type());
    }

    #[test]
    fn test_fields_to_named_fields_enum_variant_unamed() {
        let mut v: Variant = tokens_to_syn(quote! {
            PointChanged(f64, f64)
        });
        let result = fields_to_named_fields_mut(&mut v.fields);
        assert!(result.is_err());
    }

    #[test]
    fn test_fields_to_named_fields_enum_variant_empty() {
        let mut v: Variant = tokens_to_syn(quote! {
            PointChanged
        });
        let result = fields_to_named_fields_mut(&mut v.fields).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_fields_to_named_fields_struct_named() {
        let mut s: ItemStruct = tokens_to_syn(quote! {
            struct Point {
                x: f64,
                y: f64
            }
        });
        let result = fields_to_named_fields_mut(&mut s.fields).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].ident.as_ref().unwrap(), "x");
        assert_eq!(result[0].ty, f64_type());
        assert_eq!(result[1].ident.as_ref().unwrap(), "y");
        assert_eq!(result[1].ty, f64_type());
    }

    #[test]
    fn test_fields_to_named_fields_struct_unamed() {
        let mut s: ItemStruct = tokens_to_syn(quote! {
            struct Point(f64, f64);
        });
        let result = fields_to_named_fields_mut(&mut s.fields);
        assert!(result.is_err());
    }

    #[test]
    fn test_fields_to_named_fields_struct_empty() {
        let mut s: ItemStruct = tokens_to_syn(quote! {
            struct Point;
        });
        let result = fields_to_named_fields_mut(&mut s.fields).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_fields_to_named_fields_mutatable() {
        let mut s: ItemStruct = tokens_to_syn(quote! {
            struct Point {
                #[attribute]
                x: f64,
                y: f64
            }
        });
        let mut result = fields_to_named_fields_mut(&mut s.fields).unwrap();
        assert_eq!(result.len(), 2);
        result[0].attrs.clear();

        let expected: ItemStruct = tokens_to_syn(quote! {
            struct Point {
                x: f64,
                y: f64
            }
        });
        assert_eq!(s, expected);
    }
}
