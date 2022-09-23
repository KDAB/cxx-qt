// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::parameter::ParsedFunctionParameter;
use std::collections::HashSet;
use syn::ImplItemMethod;

/// Describes a C++ specifier for the Q_INVOKABLE
#[derive(Eq, Hash, PartialEq)]
pub enum ParsedQInvokableSpecifiers {
    Final,
    Override,
    Virtual,
}

/// Describes a single Q_INVOKABLE for a struct
pub struct ParsedQInvokable {
    /// The original [syn::ImplItemMethod] of the invokable
    pub method: ImplItemMethod,
    /// Whether this invokable is mutable
    pub mutable: bool,
    /// The name of the C++ type for the return type if one has been specified
    pub return_cxx_type: Option<String>,
    /// The parameters of the invokable
    pub parameters: Vec<ParsedFunctionParameter>,
    /// Any specifiers that declared on the invokable
    pub specifiers: HashSet<ParsedQInvokableSpecifiers>,
}
