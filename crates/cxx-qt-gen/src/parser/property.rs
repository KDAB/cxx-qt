// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Carl Schwan <carl.schwan@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{Ident, Type, Visibility, Expr};

#[derive(Debug)]
pub enum MaybeCustomFn {
    Custom(Box<Expr>),
    Default,
}

impl std::convert::From<Option<Expr>> for MaybeCustomFn {
    fn from(item: Option<Expr>) -> Self {
        match item {
            Some(expr) => Self::Custom(Box::new(expr)),
            None => Self::Default,
        }
    }
}

/// Describes a single field for a struct
pub struct ParsedRustField {
    /// The [syn::Ident] of the field
    pub ident: Ident,
    /// The [syn::Type] of the field
    pub ty: Type,
    /// The [syn::Visibility] of the field
    pub vis: Visibility,
}

/// Describes a single Q_PROPERTY for a struct
pub struct ParsedQProperty {
    /// The [syn::Ident] of the property
    pub ident: Ident,
    /// The [syn::Type] of the property
    pub ty: Type,
    /// The [syn::Visibility] of the property
    pub vis: Visibility,
    /// The name of the C++ type if one has been specified
    pub cxx_type: Option<String>,
    /// The custom getter [syn::Expr] of the property
    ///
    /// If this is not set and not custom setter was specified, both a
    /// default getter and setter will be generated.
    pub get: Option<MaybeCustomFn>,
    /// The custom setter [syn::Expr] of the property
    ///
    /// If this is not set and not custom setter was specified, both a
    /// default getter and setter will be generated.
    pub set: Option<MaybeCustomFn>,
}
