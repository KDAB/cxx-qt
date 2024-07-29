// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::method::ParsedMethod;
use crate::parser::signals::ParsedSignal;
use crate::parser::{qenum::ParsedQEnum, qobject::ParsedQObject};
use proc_macro2::Ident;

/// The StructuredQObject contains the parsed QObject and all members.
/// This includes QEnums, QSignals, methods, etc.
pub struct StructuredQObject<'a> {
    pub declaration: &'a ParsedQObject,
    pub qenums: Vec<&'a ParsedQEnum>,
    pub methods: Vec<&'a ParsedMethod>,
    pub signals: Vec<&'a ParsedSignal>,
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
            signals: vec![],
        }
    }
}
