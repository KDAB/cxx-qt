// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::parameter::ParsedFunctionParameter;
use crate::syntax::expr::expr_to_string;
use crate::syntax::{attribute::attribute_find_path, fields::fields_to_named_fields_mut};
use syn::{Ident, ItemEnum, Result, Variant};

/// Describes an individual Signal
pub struct ParsedSignal {
    /// The name of the signal
    pub ident: Ident,
    /// The parameters of the signal
    pub parameters: Vec<ParsedFunctionParameter>,
    /// The name of the signal in C++
    pub cxx_name: Option<String>,
    /// If the signal is defined in the base class
    pub inherit: bool,
}

impl ParsedSignal {
    pub fn from(variant: &mut Variant) -> Result<Self> {
        // Find cxx_name and inherit
        let inherit = if let Some(index) = attribute_find_path(&variant.attrs, &["inherit"]) {
            // Remove the attribute from the original enum
            // so that it doesn't end up in the Rust generation
            variant.attrs.remove(index);
            true
        } else {
            false
        };
        let cxx_name = if let Some(index) = attribute_find_path(&variant.attrs, &["cxx_name"]) {
            let str = expr_to_string(&variant.attrs[index].meta.require_name_value()?.value)?;
            // Remove the attribute from the original enum
            // so that it doesn't end up in the Rust generation
            variant.attrs.remove(index);
            Some(str)
        } else {
            None
        };

        // Read the fields into parameter blocks
        let parameters = fields_to_named_fields_mut(&mut variant.fields)?
            .into_iter()
            .map(|field| {
                // Parse any cxx_type for the signal
                let cxx_type = if let Some(index) = attribute_find_path(&field.attrs, &["cxx_type"])
                {
                    let str = expr_to_string(&field.attrs[index].meta.require_name_value()?.value)?;
                    // Remove the attribute from the original enum
                    // so that it doesn't end up in the Rust generation
                    field.attrs.remove(index);
                    Some(str)
                } else {
                    None
                };

                Ok(ParsedFunctionParameter {
                    ident: field.ident.clone().unwrap(),
                    ty: field.ty.clone(),
                    cxx_type,
                })
            })
            .collect::<Result<Vec<ParsedFunctionParameter>>>()?;

        Ok(ParsedSignal {
            ident: variant.ident.clone(),
            parameters,
            cxx_name,
            inherit,
        })
    }
}

/// Describes a Signals enum for a QObject
pub struct ParsedSignalsEnum {
    /// The name of the signals enum
    pub ident: Ident,
    /// The original enum for the signals
    pub item: ItemEnum,
    /// A list of the signals defined in the enum
    pub signals: Vec<ParsedSignal>,
}

impl ParsedSignalsEnum {
    /// Constructs a ParsedSignals object from a given [syn::ItemEnum] block
    pub fn from(item: &ItemEnum, attr_index: usize) -> Result<Self> {
        // Remove the attribute index as we have processed it
        let mut item = item.clone();
        item.attrs.remove(attr_index);

        let signals = item
            .variants
            // Note we use mut here so that any cxx_type attributes can be removed
            .iter_mut()
            .map(ParsedSignal::from)
            .collect::<Result<Vec<ParsedSignal>>>()?;

        Ok(Self {
            ident: item.ident.clone(),
            item,
            signals,
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    use crate::parser::tests::f64_type;
    use crate::syntax::path::path_compare_str;

    #[test]
    fn test_parsed_signals_from_empty() {
        let e: ItemEnum = parse_quote! {
            #[cxx_qt::qsignals(MyObject)]
            enum MySignals {}
        };
        let signals = ParsedSignalsEnum::from(&e, 0).unwrap();
        assert_eq!(signals.ident, "MySignals");
        assert_eq!(signals.item.attrs.len(), 0);
        assert_eq!(signals.signals.len(), 0);
    }

    #[test]
    fn test_parsed_signals_from_empty_attrs() {
        let e: ItemEnum = parse_quote! {
            #[before]
            #[cxx_qt::qsignals(MyObject)]
            #[after]
            enum MySignals {}
        };
        let signals = ParsedSignalsEnum::from(&e, 1).unwrap();
        assert_eq!(signals.ident, "MySignals");
        assert_eq!(signals.item.attrs.len(), 2);
        assert!(path_compare_str(
            signals.item.attrs[0].meta.path(),
            &["before"]
        ));
        assert!(path_compare_str(
            signals.item.attrs[1].meta.path(),
            &["after"]
        ));
        assert_eq!(signals.signals.len(), 0);
    }

    #[test]
    fn test_parsed_signals_from_named() {
        let e: ItemEnum = parse_quote! {
            #[cxx_qt::qsignals(MyObject)]
            enum MySignals {
                Ready,
                PointChanged {
                    x: f64,
                    #[cxx_type = "f32"]
                    y: f64
                },
                #[cxx_name = "baseName"]
                #[inherit]
                ExistingSignal,
            }
        };
        let signals = ParsedSignalsEnum::from(&e, 0).unwrap();
        assert_eq!(signals.ident, "MySignals");
        assert_eq!(signals.item.attrs.len(), 0);
        assert_eq!(signals.signals.len(), 3);
        assert!(!signals.signals[0].inherit);
        assert!(signals.signals[0].cxx_name.is_none());
        assert_eq!(signals.signals[0].ident, "Ready");
        assert_eq!(signals.signals[0].parameters.len(), 0);
        assert!(!signals.signals[1].inherit);
        assert!(signals.signals[1].cxx_name.is_none());
        assert_eq!(signals.signals[1].ident, "PointChanged");
        assert_eq!(signals.signals[1].parameters.len(), 2);
        assert!(signals.signals[1].parameters[0].cxx_type.is_none());
        assert_eq!(signals.signals[1].parameters[0].ident, "x");
        assert_eq!(signals.signals[1].parameters[0].ty, f64_type());
        assert_eq!(
            signals.signals[1].parameters[1].cxx_type.as_ref().unwrap(),
            "f32"
        );
        assert_eq!(signals.signals[1].parameters[1].ident, "y");
        assert_eq!(signals.signals[1].parameters[1].ty, f64_type());
        assert!(signals.signals[2].inherit);
        assert!(signals.signals[2].cxx_name.is_some());
        assert_eq!(signals.signals[2].cxx_name.as_ref().unwrap(), "baseName");
        assert_eq!(signals.signals[2].ident, "ExistingSignal");
        assert_eq!(signals.signals[2].parameters.len(), 0);
    }

    #[test]
    fn test_parsed_signals_from_unnamed() {
        let e: ItemEnum = parse_quote! {
            #[cxx_qt::qsignals(MyObject)]
            enum MySignals {
                Ready,
                PointChanged(f64, f64),
            }
        };
        let signals = ParsedSignalsEnum::from(&e, 0);
        assert!(signals.is_err());
    }
}
