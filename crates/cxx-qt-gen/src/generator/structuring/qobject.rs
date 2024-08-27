// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::naming::Name;
use crate::parser::constructor::Constructor;
use crate::parser::inherit::ParsedInheritedMethod;
use crate::parser::method::ParsedMethod;
use crate::parser::signals::ParsedSignal;
use crate::parser::{qenum::ParsedQEnum, qobject::ParsedQObject};
use proc_macro2::Ident;
use syn::{Error, Result};

/// The StructuredQObject contains the parsed QObject and all members.
/// This includes QEnums, QSignals, methods, etc.
pub struct StructuredQObject<'a> {
    pub declaration: &'a ParsedQObject,
    pub qenums: Vec<&'a ParsedQEnum>,
    pub methods: Vec<&'a ParsedMethod>,
    pub inherited_methods: Vec<&'a ParsedInheritedMethod>,
    pub signals: Vec<&'a ParsedSignal>,
    pub constructors: Vec<&'a Constructor>,
    pub locking: bool,
    pub threading: bool,
}

impl<'a> StructuredQObject<'a> {
    pub fn has_qobject_name(&self, ident: &Ident) -> bool {
        self.declaration.name.rust_unqualified() == ident
    }

    /// Creates a [StructuredQObject] from a [ParsedQObject] with empty enum, method and signal collections
    pub fn from_qobject(qobject: &'a ParsedQObject) -> Self {
        Self {
            declaration: qobject,
            qenums: vec![],
            methods: vec![],
            inherited_methods: vec![],
            signals: vec![],
            constructors: vec![],
            locking: true,
            threading: false,
        }
    }

    pub fn method_lookup(&self, id: &Ident) -> Result<Name> {
        // TODO account for inherited methods too since those are in a different vector
        self.methods
            .iter()
            .map(|method| &method.name)
            .find(|name| name.rust_unqualified() == id)
            .cloned()
            .ok_or_else(|| Error::new_spanned(id, format!("Method with name '{id}' not found!")))
    }

    pub fn signal_lookup(&self, id: &Ident) -> Result<Name> {
        self.signals
            .iter()
            .map(|signal| &signal.name)
            .find(|name| name.rust_unqualified() == id)
            .cloned()
            .ok_or_else(|| Error::new_spanned(id, format!("Signal with name '{id}' not found!")))
    }

    #[cfg(test)]
    pub fn mock(obj: &'a ParsedQObject) -> Self {
        Self::from_qobject(obj)
    }
}
