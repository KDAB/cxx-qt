// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod constructor;
pub mod cxxqtdata;
pub mod externcxxqt;
pub mod externqobject;
pub mod inherit;
pub mod method;
pub mod parameter;
pub mod property;
pub mod qenum;
pub mod qnamespace;
pub mod qobject;
pub mod signals;
pub mod trait_impl;

use crate::{
    naming::TypeNames,
    syntax::{expr::expr_to_string, path::path_compare_str, safety::Safety},
};
use convert_case::Case;
use cxxqtdata::ParsedCxxQtData;
use std::collections::BTreeMap;
use syn::{
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Semi},
    Attribute, Error, Expr, ForeignItemFn, Ident, Item, ItemMod, Meta, Result, Token, Visibility,
};

#[derive(Copy, Clone)]
pub struct CaseConversion {
    pub cxx: Option<Case>,
    pub rust: Option<Case>,
}

/// Used to match the auto_case attributes and turn it into a Case to convert to
fn meta_to_case(attr: &Attribute, default: Case) -> Result<Case> {
    match &attr.meta {
        Meta::Path(_) => Ok(default),
        Meta::NameValue(case) => match &case.value {
            Expr::Path(expr_path) => match expr_path.path.require_ident()?.to_string().as_str() {
                "Camel" => Ok(Case::Camel),
                "Snake" => Ok(Case::Snake),
                _ => Err(Error::new(
                    attr.span(),
                    "Invalid case! You can use either `Camel` or `Snake`",
                )),
            },
            _ => Err(Error::new(
                attr.span(),
                "Case should be specified as an identifier! Like `#[auto_cxx_name = Camel]`",
            )),
        },
        _ => Err(Error::new(
            attr.span(),
            "Invalid attribute format! Use like `auto_cxx_name` or `auto_cxx_name = Camel`",
        )),
    }
}

impl CaseConversion {
    pub fn none() -> Self {
        Self {
            cxx: None,
            rust: None,
        }
    }

    /// Create a CaseConversion object from a Map of attributes, collected using `require_attributes`
    /// Parses both `auto_cxx_name` and `auto_cxx_name = Camel`
    pub fn from_attrs(attrs: &BTreeMap<&str, &Attribute>) -> Result<Self> {
        let rust = attrs
            .get("auto_rust_name")
            .map(|attr| meta_to_case(attr, Case::Snake))
            .transpose()?;
        let cxx = attrs
            .get("auto_cxx_name")
            .map(|attr| meta_to_case(attr, Case::Camel))
            .transpose()?;

        Ok(Self { rust, cxx })
    }
}

/// Validates that an invokable is either unsafe, or is in an unsafe extern block
fn check_safety(method: &ForeignItemFn, safety: &Safety) -> Result<()> {
    if safety == &Safety::Unsafe && method.sig.unsafety.is_none() {
        Err(Error::new(
            method.span(),
            "Invokables must be marked as unsafe or wrapped in an `unsafe extern \"RustQt\"` block!",
        ))
    } else {
        Ok(())
    }
}

/// Iterate the attributes of the method to extract Doc attributes (doc comments are parsed as this)
pub fn extract_docs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|attr| path_compare_str(attr.meta.path(), &["doc"]))
        .cloned()
        .collect()
}

/// Splits a path by :: separators e.g. "cxx_qt::bridge" becomes ["cxx_qt", "bridge"]
fn split_path(path_str: &str) -> Vec<&str> {
    let path = if path_str.contains("::") {
        path_str.split("::").collect::<Vec<_>>()
    } else {
        vec![path_str]
    };
    path
}

/// Collects a Map of all attributes found from the allowed list
/// Will error if an attribute which is not in the allowed list is found
pub fn require_attributes<'a>(
    attrs: &'a [Attribute],
    allowed: &'a [&str],
) -> Result<BTreeMap<&'a str, &'a Attribute>> {
    let mut output = BTreeMap::default();
    for attr in attrs {
        let index = allowed
            .iter()
            .position(|string| path_compare_str(attr.meta.path(), &split_path(string)));
        if let Some(index) = index {
            output.insert(allowed[index], attr); // Doesn't error on duplicates
        } else {
            return Err(Error::new(
                attr.span(),
                format!(
                    "Unsupported attribute! The only attributes allowed on this item are\n{}",
                    allowed.join(", ")
                ),
            ));
        }
    }
    Ok(output)
}

/// Struct representing the necessary components of a cxx mod to be passed through to generation
pub struct PassthroughMod {
    pub(crate) items: Option<Vec<Item>>,
    pub(crate) docs: Vec<Attribute>,
    pub(crate) module_ident: Ident,
    pub(crate) vis: Visibility,
}

impl PassthroughMod {
    /// Parse an item mod into it's components
    pub fn parse(module: ItemMod) -> Self {
        let items = module.content.map(|(_, items)| items);

        Self {
            items,
            docs: extract_docs(&module.attrs),
            module_ident: module.ident,
            vis: module.vis,
        }
    }
}

