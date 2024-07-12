// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashSet;

use syn::{
    parse::{Error, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Expr, Ident, Meta, MetaNameValue, Path, Result, Token, Type,
};

/// Enum type for optional flags in qproperty macro
/// Can contain a string identifier of a custom getter, setter or notifier
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum QPropertyFlag {
    Read(Option<Ident>),
    Write(Option<Ident>),
    Notify(Option<Ident>),
}

impl QPropertyFlag {
    fn from_ident(identifier: Ident, value: Option<Ident>) -> Result<Self> {
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
            Meta::Path(path) => Ok(Self::from_ident(parse_path_to_ident(&path), None)?),
            Meta::NameValue(name_value) => {
                let kv_pair = parse_meta_name_value(&name_value)?;
                Ok(Self::from_ident(kv_pair.0, Some(kv_pair.1))?)
            }
            _ => Err(Error::new(
                meta_value.span(),
                "Invalid syntax, flags must be specified as either `read` or `read = my_getter`",
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

fn parse_meta_name_value(name_value: &MetaNameValue) -> Result<(Ident, Ident)> {
    let ident = parse_path_to_ident(&name_value.path);
    let expr = &name_value.value;
    let func_signature: Ident;

    if let Expr::Path(path_expr) = expr {
        func_signature = parse_path_to_ident(&path_expr.path);
    } else {
        return Err(Error::new(
            expr.span(),
            "Function signature must be an identifier",
        ));
    }

    Ok((ident, func_signature))
}

impl ParsedQProperty {
    pub fn parse(attr: Attribute) -> Result<Self> {
        attr.parse_args_with(|input: ParseStream| -> Result<Self> {
            let ty = input.parse()?;
            let _comma = input.parse::<Token![,]>()?;
            let ident = input.parse()?;

            let mut flag_set = HashSet::new();

            if input.is_empty() {
                flag_set.insert(QPropertyFlag::Read(None));
                flag_set.insert(QPropertyFlag::Write(None));
                flag_set.insert(QPropertyFlag::Notify(None));

                // No flags so fill with default options
                Ok(Self {
                    ident,
                    ty,
                    flags: flag_set,
                })
            } else {
                let _comma = input.parse::<Token![,]>()?; // Start of final identifiers

                let punctuated_flags: Punctuated<Meta, Token![,]> =
                    Punctuated::parse_terminated(input)?;

                let flags: Vec<Meta> = punctuated_flags.into_iter().collect(); // Removes the commas while collecting into Vec

                let mut flag_set: HashSet<QPropertyFlag> = HashSet::new();

                for flag in flags {
                    flag_set.insert(QPropertyFlag::from_meta(flag)?);
                }

                Ok(Self {
                    ident,
                    ty,
                    flags: flag_set,
                })
            }
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
            #[qproperty(T, name, read = blah, write, notify = blahblah)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });
        assert!(property
            .flags
            .contains(&QPropertyFlag::Read(Some(format_ident!("blah")))));
        assert!(property.flags.contains(&QPropertyFlag::Write(None)));
        assert!(property
            .flags
            .contains(&QPropertyFlag::Notify(Some(format_ident!("blahblah")))));
    }

    #[test]
    fn test_parse_invalid_flag() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read = blah, a, notify = blahblah)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err())
    }

    #[test]
    fn test_parse_missing_flags() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, notify = blahblah)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert!(property
            .flags
            .contains(&QPropertyFlag::Notify(Some(format_ident!("blahblah")))))
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
