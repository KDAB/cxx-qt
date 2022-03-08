// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use quote::ToTokens;
use std::io::Write;

fn main() {
    let opt = cxx_gen::Opt::default();

    // Read the cargo folder and out folder
    let mut dir_manifest = std::env::var("CARGO_MANIFEST_DIR").expect("Could not get manifest dir");
    if cfg!(windows) {
        dir_manifest = dir_manifest.replace('\\', "/");
    }
    let mut dir_out = std::env::var("OUT_DIR").expect("Could not get out dir");
    if cfg!(windows) {
        dir_out = dir_out.replace('\\', "/");
    }

    // Prepare directories we'll write to
    let include_path = format!("{}/cxx-qt-lib/include", dir_out);
    std::fs::create_dir_all(&include_path).expect("Could not create cxx-qt-lib include dir");
    let src_path = format!("{}/cxx-qt-lib/src", dir_out);
    std::fs::create_dir_all(&src_path).expect("Could not create cxx-qt-lib sc dir");

    let qfiles = vec![
        "qcolor",
        "qdate",
        "qdatetime",
        "qpoint",
        "qpointf",
        "qrect",
        "qrectf",
        "qsize",
        "qsizef",
        "qstring",
        "qtime",
        "qurl",
    ];
    for qfile in qfiles {
        // Read the rust source files
        let path = format!("{}/src/{}.rs", dir_manifest, qfile);
        let content = std::fs::read_to_string(path).expect("Could not read Rust file");
        let file = syn::parse_file(&content).unwrap();

        // Generate the CXX header and cc for the file
        let gen_result = cxx_gen::generate_header_and_cc(file.into_token_stream(), &opt)
            .expect("Could not generate C++ from Rust file");

        // Write out the CXX files
        let h_path = format!("{}/{}_cxx.h", include_path, qfile);
        let mut header = std::fs::File::create(&h_path).expect("Could not create header file");
        header
            .write_all(&gen_result.header)
            .expect("Could not write header file");

        let cpp_path = format!("{}/{}_cxx.cpp", src_path, qfile);
        let mut cpp = std::fs::File::create(&cpp_path).expect("Could not create cpp file");
        cpp.write_all(&gen_result.implementation)
            .expect("Could not write cpp file");
    }
}
