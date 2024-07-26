// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod constructor;
pub mod cxxqtdata;
pub mod externcxxqt;
pub mod inherit;
pub mod method;
pub mod parameter;
pub mod property;
pub mod qenum;
pub mod qnamespace;
pub mod qobject;
pub mod signals;

use crate::{
    // Used for error handling when resolving the namespace of the qenum.
    naming::TypeNames,
    syntax::{attribute::attribute_take_path, expr::expr_to_string},
};
use cxxqtdata::ParsedCxxQtData;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Brace, Error, Ident, Item, ItemMod, Meta,
    Result, Token,
};

/// A struct representing a module block with CXX-Qt relevant [Item]'s
/// parsed into ParsedCxxQtData, to be used later to generate Rust & C++ code.
///
/// [Item]'s that are not handled specially by CXX-Qt are passed through for CXX to process.
pub struct Parser {
    /// The module which unknown (e.g. CXX) blocks are stored into
    pub(crate) passthrough_module: ItemMod,
    /// Any CXX-Qt data that needs generation later
    pub(crate) cxx_qt_data: ParsedCxxQtData,
    /// all type names that were found in this module, including CXX types
    pub(crate) type_names: TypeNames,
    /// The stem of the file that the CXX headers for this module will be generated into
    pub cxx_file_stem: String,
}

impl Parser {
    fn parse_mod_attributes(module: &mut ItemMod) -> Result<(Option<String>, String)> {
        let mut namespace = None;
        let mut cxx_file_stem = module.ident.to_string();

        // Remove the cxx_qt::bridge attribute
        if let Some(attr) = attribute_take_path(&mut module.attrs, &["cxx_qt", "bridge"]) {
            // If we are no #[cxx_qt::bridge] but #[cxx_qt::bridge(A = B)] then process
            if !matches!(attr.meta, Meta::Path(_)) {
                let nested =
                    attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                for meta in nested {
                    match meta {
                        Meta::NameValue(name_value) => {
                            // Parse any namespace in the cxx_qt::bridge macro
                            if name_value.path.is_ident("namespace") {
                                namespace = Some(expr_to_string(&name_value.value)?);
                                // Parse any custom file stem
                            } else if name_value.path.is_ident("cxx_file_stem") {
                                cxx_file_stem = expr_to_string(&name_value.value)?;
                            }
                        }
                        _others => {}
                    }
                }
            }
        } else {
            return Err(Error::new(
                module.span(),
                "Tried to parse a module which doesn't have a cxx_qt::bridge attribute",
            ));
        }

        Ok((namespace, cxx_file_stem))
    }

    fn parse_module_contents(
        mut module: ItemMod,
        namespace: Option<String>,
    ) -> Result<(ParsedCxxQtData, ItemMod)> {
        let mut others = vec![];

        let mut cxx_qt_data = ParsedCxxQtData::new(module.ident.clone(), namespace);

        // Check that there are items in the module
        if let Some(mut items) = module.content {
            // Find any QObject structs
            cxx_qt_data.find_qobject_types(&items.1)?;

            // Loop through items and load into qobject or others and populate mappings
            for item in items.1.drain(..) {
                // Try to find any CXX-Qt items, if found add them to the relevant
                // qobject or extern C++Qt block. Otherwise, return them to be added to other
                if let Some(other) = cxx_qt_data.parse_cxx_qt_item(item)? {
                    // Unknown item so add to the other list
                    others.push(other);
                }
            }
        }

        // Create a new module using only items that are not CXX-Qt items
        module.content = Some((Brace::default(), others));
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

    /// Constructs a Parser object from a given [ItemMod] block
    pub fn from(mut module: ItemMod) -> Result<Self> {
        let (namespace, cxx_file_stem) = Self::parse_mod_attributes(&mut module)?;
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
            passthrough_module: module,
            type_names,
            cxx_qt_data,
            cxx_file_stem,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::structuring::Structures;
    use pretty_assertions::assert_eq;
    use quote::format_ident;
    use syn::{parse_quote, ItemMod, Type};

    /// Helper which returns a f64 as a [Type]
    pub fn f64_type() -> Type {
        parse_quote! { f64 }
    }

    // PROTOTYPING ONLY REMOVE ME
    #[test]
    fn test_debug_in_cxx_qt_data() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;

                    #[qobject]
                    type MyOtherObject = super::MyOtherObjectRust;
                }

                unsafe extern "RustQt" {
                    #[qinvokable]
                    fn test_fn(self: Pin<&mut MyObject>);

                    #[qinvokable]
                    fn test_fn_two(self: Pin<&mut MyObject>);

                    #[qinvokable]
                    fn test_fn_again(self: Pin<&mut MyOtherObject>);

                    #[qsignal]
                    fn ready(self: Pin<&mut MyOtherObject>);
                }

                extern "Rust" {
                    fn test();
                }
            }
        };
        let parser = Parser::from(module.clone()).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();
        for obj in structures.qobjects {
            println!(
                "[i] Object with name: {:?} \nAnd methods:     {:?}\n",
                obj.declaration.name.clone(),
                obj.methods.keys()
            );
        }
    }

    #[test]
    fn test_parser_from_empty_module() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {}
        };
        let parser = Parser::from(module).unwrap();
        let expected_module: ItemMod = parse_quote! {
            mod ffi {}
        };
        assert_eq!(parser.passthrough_module, expected_module);
        assert_eq!(parser.cxx_qt_data.namespace, None);
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 0);
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
        let expected_module: ItemMod = parse_quote! {
            mod ffi {
                extern "Rust" {
                    fn test();
                }
            }
        };
        assert_eq!(parser.passthrough_module, expected_module);
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

        assert_ne!(parser.passthrough_module, module);

        assert_eq!(parser.passthrough_module.attrs.len(), 0);
        assert_eq!(parser.passthrough_module.ident, "ffi");
        assert_eq!(parser.passthrough_module.content.unwrap().1.len(), 0);
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

        assert_ne!(parser.passthrough_module, module);

        assert_eq!(parser.passthrough_module.attrs.len(), 0);
        assert_eq!(parser.passthrough_module.ident, "ffi");
        assert_eq!(parser.passthrough_module.content.unwrap().1.len(), 1);
        assert_eq!(parser.cxx_qt_data.namespace, None);
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 1);
    }

    #[test]
    fn test_parser_from_error() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }

                unsafe extern "RustQt" {
                    #[qsignal]
                    fn ready(self: Pin<&mut UnknownObject>);
                }
            }
        };
        let parser = Parser::from(module);
        assert!(parser.is_err());
    }

    #[test]
    fn test_parser_from_error_no_attribute() {
        let module: ItemMod = parse_quote! {
            mod ffi {
                extern "Rust" {
                    fn test();
                }
            }
        };
        let parser = Parser::from(module);
        assert!(parser.is_err());
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
