// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// Use this crate to test that missing_docs works with our generated code
#![deny(missing_docs)]

//! This example provides demostrations of most of the features of CXX-Qt
//! split into separate modules

pub mod containers;
pub mod custom_base_class;
pub mod invokables;
pub mod multiple_qobjects;
pub mod nested_qobjects;
pub mod properties;
pub mod serialisation;
pub mod signals;
pub mod singleton;
pub mod threading;
pub mod types;
pub mod uncreatable;
