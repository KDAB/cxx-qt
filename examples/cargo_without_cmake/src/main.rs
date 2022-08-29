// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cargo_imports
mod cxxqt_object;

use std::{convert::TryInto, ffi::CString, os::raw::c_char, os::raw::c_int, ptr};
// ANCHOR_END: book_cargo_imports

// ANCHOR: book_cargo_extern_c
extern "C" {
    fn run_cpp(argc: c_int, argv: *const *const c_char) -> c_int;
}
// ANCHOR_END: book_cargo_extern_c

// ANCHOR: book_cargo_rust_main
fn main() {
    // The Rust runtime exposes command line arguments as OsStrings, but
    // the QGuiApplication constructor needs *const *const c_char.
    // First, convert the OsStrings into owned CStrings.
    let args: Vec<CString> = std::env::args_os()
        .map(|string| {
            // Unix OsStrings can be directly converted to CStrings.
            #[cfg(unix)]
            use std::os::unix::ffi::OsStrExt;

            // Windows OsStrings are WTF-8 encoded, so they need to be
            // converted to UTF-8 Strings before being converted to CStrings.
            // https://simonsapin.github.io/wtf-8/
            #[cfg(windows)]
            let string = string.to_string_lossy();

            CString::new(string.as_bytes()).unwrap()
        })
        .collect();

    // Convert the CStrings into *const c_char, which are ?Sized, so they do not own
    // the data that will be passed to C++.
    let mut c_args: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
    c_args.push(ptr::null());

    // Call the C++ initialization code to start the QML GUI.
    unsafe {
        run_cpp(args.len().try_into().unwrap(), c_args.as_ptr());
    }
}
// ANCHOR_END: book_cargo_rust_main
