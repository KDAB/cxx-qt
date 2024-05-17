// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::parser::signals::ParsedSignal;
use crate::{generator::naming::CombinedIdent, naming::Name};
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::{Ident, Result};

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

pub struct QSignalHelperName {
    pub connect_name: CombinedIdent,
    pub function_call: Ident,
    pub function_drop: Ident,
    pub handler_alias: Ident,
    pub handler_alias_namespaced: String,
    pub namespace: String,
    pub struct_closure: Ident,
    pub struct_param: Ident,
}

impl QSignalHelperName {
    pub fn new(idents: &QSignalName, qobject_name: &Name) -> Result<Self> {
        let signal_ident = &idents.name.cpp;
        let qobject_ident = qobject_name.rust_unqualified().to_string();
        let handler_alias = format_ident!("{qobject_ident}CxxQtSignalHandler{signal_ident}");
        let namespace = {
            // This namespace will take the form of:
            // qobject_namespace::rust::cxxqtgen1
            //
            // We experimented with using rust::cxxqtgen1::qobject_namespace.
            // However, this currently doesn't work, as we can't fully-qualify all C++ access.
            // Therefore when refering to the QObject type (e.g. qobject_namespace::QObject),
            // It would fail, as it would look up in this helper namespace, instead of the actual
            // qobject_namespace.
            //
            // See the comment on TypeNames::cxx_qualified for why fully qualifying is
            // unfortunately not possible.
            let qobject_namespace = qobject_name.namespace();
            let namespace: Vec<_> = qobject_namespace
                .into_iter()
                .chain(vec!["rust::cxxqtgen1"])
                .collect();

            namespace.join("::")
        };

        // TODO: in the future we might improve the naming of the methods
        // to avoid collisions (maybe use a separator similar to how CXX uses $?)
        Ok(Self {
            connect_name: CombinedIdent {
                cpp: format_ident!("{}_{}", qobject_ident, idents.connect_name.cpp),
                rust: format_ident!("{}_{}", qobject_ident, idents.connect_name.rust),
            },
            function_drop: format_ident!("drop_{qobject_ident}_signal_handler_{signal_ident}"),
            function_call: format_ident!("call_{qobject_ident}_signal_handler_{signal_ident}"),
            handler_alias_namespaced: format!("::{namespace}::{handler_alias}"),
            struct_closure: format_ident!("{qobject_ident}CxxQtSignalClosure{signal_ident}"),
            struct_param: format_ident!("{qobject_ident}CxxQtSignalParams{signal_ident}"),
            namespace,
            handler_alias,
        })
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
