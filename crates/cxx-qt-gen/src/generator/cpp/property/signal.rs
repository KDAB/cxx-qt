// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::ForeignItemFn;

use crate::{
    generator::naming::{
        property::{NameState, QPropertyNames},
        qobject::QObjectNames,
    },
    parser::signals::ParsedSignal,
};

pub fn generate(idents: &QPropertyNames, qobject_idents: &QObjectNames) -> Option<ParsedSignal> {
    // We build our signal in the generation phase as we need to use the naming
    // structs to build the signal name
    if let Some(NameState::Auto(notify)) = &idents.notify {
        let cpp_class_rust = &qobject_idents.name.rust_unqualified();
        let notify_cpp = notify.cxx_unqualified();
        let notify_rust = notify.rust_unqualified();
        let method: ForeignItemFn = syn::parse_quote! {
            #[doc = "Notify for the Q_PROPERTY"]
            #[cxx_name = #notify_cpp]
            fn #notify_rust(self: Pin<&mut #cpp_class_rust>);
        };

        Some(ParsedSignal::from_property_method(
            method,
            notify.clone(),
            qobject_idents.name.rust_unqualified().clone(),
        ))
    } else {
        None
    }
}
