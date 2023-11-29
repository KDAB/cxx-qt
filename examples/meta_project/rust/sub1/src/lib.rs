// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We need to enable packed bundled libs to allow for +bundle and +whole-archive
// https://github.com/rust-lang/rust/issues/108081

mod sub1_object;

pub fn increment(number: u32) -> u32 {
    number + 2
}
