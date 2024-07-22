// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{
    parse::{Error, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Expr, Ident, Meta, MetaNameValue, Result, Token, Type,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlagState {
    Auto, // Might need to refactor to also store the generated ident here
    Custom(Ident),
}

impl FlagState {
    #[cfg(test)]
    pub fn is_auto(&self) -> bool {
        matches!(self, Self::Auto)
    }
}

/// Struct for storing the flags provided for a QProperty
#[derive(Debug)]
pub struct QPropertyFlags {
    pub(crate) read: FlagState,
    pub(crate) write: Option<FlagState>,
    pub(crate) notify: Option<FlagState>,
}

impl Default for QPropertyFlags {
    /// Default represents the flags of the desugared version of ```#[qproperty(T, ident)]```
    fn default() -> Self {
        Self {
            read: FlagState::Auto,
            write: Some(FlagState::Auto),
            notify: Some(FlagState::Auto),
        }
    }
}

/// Describes a single Q_PROPERTY for a struct
pub struct ParsedQProperty {
    /// The [syn::Ident] of the property
    pub ident: Ident,
    /// The [syn::Type] of the property
    pub ty: Type,
    /// Property flag collection
    pub flags: QPropertyFlags,
}

fn parse_meta_name_value(name_value: &MetaNameValue) -> Result<(Ident, Ident)> {
    let ident = name_value.path.require_ident()?.clone();
    let expr = &name_value.value;
    let func_signature: Ident;

    if let Expr::Path(path_expr) = expr {
        func_signature = path_expr.path.require_ident()?.clone();
    } else {
        return Err(Error::new(
            expr.span(),
            "Function signature must be an identifier",
        ));
    }

    Ok((ident, func_signature))
}

fn parse_meta(meta: Meta) -> Result<(Ident, Option<Ident>)> {
    match meta {
        Meta::Path(path) => Ok((path.require_ident()?.clone(), None)),
        Meta::NameValue(name_value) => {
            let (field, ident) = parse_meta_name_value(&name_value)?;
            Ok((field, Some(ident)))
        }
        _ => Err(Error::new(
            meta.span(),
            "Invalid syntax, flags must be specified as either `read` or `read = my_getter`",
        )),
    }
}

impl ParsedQProperty {
    pub fn parse(attr: Attribute) -> Result<Self> {
        attr.parse_args_with(|input: ParseStream| -> Result<Self> {
            let ty = input.parse()?;
            let _comma = input.parse::<Token![,]>()?;
            let ident = input.parse()?;

            if input.is_empty() {
                // No flags passed so desugar: #[qproperty(T, ident)] -> #[qproperty(T, ident, read, write, notify)]
                Ok(Self {
                    ident,
                    ty,
                    flags: QPropertyFlags::default(),
                })
            } else {
                let _comma = input.parse::<Token![,]>()?; // Start of final identifiers

                let punctuated_flags: Punctuated<Meta, Token![,]> =
                    Punctuated::parse_terminated(input)?;

                // Remove the commas and collect the individual meta items
                let flags: Vec<Meta> = punctuated_flags.clone().into_iter().collect();

                let mut read = None;
                let mut write = None;
                let mut notify = None;

                // Create mutable closure to capture the variables for setting with the Meta values
                let mut update_fields = |ident: &Ident, value: Option<Ident>| -> Result<()> {
                    let variable = match ident.to_string().as_str() {
                        "read" => &mut read,
                        "write" => &mut write,
                        "notify" => &mut notify,
                        _ => {
                            return Err(Error::new(
                                ident.span(),
                                "Invalid flag passed, must be one of read, write, notify",
                            ));
                        }
                    };
                    *variable = Some(value.map_or(FlagState::Auto, FlagState::Custom));

                    Ok(())
                };

                for flag in flags {
                    let (field, maybe_value) = parse_meta(flag)?;
                    update_fields(&field, maybe_value)?;
                }

                if let Some(read) = read {
                    Ok(Self {
                        ident,
                        ty,
                        flags: QPropertyFlags {
                            read,
                            write,
                            notify,
                        },
                    })
                } else {
                    Err(Error::new(
                        punctuated_flags.span(),
                        "If flags are passed, read must be explicitly specified",
                    ))
                }
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
            #[qproperty(T, name, read, write = myGetter,)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });
    }

    #[test]
    fn test_parse_flags_read() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });
    }

    #[test]
    fn test_parse_flags_all() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read, write, notify)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });

        assert!(property.flags.read.is_auto());

        assert!(property.flags.write.is_some());
        assert!(property.flags.notify.is_some());

        assert!(property.flags.write.unwrap().is_auto());
        assert!(property.flags.notify.unwrap().is_auto());
    }

    #[test]
    fn test_parse_flags_kw() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read = my_getter, write, notify = my_notifier)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });

        assert_eq!(
            property.flags.read,
            FlagState::Custom(format_ident!("my_getter"))
        );

        assert_eq!(property.flags.write, Some(FlagState::Auto));

        assert!(property.flags.notify.is_some());

        assert_eq!(
            property.flags.notify,
            Some(FlagState::Custom(format_ident!("my_notifier")))
        );
    }

    #[test]
    fn test_parse_flags_invalid() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read = blah, a, notify = blahblah)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err())
    }

    #[test]
    fn test_parse_flags_missing_read() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, notify = blahblah)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
    }

    #[test]
    fn test_parse_flags_invalid_literal() {
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
    fn test_parse_property_no_params() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty()]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
    }
}
