// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod cxxqtdata;
pub mod invokable;
pub mod parameter;
pub mod property;
pub mod qobject;
pub mod signals;

use crate::syntax::attribute::{attribute_find_path, attribute_tokens_to_map, AttributeDefault};
use cxxqtdata::ParsedCxxQtData;
use syn::{spanned::Spanned, token::Brace, Error, Ident, ItemMod, LitStr, Result};

/// A struct representing a module block with CXX-Qt relevant [syn::Item]'s
/// parsed into ParsedCxxQtData, to be used later to generate Rust & C++ code.
///
/// [syn::Item]'s that are not handled specially by CXX-Qt are passed through for CXX to process.
pub struct Parser {
    /// The module which unknown (eg CXX) blocks are stored into
    pub passthrough_module: ItemMod,
    /// Any CXX-Qt data that needs generation later
    pub cxx_qt_data: ParsedCxxQtData,
    /// The stem of the file that the CXX headers for this module will be generated into
    pub cxx_file_stem: String,
}

impl Parser {
    /// Constructs a Parser object from a given [syn::ItemMod] block
    pub fn from(mut module: ItemMod) -> Result<Self> {
        let mut cxx_qt_data = ParsedCxxQtData::default();
        let mut others = vec![];
        let mut cxx_file_stem = module.ident.to_string();

        // Remove the cxx_qt::bridge attribute
        if let Some(index) = attribute_find_path(&module.attrs, &["cxx_qt", "bridge"]) {
            let attr_map = attribute_tokens_to_map::<Ident, LitStr>(
                &module.attrs[index],
                AttributeDefault::None,
            )?;
            // Parse any namespace in the cxx_qt::bridge macro
            cxx_qt_data.namespace =
                if let Some(lit_str) = attr_map.get(&quote::format_ident!("namespace")) {
                    lit_str.value()
                } else {
                    "".to_owned()
                };

            // Parse any custom file stem
            if let Some(stem) = attr_map.get(&quote::format_ident!("cxx_file_stem")) {
                cxx_file_stem = stem.value();
            }

            module.attrs.remove(index);
        } else {
            return Err(Error::new(
                module.span(),
                "Tried to parse a module which doesn't have a cxx_qt::bridge attribute",
            ));
        }

        // Check that there are items in the module
        if let Some(mut items) = module.content {
            // Find any QObject structs
            cxx_qt_data.find_qobject_structs(&items.1)?;

            // Loop through any qobjects that were found
            if !cxx_qt_data.qobjects.is_empty() {
                // Copy the namespace from the module into each QObject
                //
                // TODO: later we will read the namespace attribute from the qobject as well
                for qobject in cxx_qt_data.qobjects.values_mut() {
                    qobject.namespace = cxx_qt_data.namespace.clone();
                }

                for item in items.1.drain(..) {
                    // Try to find any CXX-Qt items, if found add them to the relevant
                    // qobject. Otherwise return them to be added to other
                    if let Some(other) = cxx_qt_data.parse_cxx_qt_item(item)? {
                        // Unknown item so add to the other list
                        others.push(other);
                    }
                }
            } else {
                // No qobjects found so pass everything through
                others.extend(items.1);
            }
        }

        // Create a new module using only items that are not CXX-Qt items
        module.content = Some((Brace::default(), others));

        // Return the successful Parser object
        Ok(Self {
            passthrough_module: module,
            cxx_qt_data,
            cxx_file_stem,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::utils::tokens_to_syn;
    use quote::quote;
    use syn::ItemMod;

    #[test]
    fn test_parser_from_empty_module() {
        let macro_bridge = crate::tests::rust::cxx_qt::macro_bridge();
        let module = crate::tests::rust::cxx::module_wrap(quote! {});
        let input: ItemMod = tokens_to_syn(quote! {
            #macro_bridge
            #module
        });
        let output: ItemMod = tokens_to_syn(quote! {
            #module
        });

        let parser = Parser::from(input).unwrap();
        assert_eq!(parser.passthrough_module, output);
        assert_eq!(parser.cxx_qt_data.namespace, "");
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 0);
    }

    #[test]
    fn test_parser_from_cxx_items() {
        let macro_bridge = crate::tests::rust::cxx_qt::macro_bridge();
        let extern_rust = crate::tests::rust::cxx::extern_rust();
        let module = crate::tests::rust::cxx::module_wrap(quote! {
            #extern_rust
        });
        let input: ItemMod = tokens_to_syn(quote! {
            #macro_bridge
            #module
        });
        let output: ItemMod = tokens_to_syn(quote! {
            #module
        });

        let parser = Parser::from(input).unwrap();
        assert_eq!(parser.passthrough_module, output);
        assert_eq!(parser.cxx_qt_data.namespace, "");
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 0);
    }

    #[test]
    fn test_parser_from_cxx_qt_items() {
        let macro_qobject = crate::tests::rust::cxx_qt::macro_qobject();
        let struct_qobject = crate::tests::rust::cxx_qt::struct_qobject();
        let macro_qsignals = crate::tests::rust::cxx_qt::macro_qsignals();
        let enum_qobjects = crate::tests::rust::cxx_qt::enum_qsignals();
        let module = crate::tests::rust::cxx::module_wrap(quote! {
            #macro_qobject
            #struct_qobject

            #macro_qsignals
            #enum_qobjects
        });
        let macro_bridge = crate::tests::rust::cxx_qt::macro_bridge_with_namespace();
        let input: ItemMod = tokens_to_syn(quote! {
            #macro_bridge
            #module
        });

        let parser = Parser::from(input.clone()).unwrap();
        assert_ne!(parser.passthrough_module, input);

        assert_eq!(parser.passthrough_module.attrs.len(), 0);
        assert_eq!(parser.passthrough_module.ident, "ffi");
        assert_eq!(parser.passthrough_module.content.unwrap().1.len(), 0);
        assert_eq!(parser.cxx_qt_data.namespace, "cxx_qt");
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 1);
    }

