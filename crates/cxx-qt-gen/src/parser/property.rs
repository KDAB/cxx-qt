// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{
    parse::{Error, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Expr, Ident, Meta, MetaNameValue, Path, Result, Token, Type,
};

/// An optional identifier of the functions passed with flags
/// e.g. read = my_getter, IdentFlag would be used to store my_getter
type IdentFlag = Option<Ident>;

/// Struct for storing the flags provided for a QProperty and their optional identifiers ([IdentFlag])
#[derive(Debug)]
pub struct QPropertyFlags {
    // TODO: Refactor to use an enum type for representing all 3 states of write and notify
    // Or use an enum for {Auto, Custom(Ident)}, this is optional for write and notify, but not read
    pub(crate) read: IdentFlag,
    pub(crate) write: Option<IdentFlag>,
    pub(crate) notify: Option<IdentFlag>,
}

impl Default for QPropertyFlags {
    // Default represents the flags of the desugared version of #[qproperty(T, ident)]
    fn default() -> Self {
        Self {
            read: None,
            write: Some(None),
            notify: Some(None),
        }
    }
}

impl QPropertyFlags {
    // Doesn't really represent a realistic state, just used for collecting in the parser
    pub fn new() -> Self {
        Self {
            read: None,
            write: None,
            notify: None,
        }
    }

    /// Takes an [Ident] and matches it to the valid flags, setting fields if a match is found, otherwise will error
    fn set_field_by_ident(&mut self, key: Ident, value: IdentFlag) -> Result<String> {
        match key.to_string().as_str() {
            "read" => self.read = value,
            "write" => self.write = Some(value),
            "notify" => self.notify = Some(value),
            _ => {
                return Err(Error::new(
                    key.span(), // TODO: check if this is an acceptable form of erroring for non Syn functions
                    "Invalid flag passed, must be one of read, write, notify",
                ));
            }
        };
        Ok(key.to_string()) // Return the string back up for checking which flags were set
    }

    fn add_from_meta(&mut self, meta_value: Meta) -> Result<String> {
        match meta_value {
            Meta::Path(path) => Ok(self.set_field_by_ident(parse_path_to_ident(&path), None)?),
            Meta::NameValue(name_value) => {
                let kv_pair = parse_meta_name_value(&name_value)?;
                let value: IdentFlag = Some(kv_pair.1);

                Ok(self.set_field_by_ident(kv_pair.0, value)?)
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
    /// Property flag collection
    pub flags: QPropertyFlags,
}

fn parse_path_to_ident(path: &Path) -> Ident {
    // when used, should only ever contain 1 path segment
    path.segments[0].ident.clone()
}

// TODO: Returning struct instead of tuple might be more descriptive
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

                let mut passed_flags = QPropertyFlags::new();

                // Used to check that if flags are specified, read is included
                // Can be removed in lieu of checking fields if the enum approach is used
                let mut flag_strings: Vec<String> = Vec::new();

                for flag in flags {
                    let flag_string: String = passed_flags.add_from_meta(flag)?;
                    flag_strings.push(flag_string);
                }

                if !flag_strings.contains(&String::from("read")) {
                    return Err(Error::new(
                        punctuated_flags.span(),
                        "If flags are passed, read must be explicitly specified",
                    ));
                }

                Ok(Self {
                    ident,
                    ty,
                    flags: passed_flags,
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
        assert!(property.flags.write.is_some());
        assert!(property.flags.notify.is_some());
    }

    #[test]
    fn test_parse_flags_kw() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read = blah, write, notify = blahblah)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });

        assert!(property.flags.read.is_some());
        assert_eq!(property.flags.read.unwrap(), format_ident!("blah"));

        assert!(property.flags.write.is_some());

        assert!(property.flags.notify.is_some());
        let notify = property.flags.notify.unwrap();
        assert!(notify.is_some());
        assert_eq!(notify.unwrap(), format_ident!("blahblah"));
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
    fn test_parse_missing_flags() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, notify = blahblah)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
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
            #[qproperty(num)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
    }
}
