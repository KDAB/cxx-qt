// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::parameter::ParsedFunctionParameter;
use crate::syntax::{
    attribute::{attribute_find_path, attribute_tokens_to_value},
    fields::fields_to_named_fields_mut,
};
use syn::{Ident, ItemEnum, LitStr, Result, Variant};

/// Describes an individual Signal
pub struct ParsedSignal {
    /// The name of the signal
    pub ident: Ident,
    /// The parameters of the signal
    pub parameters: Vec<ParsedFunctionParameter>,
    // TODO: later this will describe if the signal has an attribute
    // stating that the signal exists in the base class
}

impl ParsedSignal {
    pub fn from(variant: &mut Variant) -> Result<Self> {
        // Read the fields into parameter blocks
        let parameters = fields_to_named_fields_mut(&mut variant.fields)?
            .into_iter()
            .map(|field| {
                // Parse any cxx_type for the signal
                let cxx_type = if let Some(index) = attribute_find_path(&field.attrs, &["cxx_type"])
                {
                    let str = attribute_tokens_to_value::<LitStr>(&field.attrs[index])?.value();
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
            // TODO: later we might have attributes on the signal
            // to state if they exist in the base class, these could be
            // extracted and stored in ParsedSignal here
            ident: variant.ident.clone(),
            parameters,
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
    use super::*;

    use crate::syntax::path::path_compare_str;
    use crate::tests::{rust::common::f64_type, utils::tokens_to_syn};
    use quote::quote;

    #[test]
    fn test_parsed_signals_from_empty() {
        let e: ItemEnum = tokens_to_syn(quote! {
            #[cxx_qt::qsignals(MyObject)]
            enum MySignals {}
        });
        let signals = ParsedSignalsEnum::from(&e, 0).unwrap();
        assert_eq!(signals.ident, "MySignals");
        assert_eq!(signals.item.attrs.len(), 0);
        assert_eq!(signals.signals.len(), 0);
    }

    #[test]
    fn test_parsed_signals_from_empty_attrs() {
        let e: ItemEnum = tokens_to_syn(quote! {
            #[before]
            #[cxx_qt::qsignals(MyObject)]
            #[after]
            enum MySignals {}
        });
        let signals = ParsedSignalsEnum::from(&e, 1).unwrap();
        assert_eq!(signals.ident, "MySignals");
        assert_eq!(signals.item.attrs.len(), 2);
        assert!(path_compare_str(&signals.item.attrs[0].path, &["before"]));
        assert!(path_compare_str(&signals.item.attrs[1].path, &["after"]));
        assert_eq!(signals.signals.len(), 0);
    }

    #[test]
    fn test_parsed_signals_from_named() {
        let e: ItemEnum = tokens_to_syn(quote! {
            #[cxx_qt::qsignals(MyObject)]
            enum MySignals {
                Ready,
                PointChanged {
                    x: f64,
                    #[cxx_type = "f32"]
                    y: f64
                },
            }
        });
        let signals = ParsedSignalsEnum::from(&e, 0).unwrap();
        assert_eq!(signals.ident, "MySignals");
        assert_eq!(signals.item.attrs.len(), 0);
        assert_eq!(signals.signals.len(), 2);
        assert_eq!(signals.signals[0].ident, "Ready");
        assert_eq!(signals.signals[0].parameters.len(), 0);
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
    }

    #[test]
    fn test_parsed_signals_from_unnamed() {
        let e: ItemEnum = tokens_to_syn(quote! {
            #[cxx_qt::qsignals(MyObject)]
            enum MySignals {
                Ready,
                PointChanged(f64, f64),
            }
        });
        let signals = ParsedSignalsEnum::from(&e, 0);
        assert!(signals.is_err());
    }
}
