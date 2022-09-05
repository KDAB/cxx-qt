// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{Ident, Type};

/// Describes a single parameter for a function
pub struct ParsedFunctionParameter {
    /// The [syn::Ident] of the parameter
    pub ident: Ident,
    /// The [syn::Type] of the parameter
    pub ty: Type,
    /// The name of the C++ type if one has been specified
    pub cxx_type: Option<String>,
}
