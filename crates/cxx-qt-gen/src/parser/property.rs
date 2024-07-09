// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashSet;

use syn::{parse::ParseStream, punctuated::Punctuated, Attribute, Ident, Result, Token, Type};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum QPropertyFlag {
    Read,
    Write,
    Notify,
}

/// Describes a single Q_PROPERTY for a struct
pub struct ParsedQProperty {
    /// The [syn::Ident] of the property
    pub ident: Ident,
    /// The [syn::Type] of the property
    pub ty: Type,
    /// HashSet of [QPropertyFlag]s which were specified
    pub flags: HashSet<QPropertyFlag>,
}

impl ParsedQProperty {
    pub fn parse(attr: Attribute) -> Result<Self> {
        attr.parse_args_with(|input: ParseStream| -> Result<Self> {
            let ty = input.parse()?;
            let _comma = input.parse::<Token![,]>()?;
            let ident = input.parse()?;

            if input.is_empty() {
                // No flags so return with empty HashSet
                return Ok(Self {
                    ident,
                    ty,
                    flags: Default::default(),
                });
            }

            let _comma = input.parse::<Token![,]>()?; // Start of final identifiers

            // TODO: Allow parser to store pairs of items e.g read = get_value, Might be useful to use Meta::NameValue
            let punctuated_flags: Punctuated<Ident, Token![,]> =
                Punctuated::parse_terminated(input)?;

            let flags: Vec<Ident> = punctuated_flags.into_iter().collect(); // Removes the commas while collecting into Vec

            let mut flags_set: HashSet<QPropertyFlag> = HashSet::new();

            for identifier in flags {
                match identifier.to_string().as_str() {
                    "read" => flags_set.insert(QPropertyFlag::Read),
                    "write" => flags_set.insert(QPropertyFlag::Write),
                    "notify" => flags_set.insert(QPropertyFlag::Notify),
                    _ => panic!("Invalid Token"), // TODO: might not be a good idea to error here
                };
            }

            // TODO: later we'll need to parse setters and getters here
            // which are key-value, hence this not being parsed as a list

            Ok(Self {
                ident,
                ty,
                flags: flags_set,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::format_ident;
    use syn::{parse_quote, ItemStruct};

    #[test]
    fn test_parse_property() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });
    }

    #[test]
    fn test_parse_read_flag() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });
        assert!(property.flags.contains(&QPropertyFlag::Read));
    }

    #[test]
    fn test_parse_all_flags() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read, write, notify)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });
        assert!(property.flags.contains(&QPropertyFlag::Read));
        assert!(property.flags.contains(&QPropertyFlag::Write));
        assert!(property.flags.contains(&QPropertyFlag::Notify));
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_flags() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read, write, A)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
    }

    #[test]
    fn test_parse_property_arg_extra() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, A = B)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
    }

    #[test]
    fn test_parse_property_arg_wrong() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(A = B, name)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
    }

    #[test]
    fn test_parse_property_no_name() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
    }

    #[test]
    fn test_parse_property_no_type() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
    }
}
