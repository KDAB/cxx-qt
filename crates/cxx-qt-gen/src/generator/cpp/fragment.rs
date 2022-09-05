// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::cpp::types::CppType;

/// A generic C++ header and source pairing
pub struct CppFragmentPair {
    pub header: String,
    pub source: String,
}

pub struct CppNamedType {
    pub ident: String,
    pub ty: CppType,
}
