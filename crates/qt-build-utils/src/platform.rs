// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{env, process::Command};

fn command_help_output(command: &str) -> std::io::Result<std::process::Output> {
    Command::new(command).args(["--help"]).output()
}

/// Linking executables (including tests) with Cargo that link to Qt fails to link with GNU ld.bfd,
/// which is the default on most Linux distributions, so use GNU ld.gold, lld, or mold instead.
/// If you are using a C++ build system such as CMake to do the final link of the executable, you do
/// not need to call this function.
///
/// With Apple devices we set -fapple-link-rtlib as we build with -nodefaultlibs
/// otherwise we cannot user helpers from the compiler runtime in Qt
///
/// This does nothing on non-Unix platforms.
pub struct QtPlatformLinker;

impl QtPlatformLinker {
    /// Initialize support for linking executables with Cargo that link to Qt
    pub fn init() {
        if env::var("CARGO_CFG_UNIX").is_err() {
            return;
        }

        if let Ok(vendor) = env::var("CARGO_CFG_TARGET_VENDOR") {
            if vendor == "apple" {
                // Tell clang link to clang_rt as we build with -nodefaultlibs
                // otherwise we cannot use helpers from the compiler runtime in Qt
                println!("cargo::rustc-link-arg=-fapple-link-rtlib");
            }
        }

        let flags = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();
        // Don't override custom flags
        if !flags.contains("-fuse-ld") {
            // ld is the system default linker. On Linux, this is usually GNU ld.bfd, but it may be symlinked to another
            // linker. On macOS, Xcode ships lld with the executable named ld.
            let ld_help = String::from_utf8(
                command_help_output("ld")
                    .expect("Could not run ld command")
                    .stdout,
            )
            .unwrap();
            // bfd supports some exotic targets that other linkers do not.
            let ld_is_bfd = ld_help.contains("symbolsrec")
                || ld_help.contains("verilog")
                || ld_help.contains("tekhex");

            // Whatever linker is being used that's not bfd will likely work.
            if !ld_is_bfd {
                return;
            }

            // mold is fastest, but specifing mold with -fuse-ld requires GCC >= 12 or Clang.
            // Unfortunately cargo does not provide a means to set the linker driver via build scripts,
            // so linking would fail trying to use -fuse-ld=mold with GCC < 12 even if clang is installed.
            // So, prefer lld and gold to mold for robustness on the widest range of systems.
            // mold can still be used by manually specifying it in ~/.cargo/config.toml or the RUSTFLAGS environment variable.
            if command_help_output("lld").is_ok() {
                println!("cargo::rustc-link-arg=-fuse-ld=lld");
            } else if command_help_output("ld.gold").is_ok() {
                println!("cargo::rustc-link-arg=-fuse-ld=gold");
            } else if command_help_output("mold").is_ok() {
                println!("cargo::rustc-link-arg=-fuse-ld=mold");
            } else {
                println!("cargo::warning=Neither mold, lld, nor gold linkers were found. Linking with GNU ld.bfd will likely fail.");
            }
        }
    }
}
