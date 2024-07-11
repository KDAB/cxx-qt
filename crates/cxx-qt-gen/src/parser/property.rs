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

use quote::format_ident;

/// An optional identifier of the functions passed with flags
/// e.g. read = my_getter, IdentFlag would be used to store my_getter
type IdentFlag = Option<Ident>;

/// Struct for storing the flags provided for a QProperty and their optional identifiers ([IdentFlag])
#[derive(Debug)]
pub struct QPropertyFlags {
    read: Option<IdentFlag>, // TODO: maybe change this to better represent the data
    write: Option<IdentFlag>,
    notify: Option<IdentFlag>,
}

impl QPropertyFlags {
    pub fn new() -> Self {
        Self {
            read: None,
            write: None,
            notify: None,
        }
    }

    pub fn all_flags() -> Self {
        Self {
            read: None,
            write: Some(None),
            notify: Some(None),
        }
    }

    fn set_field_by_string(&mut self, key: String, value: IdentFlag) -> Result<()> {
        match key.as_str() {
            "read" => self.read = Some(value),
            "write" => self.write = Some(value),
            "notify" => self.notify = Some(value),
            _ => {
                return Err(Error::new(
                    format_ident!("{}", key).span(), // TODO: check if this is an acceptable form of erroring for non Syn functions
                    "Invalid flag passed, must be one of read, write, notify",
                ));
            }
        };
        Ok(())
    }

    fn add_from_meta(&mut self, meta_value: Meta) -> Result<()> {
        match meta_value {
            Meta::Path(path) => {
                let ident: String = parse_path_to_ident(&path).to_string();

                self.set_field_by_string(ident, None)?;
                Ok(())
            }
            Meta::NameValue(name_value) => {
                let kv_pair = parse_meta_name_value(&name_value)?;

                let ident: String = kv_pair.0.to_string();
                let value: IdentFlag = Some(kv_pair.1);

                self.set_field_by_string(ident, value)?;
                Ok(())
            }
            _ => Err(Error::new(
                meta_value.span(),
                "Invalid syntax, flags must be specified as either `read` or `read = my_getter`",
            )),
        };
        Ok(())
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
    path.segments[0].ident.clone()
}

// Returning struct instead of tuple might be more descriptive
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
                // No flags so fill with default options
                Ok(Self {
                    ident,
                    ty,
                    flags: QPropertyFlags::all_flags(),
                })
            } else {
                let _comma = input.parse::<Token![,]>()?; // Start of final identifiers

                let punctuated_flags: Punctuated<Meta, Token![,]> =
                    Punctuated::parse_terminated(input)?;

                let flags: Vec<Meta> = punctuated_flags.into_iter().collect(); // Removes the commas while collecting into Vec

                let mut passed_flags = QPropertyFlags::new();

                for flag in flags {
                    passed_flags.add_from_meta(flag)?;
                }

                println!("Final flags: {:?}", passed_flags);

                if passed_flags.read.is_none() {
                    return Err(Error::new(
                        input.span(),
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
    fn test_parse_read_flag() {
        let mut input: ItemStruct = parse_quote! {
            #[qproperty(T, name, read)]
            struct MyStruct;
        };
        let property = ParsedQProperty::parse(input.attrs.remove(0)).unwrap();
        assert_eq!(property.ident, format_ident!("name"));
        assert_eq!(property.ty, parse_quote! { T });
        // assert!(property.flags.contains(&QPropertyFlags::Read(None)));
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
        // assert!(property.flags.contains(&QPropertyFlags::Read(None)));
        // assert!(property.flags.contains(&QPropertyFlags::Write(None)));
        // assert!(property.flags.contains(&QPropertyFlags::Notify(None)));
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
        // assert!(property
        //     .flags
        //     .contains(&QPropertyFlag::Read(Some(format_ident!("blah")))));
        // assert!(property.flags.contains(&QPropertyFlag::Write(None)));
        // assert!(property
        //     .flags
        //     .contains(&QPropertyFlag::Notify(Some(format_ident!("blahblah")))));
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
        let property = ParsedQProperty::parse(input.attrs.remove(0));
        assert!(property.is_err());
        // assert!(property
        //     .flags
        //     .contains(&QPropertyFlag::Notify(Some(format_ident!("blahblah")))))
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
