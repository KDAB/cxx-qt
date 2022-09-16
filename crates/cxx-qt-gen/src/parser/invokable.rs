// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::ImplItemMethod;

/// Describes a single Q_INVOKABLE for a struct
pub struct ParsedQInvokable {
    /// The original [syn::ImplItemMethod] of the invokable
    pub method: ImplItemMethod,
    /// Whether this invokable is mutable
    pub mutable: bool,
    /// The name of the C++ type for the return type if one has been specified
    pub return_cxx_type: Option<String>,
}
