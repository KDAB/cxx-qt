// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::process::Command;

/// Whether apple is the current target
pub(crate) fn is_apple_target() -> bool {
    std::env::var("TARGET")
        .map(|target| target.contains("apple"))
        .unwrap_or_else(|_| false)
}

/// Whether emscripten is the current target
pub(crate) fn is_emscripten_target() -> bool {
    std::env::var("CARGO_CFG_TARGET_OS") == Ok("emscripten".to_owned())
}

/// Wrap a command in a native subshell
pub(crate) fn native_shell_command(command: &str) -> Command {
    let mut result: Command;
    if cfg!(target_os = "windows") {
        result = Command::new("cmd");
        result.args(["/C", command]);
    } else {
        result = Command::new("sh");
        result.args(["-c", command]);
    }
    result
}
