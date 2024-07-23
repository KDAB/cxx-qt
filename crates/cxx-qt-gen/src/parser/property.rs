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
/// Enum representing the possible states of a flag passed to a QProperty
/// Auto is the state where a user passed for example `read` but no custom function
/// Custom(Ident) is the state where a user passed for example `read = my_getter` and the ident stored in this case would be `my_getter`
pub enum FlagState {
    Auto,
    Custom(Ident),
}

/// Struct for storing the flags provided for a QProperty
#[derive(Debug)]
pub struct QPropertyFlags {
    pub(crate) read: FlagState,
    pub(crate) write: Option<FlagState>,
    pub(crate) notify: Option<FlagState>,
    pub(crate) reset: Option<Ident>, // TODO: in future might be able to generate the function if T has a default
    pub(crate) is_final: bool,
    pub(crate) constant: bool,
    pub(crate) required: bool,
}

impl Default for QPropertyFlags {
    /// Default represents the flags of the desugared version of ```#[qproperty(T, ident)]```
    fn default() -> Self {
        Self {
            read: FlagState::Auto,
            write: Some(FlagState::Auto),
            notify: Some(FlagState::Auto),
            reset: None,
            is_final: false,
            constant: false,
            required: false,
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

                let mut constant = false;
                let mut required = false;
                let mut is_final = false;
                let mut reset = None;

                let map_auto_or_custom = |variable: &mut Option<FlagState>, value: &Option<Ident>| {
                    *variable = Some(value.as_ref().map_or(FlagState::Auto, |ident| FlagState::Custom(ident.clone())));
                };

                let invalid_flag_error = || Error::new(
                    ident.span(),
                    "Invalid flag passed, must be one of READ, WRITE, NOTIFY, RESET, CONSTANT, REQUIRED, FINAL",
                );

                // Create mutable closure to capture the variables for setting with the Meta values
                let mut update_fields = |ident: &Ident, value: Option<Ident>| -> Result<()> {
                    match ident.to_string().as_str() {
                        "READ" => map_auto_or_custom(&mut read, &value),
                        "WRITE" => map_auto_or_custom(&mut write, &value),
                        "NOTIFY" => map_auto_or_custom(&mut notify, &value),
                        "CONSTANT" => constant = true,
                        "REQUIRED" => required = true,
                        "FINAL" => is_final = true,
                        "RESET" => reset = Some(
                                value.ok_or_else(|| Error::new(
                                    ident.span(),
                                    "Reset flag must be given a user defined reset function, like `RESET = my_reset`",
                                ))?
                            ),
                        _ => {
                            return Err(Error::new(
                                ident.span(),
                                "Invalid flag passed, must be one of READ, WRITE, NOTIFY, RESET, CONSTANT, REQUIRED, FINAL",
                            ));
                        }
                    };

                    Ok(())
                };

                for flag in flags {
                    let (field, maybe_value) = parse_meta(flag)?;
                    update_fields(&field, maybe_value)?;
                }

                // Constance check
                if constant && (write.is_some() || notify.is_some()) {
                    return Err(Error::new(
                        punctuated_flags.span(),
                        "constant properties cannot have a setter or notify signal",
                    ))
                }

                if let Some(read) = read {
                    Ok(Self {
                        ident,
                        ty,
                        flags: QPropertyFlags {
                            read,
                            write,
                            notify,
                            reset,
                            is_final,
                            constant,
                            required,
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
    fn test_parse_constant_incorrect() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ, WRITE, NOTIFY, CONSTANT)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
    }

    #[test]
    fn test_parse_constant() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ, CONSTANT)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert!(property.flags.constant);
    }

    #[test]
    fn test_parse_reset_incorrect() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ, RESET)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
    }

    #[test]
    fn test_parse_property() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ, WRITE = myGetter,)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });
    }

    #[test]
    fn test_parse_flags_read() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });
    }

    #[test]
    fn test_parse_flags_all() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ, WRITE, NOTIFY, REQUIRED, RESET = my_reset, FINAL)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });

        assert_eq!(property.flags.read, FlagState::Auto);
        assert_eq!(property.flags.read, FlagState::Auto);

        assert_eq!(property.flags.write, Some(FlagState::Auto));
        assert_eq!(property.flags.notify, Some(FlagState::Auto));

        assert!(property.flags.required);
        assert!(property.flags.is_final);

        assert_eq!(property.flags.reset, Some(format_ident!("my_reset")));
        assert_eq!(property.flags.notify, Some(FlagState::Auto));
        assert_eq!(property.flags.write, Some(FlagState::Auto));
    }

    #[test]
    fn test_parse_flags_kw() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ = my_getter, WRITE, NOTIFY = my_notifier)]
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

        assert_eq!(
            property.flags.notify,
            Some(FlagState::Custom(format_ident!("my_notifier")))
        );
    }

    #[test]
    fn test_parse_flags_invalid() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ = blah, a, NOTIFY = blahblah)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err())
    }

    #[test]
    fn test_parse_flags_missing_read() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, NOTIFY = blahblah)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
    }

    #[test]
    fn test_parse_flags_invalid_literal() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, NOTIFY = 3)]
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
