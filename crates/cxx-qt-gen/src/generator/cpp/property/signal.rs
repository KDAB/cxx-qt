// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{spanned::Spanned, Error, ForeignItemFn};

use crate::{
    generator::naming::{property::QPropertyNames, qobject::QObjectNames},
    parser::signals::ParsedSignal,
};

pub fn generate(idents: &QPropertyNames, qobject_idents: &QObjectNames) -> ParsedSignal {
    // We build our signal in the generation phase as we need to use the naming
    // structs to build the signal name
    let cpp_class_rust = &qobject_idents.name.rust_unqualified();
    let notify_binding = &idents.notify.clone().expect("Notify was empty!");
    // TODO: modify the func to return result
    // let notify_binding = match &idents.notify {
    //     Some(notify) => notift,
    //     _ => return Err(Error::new(cxx_ty.span(), "Property did not include a notify field"))
    // };

    let notify_cpp = notify_binding.cxx_unqualified();
    let notify_rust = notify_binding.rust_unqualified();
    let method: ForeignItemFn = syn::parse_quote! {
        #[doc = "Notify for the Q_PROPERTY"]
        #[cxx_name = #notify_cpp]
        fn #notify_rust(self: Pin<&mut #cpp_class_rust>);
    };
    ParsedSignal::from_property_method(
        method,
        idents.notify.clone().expect("Notify was empty!"),
        qobject_idents.name.rust_unqualified().clone(),
    )
}
