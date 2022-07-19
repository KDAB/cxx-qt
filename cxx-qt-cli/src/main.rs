// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::Parser;
use cxx_qt_build::GeneratedCpp;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// path to Rust file containing module with #[cxx_qt::bridge] attribute macro
    #[clap(short, long, value_parser)]
    input: PathBuf,

    /// directory to output generated C++ files
    #[clap(short, long, value_parser)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let generated_code = GeneratedCpp::new(&cli.input);
    let output_file_paths = generated_code.write_to_directory(&cli.output);

    for output_file_path in output_file_paths {
        println!("{}", output_file_path.display());
    }
}
