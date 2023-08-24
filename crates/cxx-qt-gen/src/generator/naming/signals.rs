// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::generator::naming::CombinedIdent;
use crate::parser::signals::ParsedSignal;
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

/// Names for parts of a Q_SIGNAL
pub struct QSignalName {
    pub name: CombinedIdent,
    pub connect_name: CombinedIdent,
    pub on_name: Ident,
}

impl From<&ParsedSignal> for QSignalName {
    fn from(signal: &ParsedSignal) -> Self {
        Self {
            name: signal.ident.clone(),
            connect_name: CombinedIdent::connect_from_signal(&signal.ident),
            on_name: on_from_signal(&signal.ident.rust),
        }
    }
}

fn on_from_signal(ident: &Ident) -> Ident {
    format_ident!("on_{}", ident.to_string().to_case(Case::Snake))
}

impl CombinedIdent {
    fn connect_from_signal(ident: &CombinedIdent) -> Self {
        Self {
            // Use signalConnect instead of onSignal here so that we don't
            // create a C++ name that is similar to the QML naming scheme for signals
            cpp: format_ident!("{}Connect", ident.cpp.to_string().to_case(Case::Camel)),
            rust: format_ident!("connect_{}", ident.rust.to_string().to_case(Case::Snake)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::parse_quote;

    #[test]
    fn test_parsed_signal() {
        let qsignal = ParsedSignal {
            method: parse_quote! {
                fn data_changed(self: Pin<&mut MyObject>);
            },
            qobject_ident: format_ident!("MyObject"),
            mutable: true,
            parameters: vec![],
            ident: CombinedIdent {
                cpp: format_ident!("dataChanged"),
                rust: format_ident!("data_changed"),
            },
            safe: true,
            inherit: false,
            private: false,
        };

        let names = QSignalName::from(&qsignal);
        assert_eq!(names.name.cpp, format_ident!("dataChanged"));
        assert_eq!(names.name.rust, format_ident!("data_changed"));
        assert_eq!(names.connect_name.cpp, format_ident!("dataChangedConnect"));
        assert_eq!(
            names.connect_name.rust,
            format_ident!("connect_data_changed")
        );
        assert_eq!(names.on_name, format_ident!("on_data_changed"));
    }

    #[test]
    fn test_parsed_signal_existing_cxx_name() {
        let qsignal = ParsedSignal {
            method: parse_quote! {
                #[cxx_name = "baseName"]
                fn existing_signal(self: Pin<&mut MyObject>);
            },
            qobject_ident: format_ident!("MyObject"),
            mutable: true,
            parameters: vec![],
            ident: CombinedIdent {
                cpp: format_ident!("baseName"),
                rust: format_ident!("existing_signal"),
            },
            safe: true,
            inherit: false,
            private: false,
        };

        let names = QSignalName::from(&qsignal);
        assert_eq!(names.name.cpp, format_ident!("baseName"));
        assert_eq!(names.name.rust, format_ident!("existing_signal"));
        assert_eq!(names.connect_name.cpp, format_ident!("baseNameConnect"));
        assert_eq!(
            names.connect_name.rust,
            format_ident!("connect_existing_signal")
        );
        assert_eq!(names.on_name, format_ident!("on_existing_signal"));
    }
}
