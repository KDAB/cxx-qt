// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::method::ParsedMethod;
use crate::parser::signals::ParsedSignal;
use crate::parser::{qenum::ParsedQEnum, qobject::ParsedQObject};
use std::collections::HashMap;
use syn::Ident;

/// The StructuredQObject contains the parsed QObject and all members.
/// This includes QEnums, QSignals, methods, etc.
pub struct StructuredQObject<'a> {
    pub declaration: &'a ParsedQObject,
    pub qenums: Vec<&'a ParsedQEnum>,
    pub methods: HashMap<Ident, &'a ParsedMethod>,
    pub signals: HashMap<Ident, &'a ParsedSignal>,
}
