// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use pretty_assertions::assert_str_eq;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::io::Write;

/// Helper to ensure that a given syn item is the same as the given TokenStream
pub fn assert_tokens_eq<T: ToTokens>(item: &T, tokens: TokenStream) {
    assert_str_eq!(item.to_token_stream().to_string(), tokens.to_string());
}

/// Helper for format Rust code
pub fn format_rs_source(rs_code: &str) -> String {
    // NOTE: this error handling is pretty rough so should only used for tests
    let mut command = std::process::Command::new("rustfmt");
    let mut child = command
        .args(["--emit", "stdout"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    // Scope stdin to force an automatic flush
    {
        let mut stdin = child.stdin.take().unwrap();
        write!(stdin, "{}", rs_code).unwrap();
    }

    let output = child.wait_with_output().unwrap();
    let output = String::from_utf8(output.stdout).unwrap();

    // Quote does not retain empty lines so we throw them away in the case of the
    // reference string as to not cause clashes
    output.replace("\n\n", "\n")
}

/// Helper to parse a quote TokenStream into a given syn item
pub fn tokens_to_syn<T: syn::parse::Parse>(tokens: proc_macro2::TokenStream) -> T {
    syn::parse2(tokens.into_token_stream()).unwrap()
}
