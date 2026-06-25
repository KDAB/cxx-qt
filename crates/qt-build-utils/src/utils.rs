// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{path::Path, process::Command};

/// Ensure that the given executable runs
///
/// This allows for raising errors that point to linker failures
/// as --help should always work
pub(crate) fn check_executable_help(executable: &Path) -> anyhow::Result<()> {
    let output = Command::new(executable)
        .arg("--help")
        // Binaries should work without environment and this prevents
        // LD_LIBRARY_PATH from causing different Qt version clashes
        .env_clear()
        .output()
        .map_err(|err| {
            anyhow::anyhow!(
                "{} process failed to complete: '{}'",
                executable.display(),
                err
            )
        })?;
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "{} unexpectedly exited a non-zero status code: '{:#?}'",
            executable.display(),
            output
        ));
    }

    Ok(())
}

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

/// Create a command to run qmake with a query.
///
/// On Windows, this wraps the invocation in `cmd /C` so that qmake can be a script.
pub(crate) fn qmake_query_command(qmake_path: &Path, var_name: &str) -> Command {
    let mut cmd;
    if cfg!(target_os = "windows") {
        cmd = Command::new("cmd");
        let qmake_command = format!("\"{}\" -query {}", qmake_path.display(), var_name);
        cmd.args(["/C", &qmake_command]);
    } else {
        cmd = Command::new(qmake_path);
        cmd.args(["-query", var_name]);
    }
    cmd
}
