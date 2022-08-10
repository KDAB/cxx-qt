// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::parameter::ParsedFunctionParameter;
use crate::syntax::fields::fields_named_to_ident_type;
use syn::{Ident, ItemEnum, Result, Variant};

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
    pub fn from(variant: &Variant) -> Result<Self> {
        // Read the fields into parameter blocks
        let parameters = fields_named_to_ident_type(&variant.fields)?
            .into_iter()
            .map(|(ident, ty)| ParsedFunctionParameter { ident, ty })
            .collect();

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
            .iter()
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

    use crate::parser::tests::f64_type;
    use crate::syntax::path::path_compare_str;
    use crate::tests::tokens_to_syn;
    use quote::quote;

    #[test]
    fn test_parsed_signals_from_empty() {
        let e: ItemEnum = tokens_to_syn(quote! {
            #[cxx_qt::signals(MyObject)]
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
            #[cxx_qt::signals(MyObject)]
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
            #[cxx_qt::signals(MyObject)]
            enum MySignals {
                Ready,
                PointChanged { x: f64, y: f64 },
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
        assert_eq!(signals.signals[1].parameters[0].ident, "x");
        assert_eq!(signals.signals[1].parameters[0].ty, f64_type());
        assert_eq!(signals.signals[1].parameters[1].ident, "y");
        assert_eq!(signals.signals[1].parameters[1].ty, f64_type());
    }

    #[test]
    fn test_parsed_signals_from_unnamed() {
        let e: ItemEnum = tokens_to_syn(quote! {
            #[cxx_qt::signals(MyObject)]
            enum MySignals {
                Ready,
                PointChanged(f64, f64),
            }
        });
        let signals = ParsedSignalsEnum::from(&e, 0);
        assert!(signals.is_err());
    }
}