    #[test]
    fn test_parser_from_cxx_and_cxx_qt_items() {
        let macro_qobject = crate::tests::rust::cxx_qt::macro_qobject();
        let struct_qobject = crate::tests::rust::cxx_qt::struct_qobject();
        let macro_qsignals = crate::tests::rust::cxx_qt::macro_qsignals();
        let enum_qobjects = crate::tests::rust::cxx_qt::enum_qsignals();
        let cxx_extern_rust = crate::tests::rust::cxx::extern_rust();
        let module = crate::tests::rust::cxx::module_wrap(quote! {
            #macro_qobject
            #struct_qobject

            #macro_qsignals
            #enum_qobjects

            #cxx_extern_rust
        });
        let macro_bridge = crate::tests::rust::cxx_qt::macro_bridge();
        let input: ItemMod = tokens_to_syn(quote! {
            #macro_bridge
            #module
        });

        let parser = Parser::from(input.clone()).unwrap();
        assert_ne!(parser.passthrough_module, input);

        assert_eq!(parser.passthrough_module.attrs.len(), 0);
        assert_eq!(parser.passthrough_module.ident, "ffi");
        assert_eq!(parser.passthrough_module.content.unwrap().1.len(), 1);
        assert_eq!(parser.cxx_qt_data.namespace, "");
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 1);
    }

    #[test]
    fn test_parser_from_error() {
        let macro_qobject = crate::tests::rust::cxx_qt::macro_qobject();
        let struct_qobject = crate::tests::rust::cxx_qt::struct_qobject();
        let macro_qsignals_unknown_qobject =
            crate::tests::rust::cxx_qt::macro_qsignals_unknown_qobject();
        let enum_qobjects = crate::tests::rust::cxx_qt::enum_qsignals();
        let module = crate::tests::rust::cxx::module_wrap(quote! {
            #macro_qobject
            #struct_qobject

            #macro_qsignals_unknown_qobject
            #enum_qobjects
        });
        let macro_bridge = crate::tests::rust::cxx_qt::macro_bridge_with_namespace();
        let input: ItemMod = tokens_to_syn(quote! {
            #macro_bridge
            #module
        });

        let parser = Parser::from(input);
        assert!(parser.is_err());
    }

    #[test]
    fn test_parser_from_error_no_attribute() {
        let extern_rust = crate::tests::rust::cxx::extern_rust();
        let module = crate::tests::rust::cxx::module_wrap(quote! {
            #extern_rust
        });
        let input: ItemMod = tokens_to_syn(quote! {
            #module
        });

        let parser = Parser::from(input);
        assert!(parser.is_err());
    }
}
