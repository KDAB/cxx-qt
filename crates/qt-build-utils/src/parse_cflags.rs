// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: pkg-config crate contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module contains code from the [pkg-config crate](https://github.com/rust-lang/pkg-config-rs).
//! It has been decoupled from the pkg-config crate because qt-build-utils reads Qt's .prl files instead, which
//! does not require a pkg-config executable to be available.

use std::{collections::HashSet, env, sync::OnceLock};

static mut LINKED_OBJECT_FILES: OnceLock<HashSet<String>> = OnceLock::new();

/// Extract the &str to pass to cargo:rustc-link-lib from a filename (just the file name, not including directories)
/// using target-specific logic.
fn extract_lib_from_filename<'a>(target: &str, filename: &'a str) -> Option<&'a str> {
    fn test_suffixes<'b>(filename: &'b str, suffixes: &[&str]) -> Option<&'b str> {
        for suffix in suffixes {
            if let Some(lib_basename) = filename.strip_suffix(suffix) {
                return Some(lib_basename);
            }
        }
        None
    }

    let prefix = "lib";
    #[allow(clippy::collapsible_else_if)]
    if target.contains("msvc") {
        // According to link.exe documentation:
        // https://learn.microsoft.com/en-us/cpp/build/reference/link-input-files?view=msvc-170
        //
        //   LINK doesn't use file extensions to make assumptions about the contents of a file.
        //   Instead, LINK examines each input file to determine what kind of file it is.
        //
        // However, rustc appends `.lib` to the string it receives from the -l command line argument,
        // which it receives from Cargo via cargo:rustc-link-lib:
        // https://github.com/rust-lang/rust/blob/657f246812ab2684e3c3954b1c77f98fd59e0b21/compiler/rustc_codegen_ssa/src/back/linker.rs#L828
        // https://github.com/rust-lang/rust/blob/657f246812ab2684e3c3954b1c77f98fd59e0b21/compiler/rustc_codegen_ssa/src/back/linker.rs#L843
        // So the only file extension that works for MSVC targets is `.lib`
        test_suffixes(filename, &[".lib"])
    } else if target.contains("windows") && target.contains("gnu") {
        // GNU targets for Windows, including gnullvm, use `LinkerFlavor::Gcc` internally in rustc,
        // which tells rustc to use the GNU linker. rustc does not prepend/append to the string it
        // receives via the -l command line argument before passing it to the linker:
        // https://github.com/rust-lang/rust/blob/657f246812ab2684e3c3954b1c77f98fd59e0b21/compiler/rustc_codegen_ssa/src/back/linker.rs#L446
        // https://github.com/rust-lang/rust/blob/657f246812ab2684e3c3954b1c77f98fd59e0b21/compiler/rustc_codegen_ssa/src/back/linker.rs#L457
        // GNU ld can work with more types of files than just the .lib files that MSVC's link.exe needs.
        // GNU ld will prepend the `lib` prefix to the filename if necessary, so it is okay to remove
        // the `lib` prefix from the filename. The `.a` suffix *requires* the `lib` prefix.
        // https://sourceware.org/binutils/docs-2.39/ld.html#index-direct-linking-to-a-dll
        if let Some(filename) = filename.strip_prefix(prefix) {
            test_suffixes(filename, &[".dll.a", ".dll", ".lib", ".a"])
        } else {
            test_suffixes(filename, &[".dll.a", ".dll", ".lib"])
        }
    } else if target.contains("apple") {
        if let Some(filename) = filename.strip_prefix(prefix) {
            test_suffixes(filename, &[".a", ".so", ".dylib"])
        } else {
            None
        }
    } else {
        if let Some(filename) = filename.strip_prefix(prefix) {
            test_suffixes(filename, &[".a", ".so"])
        } else {
            None
        }
    }
}

/// Split link_args produced by pkg-config --cflags and / or --libs into separate flags.
///
/// Backslash in link_args is used to preserve literal meaning of following byte.  Different words are
/// separated by unescaped space. Other whitespace characters generally should not occur unescaped
/// at all, apart from the newline at the end of link_args. For compatibility with what others
/// consumers of pkg-config link_args would do in this scenario, they are used here for splitting as
/// well.
fn split_flags(link_args: &[u8]) -> Vec<String> {
    let mut word = Vec::new();
    let mut words = Vec::new();
    let mut escaped = false;

    for &b in link_args {
        match b {
            _ if escaped => {
                escaped = false;
                word.push(b);
            }
            b'\\' => escaped = true,
            b'\t' | b'\n' | b'\r' | b' ' => {
                if !word.is_empty() {
                    words.push(String::from_utf8(word).unwrap());
                    word = Vec::new();
                }
            }
            _ => word.push(b),
        }
    }

    if !word.is_empty() {
        words.push(String::from_utf8(word).unwrap());
    }

    words
}

