// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Alessandro Ambrosano <alessandro.ambrosano@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example implements an empty bridge, and it's used as a corner case to test
//! the build procedure
/// An empty CXX-Qt bridge
#[cxx_qt::bridge]
pub mod qobject {
}
