// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{naming::Name, parser::signals::ParsedSignal};
use quote::format_ident;
use syn::Ident;

/// Names for parts of a Q_SIGNAL
pub struct QSignalNames {
    pub name: Name,
    pub connect_name: Name,
    pub on_name: Ident,
}

impl From<&ParsedSignal> for QSignalNames {
    fn from(signal: &ParsedSignal) -> Self {
        Self {
            name: signal.name.clone(),
            connect_name: connect_name_from_signal(&signal.name),
            on_name: on_from_signal(signal.name.rust_unqualified()),
        }
    }
}

fn connect_name_from_signal(name: &Name) -> Name {
    name.clone()
        .with_rust_name(format_ident!("connect_{}", name.rust_unqualified()))
        // Use signalConnect instead of onSignal here so that we don't
        // create a C++ name that is similar to the QML naming scheme for signals
        .with_cxx_name(format!("{}Connect", name.cxx_unqualified()))
}

fn on_from_signal(ident: &Ident) -> Ident {
    format_ident!("on_{}", ident.to_string())
}

pub struct QSignalHelperNames {
    pub connect_name: Name,
    pub function_call: Ident,
    pub function_drop: Ident,
    pub handler_alias: Ident,
    pub handler_alias_namespaced: String,
    pub namespace: String,
    pub struct_closure: Ident,
    pub struct_param: Ident,
}

impl QSignalHelperNames {
    pub fn new(idents: &QSignalNames, qobject_name: &Name) -> Self {
        let signal_ident = &idents.name.cxx_unqualified();
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

        let connect_name = Name::new(format_ident!(
            "{}_{}",
            qobject_name.rust_unqualified(),
            idents.connect_name.rust_unqualified()
        ))
        .with_cxx_name(format!(
            "{}_{}",
            qobject_name.cxx_unqualified(),
            idents.connect_name.cxx_unqualified()
        ));

        // TODO: in the future we might improve the naming of the methods
        // to avoid collisions (maybe use a separator similar to how CXX uses $?)
        Self {
            connect_name,
            function_drop: format_ident!("drop_{qobject_ident}_signal_handler_{signal_ident}"),
            function_call: format_ident!("call_{qobject_ident}_signal_handler_{signal_ident}"),
            handler_alias_namespaced: format!("::{namespace}::{handler_alias}"),
            struct_closure: format_ident!("{qobject_ident}CxxQtSignalClosure{signal_ident}"),
            struct_param: format_ident!("{qobject_ident}CxxQtSignalParams{signal_ident}"),
            namespace,
            handler_alias,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::parse_quote;

    #[test]
    fn test_parsed_signal() {
        let method = parse_quote! {
            #[cxx_name = "dataChanged"]
            fn data_changed(self: Pin<&mut MyObject>);
        };
        let qsignal = ParsedSignal::mock(&method);

        let names = QSignalNames::from(&qsignal);
        assert_eq!(names.name.cxx_unqualified(), "dataChanged");
        assert_eq!(
            names.name.rust_unqualified(),
            &format_ident!("data_changed")
        );
        assert_eq!(names.connect_name.cxx_unqualified(), "dataChangedConnect");
        assert_eq!(
            names.connect_name.rust_unqualified(),
            &format_ident!("connect_data_changed")
        );
        assert_eq!(names.on_name, format_ident!("on_data_changed"));
    }

    #[test]
    fn test_parsed_signal_existing_cxx_name() {
        let method = parse_quote! {
            #[cxx_name = "baseName"]
            fn existing_signal(self: Pin<&mut MyObject>);
        };
        let qsignal = ParsedSignal::mock(&method);

        let names = QSignalNames::from(&qsignal);
        assert_eq!(names.name.cxx_unqualified(), "baseName");
        assert_eq!(
            names.name.rust_unqualified(),
            &format_ident!("existing_signal")
        );
        assert_eq!(names.connect_name.cxx_unqualified(), "baseNameConnect");
        assert_eq!(
            names.connect_name.rust_unqualified(),
            &format_ident!("connect_existing_signal")
        );
        assert_eq!(names.on_name, format_ident!("on_existing_signal"));
    }
}