/// A struct representing a module block with CXX-Qt relevant [syn::Item]'s
/// parsed into ParsedCxxQtData, to be used later to generate Rust & C++ code.
///
/// [syn::Item]'s that are not handled specially by CXX-Qt are passed through for CXX to process.
pub struct Parser {
    /// The module which unknown (eg CXX) blocks are stored into
    pub(crate) passthrough_module: PassthroughMod,
    /// Any CXX-Qt data that needs generation later
    pub(crate) cxx_qt_data: ParsedCxxQtData,
    /// all type names that were found in this module, including CXX types
    pub(crate) type_names: TypeNames,
}

impl Parser {
    fn parse_mod_attributes(module: &mut ItemMod) -> Result<Option<String>> {
        let attrs = require_attributes(&module.attrs, &["doc", "cxx_qt::bridge"])?;
        let mut namespace = None;

        // Check for the cxx_qt::bridge attribute
        if let Some(attr) = attrs.get("cxx_qt::bridge") {
            // If we are not #[cxx_qt::bridge] but #[cxx_qt::bridge(A = B)] then process
            if !matches!(attr.meta, Meta::Path(_)) {
                let nested =
                    attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                for meta in nested {
                    match meta {
                        Meta::NameValue(ref name_value) => {
                            // Parse any namespace in the cxx_qt::bridge macro
                            if name_value.path.is_ident("namespace") {
                                namespace = Some(expr_to_string(&name_value.value)?);
                                // Parse any custom file stem
                            } else if name_value.path.is_ident("cxx_file_stem") {
                                return Err(Error::new(
                                    meta.span(),
                                    "cxx_file_stem is unsupported, instead the input file name will be used",
                                ));
                            }
                        }
                        _others => {}
                    }
                }
            }
        } else {
            return Err(Error::new(
                module.span(),
                "Tried to parse a module which doesn't have a cxx_qt::bridge attribute!",
            ));
        }

        Ok(namespace)
    }

    fn parse_module_contents(
        mut module: ItemMod,
        namespace: Option<String>,
    ) -> Result<(ParsedCxxQtData, ItemMod)> {
        let mut others = vec![];

        let mut cxx_qt_data = ParsedCxxQtData::new(module.ident.clone(), namespace);

        // Check that there are items in the module
        if let Some((_, items)) = module.content {
            // Loop through items and load into qobject or others and populate mappings
            for item in items.into_iter() {
                // Try to find any CXX-Qt items, if found add them to the relevant
                // qobject or extern C++Qt block. Otherwise return them to be added to other
                if let Some(other) = cxx_qt_data.parse_cxx_qt_item(item)? {
                    // Unknown item so add to the other list
                    others.push(other);
                }
            }
        }

        // Create a new module using only items that are not CXX-Qt items
        if !others.is_empty() {
            module.content = Some((Brace::default(), others));
            module.semi = None;
        } else {
            module.content = None;
            module.semi = Some(Semi::default());
        }
        Ok((cxx_qt_data, module))
    }

    /// The "Naming phase", it generates a list of all nameable types in our bridge.
    fn naming_phase(
        cxx_qt_data: &mut ParsedCxxQtData,
        cxx_items: &[Item],
        module_ident: &Ident,
    ) -> Result<TypeNames> {
        TypeNames::from_parsed_data(
            cxx_qt_data,
            cxx_items,
            cxx_qt_data.namespace.as_deref(),
            module_ident,
        )
    }

    /// Constructs a Parser object from a given [syn::ItemMod] block
    pub fn from(mut module: ItemMod) -> Result<Self> {
        let namespace = Self::parse_mod_attributes(&mut module)?;
        let (mut cxx_qt_data, module) = Self::parse_module_contents(module, namespace)?;
        let type_names = Self::naming_phase(
            &mut cxx_qt_data,
            module
                .content
                .as_ref()
                .map(|brace_and_items| &brace_and_items.1)
                .unwrap_or(&vec![]),
            &module.ident,
        )?;

        // Return the successful Parser object
        Ok(Self {
            passthrough_module: PassthroughMod::parse(module),
            type_names,
            cxx_qt_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::assert_parse_errors;
    use pretty_assertions::assert_eq;
    use quote::format_ident;
    use syn::{parse_quote, ItemMod, Type};

    /// Helper which returns a f64 as a [syn::Type]
    pub fn f64_type() -> Type {
        parse_quote! { f64 }
    }
    #[test]
    fn test_parser_from_empty_module() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {}
        };
        let parser = Parser::from(module).unwrap();

        assert!(parser.passthrough_module.items.is_none());
        assert!(parser.passthrough_module.docs.is_empty());
        assert_eq!(parser.passthrough_module.module_ident, "ffi");
        assert_eq!(parser.passthrough_module.vis, Visibility::Inherited);
        assert_eq!(parser.cxx_qt_data.namespace, None);
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 0);
    }

