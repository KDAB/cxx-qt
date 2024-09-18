// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::naming::property::property_name_from_rust_name;
use crate::naming::Name;
use crate::syntax::expr::expr_to_string;
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
    pub name: Name,
    /// The [syn::Type] of the property
    pub ty: Type,
    /// Property flag collection
    pub flags: QPropertyFlags,
}

fn parse_meta_name_value(name_value: &MetaNameValue) -> Result<(Ident, Ident)> {
    let ident = name_value.path.require_ident()?.clone();
    let expr = &name_value.value;

    // Only cxx_name and rust_name can use string literals for values
    let value = if ident == "cxx_name" || ident == "rust_name" {
        let string = expr_to_string(expr).map_err(|_| {
            Error::new(
                ident.span(),
                "cxx_name and rust_name must use string values like `cxx_name = \"myName\"`!",
            )
        })?;
        syn::parse_str::<Ident>(&string)?
    } else if let Expr::Path(path_expr) = expr {
        path_expr.path.require_ident()?.clone()
    } else {
        return Err(Error::new(
            expr.span(),
            "Function signatures must be identifiers!",
        ));
    };

    Ok((ident, value))
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
            "Invalid syntax, flags must be specified as either `READ` or `READ = my_getter`!",
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
                    name: property_name_from_rust_name(ident),
                    ty,
                    flags: QPropertyFlags::default(),
                })
            } else {
                let _comma = input.parse::<Token![,]>()?; // Start of final identifiers

                let punctuated_flags: Punctuated<Meta, Token![,]> =
                    Punctuated::parse_terminated(input)?;

                // Remove the commas and collect the individual meta items
                let flags: Vec<Meta> = punctuated_flags.clone().into_iter().collect();
                let mut read_required = false;

                let mut read = None;
                let mut write = None;
                let mut notify = None;

                let mut constant = false;
                let mut required = false;
                let mut is_final = false;
                let mut reset = None;
                let mut cxx_name = None;
                let mut rust_name = None;

                let map_auto_or_custom = |variable: &mut Option<FlagState>, value: &Option<Ident>| {
                    *variable = Some(value.as_ref().map_or(FlagState::Auto, |ident| FlagState::Custom(ident.clone())));
                };

                // Create mutable closure to capture the variables for setting with the Meta values
                let mut update_fields = |ident: &Ident, value: Option<Ident>| -> Result<()> {
                    // Closure for flags which must be a NameValue, returning an Error if they aren't
                    let require_value = |item: &str, usage: &str| -> Result<Option<Ident>> {
                        Some(
                            value.clone().ok_or_else(|| Error::new(
                                ident.span(),
                                format!("{item} needs a value passed like `{usage}`!"),
                            ))
                        ).transpose()
                    };

                    // Matches the flag, and if it is a QProperty flag instead of a naming flag set flag to ensure READ was passed
                    match ident.to_string().as_str() {
                        "cxx_name" => cxx_name = require_value("cxx_name", "cxx_name = \"myName\"")?,
                        "rust_name" => rust_name = require_value("rust_name", "rust_name = \"my_name\"")?,
                        property_flag => {
                            read_required = true;
                            match property_flag {
                                "READ" => map_auto_or_custom(&mut read, &value),
                                "WRITE" => map_auto_or_custom(&mut write, &value),
                                "NOTIFY" => map_auto_or_custom(&mut notify, &value),
                                "CONSTANT" => constant = true,
                                "REQUIRED" => required = true,
                                "FINAL" => is_final = true,
                                "RESET" => reset = require_value("RESET flag", "RESET = my_reset_fn")?,
                                _ => return Err(Error::new(
                                    ident.span(),
                                    "Invalid flag passed!, must be one of\n  READ, WRITE, NOTIFY, RESET, CONSTANT, REQUIRED, FINAL or cxx_name / rust_name",
                                ))
                            }
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
                        "QProperties marked as CONSTANT cannot have a setter or notify signal!",
                    ))
                }

                let name = Name::new(ident).with_options(cxx_name.map(|ident| ident.to_string()), rust_name, true);

                // This check is needed otherwise this fn would error unless READ, WRITE, etc... was passed with cxx_name
                if read_required {
                    if let Some(read) = read {
                        Ok(Self {
                            name,
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
                            "If any flags are passed, READ must be explicitly specified!",
                        ))
                    }
                } else {
                    Ok(Self {
                        name,
                        ty,
                        flags: QPropertyFlags::default(), // This block is hit if no flags, or only cxx / rust name were passed
                    })
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_errors;
    use quote::format_ident;
    use syn::{parse_quote, ItemStruct};

    #[test]
    fn test_parse_named_property() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, cxx_name = "myName", rust_name = "my_name")]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();

        assert_eq!(property.name.cxx_unqualified(), "myName");
        assert_eq!(property.name.rust_unqualified(), "my_name");
    }

    #[test]
    fn test_parse_invalid() {
        assert_parse_errors! {
            ParsedQProperty::parse =>

            // Non-constant property with constant flag
            { #[qproperty(T, name, READ, WRITE, NOTIFY, CONSTANT)] }
            // Reset was not provided a function
            { #[qproperty(T, name, READ, RESET)] }
            // Unknown flag
            { #[qproperty(T, name, READ = blah, a, NOTIFY = blahblah)] }
            // Invalid function specification syntax
            { #[qproperty(T, name, READ(my_getter))] }
            // Read missing
            { #[qproperty(T, name, NOTIFY = blahblah)] }
            // Invalid type for custom fn
            { #[qproperty(T, name, NOTIFY = 3)] }
            // Invalid type format
            { #[qproperty(A = B, name)] }
            // Extra non-flag arg
            { #[qproperty(T, name, A = B)] }
            // Name missing
            { #[qproperty(T)] }
            // No args
            { #[qproperty()] }
            // cxx_name should be a string
            { #[qproperty(T, name, cxx_name = myName)] }
            // cxx_name should be a valid non_empty ident
            { #[qproperty(T, name, cxx_name = "")] }
            // cxx_name had no value provided
            { #[qproperty(T, name, cxx_name)] }
        }
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
    fn test_parse_property() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ, WRITE = myGetter,)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.name.rust_unqualified(), "name");
        assert_eq!(property.ty, parse_quote! { T });
    }

    #[test]
    fn test_parse_flags_read() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.name.rust_unqualified(), "name");
        assert_eq!(property.ty, parse_quote! { T });
    }

    #[test]
    fn test_parse_flags_all() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, READ, WRITE, NOTIFY, REQUIRED, RESET = my_reset, FINAL)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.name.rust_unqualified(), "name");
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
        assert_eq!(property.name.rust_unqualified(), "name");
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
}
