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
    pub enum_name: Ident,
    pub name: CombinedIdent,
    pub queued_name: CombinedIdent,
}

impl From<&ParsedSignal> for QSignalName {
    fn from(signal: &ParsedSignal) -> Self {
        Self::from(&signal.ident)
    }
}

impl From<&Ident> for QSignalName {
    fn from(ident: &Ident) -> Self {
        Self {
            enum_name: ident.clone(),
            name: name_from_ident(ident),
            queued_name: queued_name_from_ident(ident),
        }
    }
}

/// For a given signal ident generate the Rust and C++ names
fn name_from_ident(ident: &Ident) -> CombinedIdent {
    CombinedIdent {
        cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
        // Note that signal names are in camel case so we need to convert to snake and can't clone
        rust: format_ident!("{}", ident.to_string().to_case(Case::Snake)),
    }
}

/// For a given signal ident generate the Rust and C++ queued name
fn queued_name_from_ident(ident: &Ident) -> CombinedIdent {
    // Note that signal names are in camel case so we need to convert to snake first
    let ident = format_ident!("emit_{}", ident.to_string().to_case(Case::Snake));
    CombinedIdent {
        cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
        rust: ident,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_property() {
        let qsignal = ParsedSignal {
            ident: format_ident!("DataChanged"),
            parameters: vec![],
        };

        let names = QSignalName::from(&qsignal);
        assert_eq!(names.enum_name, format_ident!("DataChanged"));
        assert_eq!(names.name.cpp, format_ident!("dataChanged"));
        assert_eq!(names.name.rust, format_ident!("data_changed"));
        assert_eq!(names.queued_name.cpp, format_ident!("emitDataChanged"));
        assert_eq!(names.queued_name.rust, format_ident!("emit_data_changed"));
    }
}
