// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod qobject;

use qobject::ParsedCxxQtData;
use syn::{token::Brace, ItemMod, Result};

/// A struct representing a module block with CXX-Qt relevant [syn::Item]'s
/// parsed into ParsedCxxQtData, to be used later to generate Rust & C++ code.
///
/// [syn::Item]'s that are not handled specially by CXX-Qt are passed through for CXX to process.
pub struct Parser {
    /// The module which unknown (eg CXX) blocks are stored into
    pub passthrough_module: ItemMod,
    /// Any CXX-Qt data that needs generation later
    pub cxx_qt_data: ParsedCxxQtData,
}

impl Parser {
    /// Constructs a Parser object from a given [syn::ItemMod] block
    pub fn from(mut module: ItemMod) -> Result<Self> {
        let mut cxx_qt_data = ParsedCxxQtData::default();
        let mut others = vec![];

        // Check that there are items in the module
        if let Some(mut items) = module.content {
            // Find any QObject structs
            cxx_qt_data.find_qobject_keys(&items.1)?;

            // Loop through any qobjects that were found
            if !cxx_qt_data.qobjects.is_empty() {
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::ItemMod;

    #[test]
    fn test_parser_from_empty_module() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge]
            mod ffi {}
        });
        let parser = Parser::from(module.clone());
        assert!(parser.is_ok());
        let parser = parser.unwrap();
        assert_eq!(parser.passthrough_module, module);
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 0);
    }

    #[test]
    fn test_parser_from_cxx_items() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "Rust" {
                    fn test();
                }
            }
        });
        let parser = Parser::from(module.clone());
        assert!(parser.is_ok());
        let parser = parser.unwrap();
        assert_eq!(parser.passthrough_module, module);
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 0);
    }

    #[test]
    fn test_parser_from_cxx_qt_items() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge]
            mod ffi {
                struct RustObj;

                #[cxx_qt::signals(RustObj)]
                enum MySignals {
                    Ready,
                }
            }
        });
        let parser = Parser::from(module.clone());
        assert!(parser.is_ok());
        let parser = parser.unwrap();

        assert_ne!(parser.passthrough_module, module);

        assert_eq!(parser.passthrough_module.attrs.len(), 1);
        assert_eq!(parser.passthrough_module.ident, "ffi");
        assert_eq!(parser.passthrough_module.content.unwrap().1.len(), 0);
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 1);
    }

    #[test]
    fn test_parser_from_cxx_and_cxx_qt_items() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge]
            mod ffi {
                struct RustObj;

                #[cxx_qt::signals(RustObj)]
                enum MySignals {
                    Ready,
                }

                extern "Rust" {
                    fn test();
                }
            }
        });
        let parser = Parser::from(module.clone());
        assert!(parser.is_ok());
        let parser = parser.unwrap();

        assert_ne!(parser.passthrough_module, module);

        assert_eq!(parser.passthrough_module.attrs.len(), 1);
        assert_eq!(parser.passthrough_module.ident, "ffi");
        assert_eq!(parser.passthrough_module.content.unwrap().1.len(), 1);
        assert_eq!(parser.cxx_qt_data.qobjects.len(), 1);
    }

    #[test]
    fn test_parser_from_error() {
        let module: ItemMod = tokens_to_syn(quote! {
            #[cxx_qt::bridge]
            mod ffi {
                struct RustObj;

                #[cxx_qt::signals(UnknownObj)]
                enum MySignals {
                    Ready,
                }
            }
        });
        let parser = Parser::from(module);
        assert!(parser.is_err());
    }
}