pub(crate) fn parse_libs_cflags(name: &str, link_args: &[u8], builder: &mut cc::Build) {
    let mut is_msvc = false;
    let target = env::var("TARGET");
    if let Ok(target) = &target {
        if target.contains("msvc") {
            is_msvc = true;
        }
    }

    let words = split_flags(link_args);

    // Handle single-character arguments like `-I/usr/include`
    let parts = words
        .iter()
        .filter(|l| l.len() > 2)
        .map(|arg| (&arg[0..2], &arg[2..]));
    for (flag, val) in parts {
        match flag {
            "-L" => {
                println!("cargo:rustc-link-search=native={val}");
            }
            "-F" => {
                println!("cargo:rustc-link-search=framework={val}");
            }
            "-I" => (),
            "-l" => {
                // These are provided by the CRT with MSVC
                if is_msvc && ["m", "c", "pthread"].contains(&val) {
                    continue;
                }

                println!("cargo:rustc-link-lib={val}");
            }
            "-D" => (),
            _ => {}
        }
    }

    // Handle multi-character arguments with space-separated value like `-framework foo`
    let mut iter = words.iter().flat_map(|arg| {
        if let Some(arg) = arg.strip_prefix("-Wl,") {
            arg.split(',').collect()
        } else {
            vec![arg.as_ref()]
        }
    });
    while let Some(part) = iter.next() {
        match part {
            "-framework" => {
                if let Some(lib) = iter.next() {
                    println!("cargo:rustc-link-lib=framework={lib}");
                }
            }
            "-isystem" | "-iquote" | "-idirafter" => {}
            _ => {
                let path = std::path::Path::new(part);
                if path.is_file() {
                    // Cargo doesn't have a means to directly specify a file path to link,
                    // so split up the path into the parent directory and library name.
                    if let (Some(dir), Some(file_name), Ok(target)) =
                        (path.parent(), path.file_name(), &target)
                    {
                        if file_name.to_string_lossy().ends_with(".o") {
                            let path_string = path.to_string_lossy().to_string();
                            unsafe {
                                // Linking will fail with duplicate symbol errors if the same .o file is linked twice.
                                // Many of Qt's .prl files repeat listing .o files that other .prl files also list.
                                let already_linked_object_files =
                                    LINKED_OBJECT_FILES.get_or_init(|| HashSet::new());
                                if !already_linked_object_files.contains(&path_string) {
                                    // Cargo doesn't have a means to directly specify an object to link,
                                    // so use the cc crate to specify it instead.
                                    // TODO: pass file path directly when link-arg library type is stabilized
                                    // https://github.com/rust-lang/rust/issues/99427#issuecomment-1562092085
                                    // TODO: remove builder argument when it's not used anymore to link object files.
                                    // also remove the dependency on cc when this is done
                                    builder.object(path);
                                }
                                LINKED_OBJECT_FILES.get_mut().unwrap().insert(path_string);
                            }
                        } else {
                            match extract_lib_from_filename(target, &file_name.to_string_lossy()) {
                                Some(lib_basename) => {
                                    println!("cargo:rustc-link-search={}", dir.display());
                                    println!("cargo:rustc-link-lib={lib_basename}");
                                }
                                None => {
                                    println!("cargo:warning=File path {} found in .prl file for {name}, but could not extract library base name to pass to linker command line", path.display());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let linker_options = words.iter().filter(|arg| arg.starts_with("-Wl,"));
    for option in linker_options {
        let mut pop = false;
        let mut ld_option = vec![];
        for subopt in option[4..].split(',') {
            if pop {
                pop = false;
                continue;
            }

            if subopt == "-framework" {
                pop = true;
                continue;
            }

            ld_option.push(subopt);
        }

        println!("cargo:rustc-link-arg=-Wl,{}", ld_option.join(","));
    }
}
