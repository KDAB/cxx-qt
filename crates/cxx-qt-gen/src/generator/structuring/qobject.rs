// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::structuring::not_found_error;
use crate::naming::Name;
use crate::parser::constructor::Constructor;
use crate::parser::inherit::ParsedInheritedMethod;
use crate::parser::method::ParsedMethod;
use crate::parser::signals::ParsedSignal;
use crate::parser::{qenum::ParsedQEnum, qobject::ParsedQObject};
use proc_macro2::Ident;
use syn::Result;

/// The StructuredQObject contains the parsed QObject and all members.
/// This includes QEnums, QSignals, methods, etc.
pub struct StructuredQObject<'a> {
    pub declaration: &'a ParsedQObject,
    pub qenums: Vec<&'a ParsedQEnum>,
    pub methods: Vec<&'a ParsedMethod>,
    pub inherited_methods: Vec<&'a ParsedInheritedMethod>,
    pub signals: Vec<&'a ParsedSignal>,
    pub constructors: Vec<&'a Constructor>,
    pub pending_methods: Vec<Name>,
    pub pending_signals: Vec<Name>,
    pub threading: bool,
}

fn lookup<T>(invokables: &[T], id: &Ident, name_getter: impl Fn(&T) -> &Name) -> Option<Name> {
    invokables
        .iter()
        .map(name_getter)
        .find(|name| name.rust_unqualified() == id)
        .cloned()
}

impl<'a> StructuredQObject<'a> {
    pub fn has_qobject_name(&self, ident: &Ident) -> bool {
        self.declaration.name.rust_unqualified() == ident
    }

    /// Creates a [StructuredQObject] from a [ParsedQObject] with empty enum, method and signal collections
    pub fn from_qobject(qobject: &'a ParsedQObject) -> Self {
        let pending_methods = qobject
            .properties
            .iter()
            .flat_map(|property| property.pending_methods())
            .collect();

        let pending_signals = qobject
            .properties
            .iter()
            .flat_map(|property| property.pending_signals())
            .collect();

        Self {
            declaration: qobject,
            qenums: vec![],
            methods: vec![],
            inherited_methods: vec![],
            signals: vec![],
            constructors: vec![],
            pending_methods,
            pending_signals,
            threading: false,
        }
    }

    /// Returns the name of the method with the provided Rust ident if it exists, or an error
    pub fn method_lookup(&self, id: &Ident) -> Result<Name> {
        lookup(&self.methods, id, |method| &method.name)
            .or_else(|| lookup(&self.inherited_methods, id, |inherited| &inherited.name)) // fallback to searching inherited methods
            .or_else(|| lookup(&self.pending_methods, id, |pending| pending))
            .ok_or_else(|| not_found_error("Method", id))
    }

    /// Returns the name of the signal with the provided Rust ident if it exists, or an error
    pub fn signal_lookup(&self, id: &Ident) -> Result<Name> {
        lookup(&self.signals, id, |signal| &signal.name)
            .or_else(|| lookup(&self.pending_signals, id, |pending| pending))
            .ok_or_else(|| not_found_error("Signal", id))
    }

    #[cfg(test)]
    pub fn mock(obj: &'a ParsedQObject) -> Self {
        Self::from_qobject(obj)
    }
}
