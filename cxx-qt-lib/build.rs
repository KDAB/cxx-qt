// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use quote::ToTokens;
use std::io::Write;

fn gen_cxx_sources(folder: &str, file_stem: &str) -> cxx_gen::GeneratedCode {
    // Read the rust source files
    let path = format!("{}/{}.rs", folder, file_stem);
    println!("cargo:rerun-if-changed={}", path);
    let content = std::fs::read_to_string(path).expect("Could not read Rust file");
    let file = syn::parse_file(&content).unwrap();

    // Generate the CXX header and cc for the file
    let opt = cxx_gen::Opt::default();
    cxx_gen::generate_header_and_cc(file.into_token_stream(), &opt)
        .expect("Could not generate C++ from Rust file")
}

fn write_cxx_sources(
    gen_result: cxx_gen::GeneratedCode,
    file_stem: &str,
    include_path: &str,
    src_path: &str,
) {
    // Write out the CXX files
    let h_path = format!("{}/{}_cxx.h", include_path, file_stem);
    let mut header = std::fs::File::create(&h_path).expect("Could not create header file");
    header
        .write_all(&gen_result.header)
        .expect("Could not write header file");

    let cpp_path = format!("{}/{}_cxx.cpp", src_path, file_stem);
    let mut cpp = std::fs::File::create(&cpp_path).expect("Could not create cpp file");
    cpp.write_all(&gen_result.implementation)
        .expect("Could not write cpp file");
}

fn main() {
    // Read the cargo folder and out folder
    let mut dir_manifest = std::env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");
    if cfg!(windows) {
        dir_manifest = dir_manifest.replace('\\', "/");
    }
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");

    let mut dir_out = std::env::var("OUT_DIR").expect("Could not get out dir");
    if cfg!(windows) {
        dir_out = dir_out.replace('\\', "/");
    }
    println!("cargo:rerun-if-env-changed=OUT_DIR");

    // Prepare directories we'll write to
    let include_path = format!("{}/cxx-qt-lib/include", dir_out);
    std::fs::create_dir_all(&include_path).expect("Could not create cxx-qt-lib include dir");
    let src_path = format!("{}/cxx-qt-lib/src", dir_out);
    std::fs::create_dir_all(&src_path).expect("Could not create cxx-qt-lib src dir");

    // Read the types directory for CXX objects
    let types_dir = format!("{}/src/types/", dir_manifest);
    println!("cargo:rerun-if-changed={}", types_dir);

    for entry in std::fs::read_dir(&types_dir).expect("Could not open types folder") {
        let path = entry.expect("Could not open file").path();
        let file_stem = path
            .file_stem()
            .expect("Could not find file name")
            .to_str()
            .expect("Could not convert to unicode");

        // Check we are a file and not the mod.rs
        if path.is_file() && file_stem != "mod" {
            write_cxx_sources(
                gen_cxx_sources(&types_dir, file_stem),
                file_stem,
                &include_path,
                &src_path,
            );
        }
    }
}
