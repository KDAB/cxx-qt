// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use quote::ToTokens;
use serde::{Deserialize, Serialize};

/// Representation of a generated CXX header, source, and name
#[derive(Serialize, Deserialize)]
struct GeneratedType {
    header: String,
    name: String,
    source: String,
}

/// Generate a CXX header, source, name for a given Rust file
fn gen_cxx_sources(folder: &str, file_stem: &str) -> GeneratedType {
    // Read the rust source files
    let path = format!("{}/{}.rs", folder, file_stem);
    println!("cargo:rerun-if-changed={}", path);
    let content = std::fs::read_to_string(path).expect("Could not read Rust file");
    let file = syn::parse_file(&content).unwrap();

    // Generate the CXX header and cc for the file
    let opt = cxx_gen::Opt::default();
    let generated = cxx_gen::generate_header_and_cc(file.into_token_stream(), &opt)
        .expect("Could not generate C++ from Rust file");

    GeneratedType {
        header: String::from_utf8(generated.header).unwrap(),
        name: format!("{}_cxx", file_stem),
        source: String::from_utf8(generated.implementation).unwrap(),
    }
}

/// Write generates types to a given file as JSON
fn write_cxx_sources(gen: Vec<GeneratedType>, path: &str) {
    let file = std::fs::File::create(path).expect("Could not create generated file");
    serde_json::to_writer(file, &gen).unwrap();
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

    // Prepare cxx-qt-lib dir we'll write to
    let path = format!("{}/cxx-qt-lib", dir_out);
    std::fs::create_dir_all(&path).expect("Could not create cxx-qt-lib dir");

    // Read the types directory for CXX objects
    let types_dir = format!("{}/src/types/", dir_manifest);
    // If any of the files in this directory change, then we need to re-run
    println!("cargo:rerun-if-changed={}", types_dir);

    let mut generated = vec![];

    for entry in std::fs::read_dir(&types_dir).expect("Could not open types folder") {
        let path = entry.expect("Could not open file").path();
        let file_stem = path
            .file_stem()
            .expect("Could not find file name")
            .to_str()
            .expect("Could not convert to unicode");

        // Check we are a file and not the mod.rs
        if path.is_file() && file_stem != "mod" {
            generated.push(gen_cxx_sources(&types_dir, file_stem));
        }
    }

    // Write the generated sources to a qt_types_cxx.json file
    write_cxx_sources(generated, &format!("{}/qt_types_cxx.json", path));
}
