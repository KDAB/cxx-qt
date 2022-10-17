// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cargo_executable_build_rs
use cxx_qt_build::CxxQtBuilder;

use std::{process::Command, str};

#[cfg(unix)]
fn command_help_output(command: &str) -> std::io::Result<std::process::Output> {
    Command::new(command).args(["--help"]).output()
}

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Qml and Network libraries. Qt Core and Gui are always
        // linked, so there is no need to specify them here.
        .qt_modules(&["Qml", "Network"])
        // Generate C++ from the `#[cxx_qt::bridge]` module
        .file("src/cxxqt_object.rs")
        // Generate C++ code from the .qrc file with the rcc tool
        // https://doc.qt.io/qt-6/resources.html
        .qrc("qml/qml.qrc")
        // Tell CxxQtBuilder's internal cc::Build struct to compile the manually
        // written C++ file in addition to the generated C++.
        .cc_builder(|cc| {
            cc.file("src/cpp/run.cpp");
            println!("cargo:rerun-if-changed=src/cpp/run.cpp");
        })
        .build();

    // This fails to link with GNU ld.bfd, which is the default on most Linux distributions, so use GNU ld.gold, lld, or mold instead.
    #[cfg(unix)]
    {
        let flags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap();
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
                println!("cargo:rustc-link-arg=-fuse-ld=lld");
            } else if command_help_output("ld.gold").is_ok() {
                println!("cargo:rustc-link-arg=-fuse-ld=gold");
            } else if command_help_output("mold").is_ok() {
                println!("cargo:rustc-link-arg=-fuse-ld=mold");
            } else {
                println!("cargo:warning=Neither mold, lld, nor gold linkers were found. Linking with GNU ld.bfd will likely fail.");
            }
        }
    }
}
// ANCHOR_END: book_cargo_executable_build_rs
