// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashSet;

use syn::{
    parse::{Error, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Ident, Lit, Meta, MetaNameValue, Path, Result, Token, Type,
};

/// Enum type for optional flags in qproperty macro
/// Can contain a string identifier of a custom getter, setter or notifier
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum QPropertyFlag {
    Read(Option<String>),
    Write(Option<String>),
    Notify(Option<String>),
}

impl QPropertyFlag {
    fn from_string(identifier: Ident, value: Option<String>) -> Result<Self> {
        return match identifier.to_string().as_str() {
            "read" => Ok(QPropertyFlag::Read(value)),
            "write" => Ok(QPropertyFlag::Write(value)),
            "notify" => Ok(QPropertyFlag::Notify(value)),
            _ => Err(Error::new(
                identifier.span(),
                "Flags must be one of read, write or notify",
            )),
        };
    }

    fn from_meta(meta_value: Meta) -> Result<Self> {
        match meta_value {
            Meta::Path(path) => Ok(Self::from_string(parse_path_to_ident(&path), None)?),
            Meta::NameValue(name_value) => {
                let kv_pair = parse_meta_name_value(&name_value)?;
                Ok(Self::from_string(kv_pair.0, Some(kv_pair.1))?)
            }
            _ => Err(Error::new(
                meta_value.span(),
                "Invalid syntax, flags must be specified as either `read` or `read = 'my_getter'`",
            )),
        }
    }
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

fn parse_path_to_ident(path: &Path) -> Ident {
    path.segments[0].ident.clone()
}

fn parse_meta_name_value(name_value: &MetaNameValue) -> Result<(Ident, String)> {
    let ident = parse_path_to_ident(&name_value.path);
    let expr = &name_value.value;
    let mut str_value: String = String::from("");
    match expr {
        syn::Expr::Lit(literal) => match &literal.lit {
            Lit::Str(str_lit) => str_value = str_lit.value(),
            _ => {
                return Err(Error::new(
                    expr.span(),
                    "Expression must be a string literal",
                ))
            }
        },
        _ => {
            return Err(Error::new(
                expr.span(),
                "Expression must be a string literal",
            ))
        }
    }

    return Ok((ident, str_value));
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

            let punctuated_flags: Punctuated<Meta, Token![,]> =
                Punctuated::parse_terminated(input)?;

            let flags: Vec<Meta> = punctuated_flags.into_iter().collect(); // Removes the commas while collecting into Vec

            let mut flag_set: HashSet<QPropertyFlag> = HashSet::new();

            for flag in flags {
                flag_set.insert(QPropertyFlag::from_meta(flag)?);
            }

            // TODO: later we'll need to parse setters and getters here
            // which are key-value, hence this not being parsed as a list

            Ok(Self {
                ident,
                ty,
                flags: flag_set,
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
        assert!(property.flags.contains(&QPropertyFlag::Read(None)));
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
        assert!(property.flags.contains(&QPropertyFlag::Read(None)));
        assert!(property.flags.contains(&QPropertyFlag::Write(None)));
        assert!(property.flags.contains(&QPropertyFlag::Notify(None)));
    }

    #[test]
    fn test_parse_kwargs() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read = "blah", write, notify = "blahblah")]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });
        assert!(property
            .flags
            .contains(&QPropertyFlag::Read(Some(String::from("blah")))));
        assert!(property.flags.contains(&QPropertyFlag::Write(None)));
        assert!(property
            .flags
            .contains(&QPropertyFlag::Notify(Some(String::from("blahblah")))));
    }

    #[test]
    fn test_parse_invalid_flag() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read = "blah", a, notify = "blahblah")]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err())
    }

    #[test]
    fn test_parse_invalid_flag_value() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read = blah, write, notify = "blahblah")]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err())
    }

    #[test]
    fn test_parse_missing_flags() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, notify = "blahblah")]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert!(property
            .flags
            .contains(&QPropertyFlag::Notify(Some(String::from("blahblah")))))
    }

    #[test]
    fn test_parse_invalid_literal() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, notify = 3)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
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
