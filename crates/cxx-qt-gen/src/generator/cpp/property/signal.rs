// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::ForeignItemFn;

use crate::{
    generator::naming::{property::QPropertyName, qobject::QObjectName},
    parser::signals::ParsedSignal,
};

pub fn generate(idents: &QPropertyName, qobject_idents: &QObjectName) -> ParsedSignal {
    // We build our signal in the generation phase as we need to use the naming
    // structs to build the signal name
    let cpp_class_rust = &qobject_idents.name.rust_unqualified();
    let notify_cpp = &idents.notify.cpp;
    let notify_rust_str = idents.notify.rust.to_string();
    let method: ForeignItemFn = syn::parse_quote! {
        #[doc = "Notify for the Q_PROPERTY"]
        #[rust_name = #notify_rust_str]
        fn #notify_cpp(self: Pin<&mut #cpp_class_rust>);
    };
    ParsedSignal::from_property_method(
        method,
        idents.notify.clone(),
        qobject_idents.name.rust_unqualified().clone(),
    )
}
