// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// Whether apple is the current target
pub(crate) fn is_apple_target() -> bool {
    // TODO: should CARGO_CFG_* be used here instead?
    std::env::var("TARGET")
        .map(|target| target.contains("apple"))
        .unwrap_or_else(|_| false)
}

/// Whether windows is the current target
pub(crate) fn is_windows_target() -> bool {
    // TODO: should CARGO_CFG_* be used here instead?
    std::env::var("TARGET")
        .map(|target| target.contains("windows"))
        .unwrap_or_else(|_| false)
}

/// Whether emscripten is the current target
pub(crate) fn is_emscripten_target() -> bool {
    std::env::var("CARGO_CFG_TARGET_OS") == Ok("emscripten".to_owned())
}