    #[test]
    fn test_incorrect_bridge_args() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(a, b, c)]
            mod ffi {
                extern "Rust" {
                    fn test();
                }
            }
        };
        assert!(Parser::from(module).is_ok()); // Meta::List args in cxx_qt bridge are ignored

        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(a = b)]
            mod ffi {
                extern "Rust" {
                    fn test();
                }
            }
        };
        assert!(Parser::from(module).is_ok()); // Meta::NameValue args which aren't `namespace` or `cxx_file_stem` are ignored
    }

    #[test]
    fn test_parser_from_cxx_items() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "Rust" {
                    fn test();
                }
            }
        };
        let parser = Parser::from(module).unwrap();
        assert_eq!(parser.passthrough_module.items.unwrap().len(), 1);
        assert!(parser.passthrough_module.docs.is_empty());
        assert_eq!(parser.passthrough_module.module_ident, "ffi");
        assert_eq!(parser.passthrough_module.vis, Visibility::Inherited);
        assert_eq!(parser.cxx_qt_data.namespace, None);
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 0);
    }

    #[test]
    fn test_parser_from_cxx_qt_items() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }

                unsafe extern "RustQt" {
                    #[qsignal]
                    fn ready(self: Pin<&mut MyObject>);
                }
            }
        };
        let parser = Parser::from(module.clone()).unwrap();

        assert!(parser.passthrough_module.items.is_none());
        assert!(parser.passthrough_module.docs.is_empty());
        assert_eq!(parser.passthrough_module.module_ident, "ffi");
        assert_eq!(parser.passthrough_module.vis, Visibility::Inherited);
        assert_eq!(parser.cxx_qt_data.namespace, Some("cxx_qt".to_owned()));
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 1);
        assert_eq!(parser.type_names.num_types(), 18);
        assert_eq!(
            parser
                .type_names
                .rust_qualified(&format_ident!("MyObject"))
                .unwrap(),
            parse_quote! { ffi::MyObject }
        );
        assert_eq!(
            parser
                .type_names
                .rust_qualified(&format_ident!("MyObjectRust"))
                .unwrap(),
            parse_quote! { ffi::MyObjectRust }
        );
    }

    #[test]
    fn test_parser_from_cxx_and_cxx_qt_items() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            /// A cxx_qt::bridge module
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }

                unsafe extern "RustQt" {
                    #[qsignal]
                    fn ready(self: Pin<&mut MyObject>);
                }

                extern "Rust" {
                    fn test();
                }
            }
        };
        let parser = Parser::from(module.clone()).unwrap();

        assert_eq!(parser.passthrough_module.items.unwrap().len(), 1);
        assert_eq!(parser.passthrough_module.docs.len(), 1);
        assert_eq!(parser.passthrough_module.module_ident, "ffi");
        assert_eq!(parser.passthrough_module.vis, Visibility::Inherited);
        assert_eq!(parser.cxx_qt_data.namespace, None);
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 1);
    }

    #[test]
    fn test_parser_invalid() {
        assert_parse_errors! {
            Parser::from =>

            {
                // Non-string namespace
                #[cxx_qt::bridge]
                mod ffi {
                    extern "Rust" {
                        #[namespace = 1]
                        type MyObject = super::MyObjectRust;
                    }
                }
            }
            {
                // No cxx_qt bridge on module
                mod ffi {
                    extern "Rust" {
                        fn test();
                    }
                }
            }
            {
                // Cxx_file_stem is deprecated
                #[cxx_qt::bridge(cxx_file_stem = "stem")]
                mod ffi {
                    extern "Rust" {
                        fn test();
                    }
                }
            }
        }
    }

    #[test]
    fn test_cxx_qobject_namespace() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "bridge_namespace")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObjectA = super::MyObjectARust;

                    #[qobject]
                    #[namespace = "type_namespace"]
                    type MyObjectB = super::MyObjectBRust;
                }

                #[namespace = "extern_namespace"]
                extern "RustQt" {
                    #[qobject]
                    type MyObjectC = super::MyObjectCRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();
        assert_eq!(parser.type_names.num_types(), 22);
        assert_eq!(
            parser
                .type_names
                .namespace(&format_ident!("MyObjectA"))
                .unwrap()
                .unwrap(),
            "bridge_namespace"
        );
        assert_eq!(
            parser
                .type_names
                .namespace(&format_ident!("MyObjectB"))
                .unwrap()
                .unwrap(),
            "type_namespace"
        );
        assert_eq!(
            parser
                .type_names
                .namespace(&format_ident!("MyObjectC"))
                .unwrap()
                .unwrap(),
            "extern_namespace"
        );

        assert_eq!(
            parser
                .type_names
                .namespace(&format_ident!("MyObjectARust"))
                .unwrap(),
            None
        );
        assert_eq!(
            parser
                .type_names
                .namespace(&format_ident!("MyObjectBRust"))
                .unwrap(),
            None
        );
        assert_eq!(
            parser
                .type_names
                .namespace(&format_ident!("MyObjectCRust"))
                .unwrap(),
            None
        );
    }
}
