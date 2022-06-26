// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemMod};

use cxx_qt_gen::{extract_qobject, generate_qobject_rs};

/// Read the C++ namespace prefix that cxx-qt-build has set for us
fn read_cpp_namespace_prefix() -> Vec<String> {
    let dir_target = std::env::var("OUT_DIR")
        .map(|s| s + "/../../../..")
        .or(std::env::var("CARGO_MANIFEST_DIR").map(|s| s + "/target"))
        .expect("Could not get target dir");
    let path = format!("{}/cxx-qt-gen/cpp_namespace_prefix.txt", dir_target);
    let contents = std::fs::read_to_string(path).expect("Could not read cpp namespace prefix");
    contents.split("::").map(|s| s.to_string()).collect()
}

/// A procedural macro which generates a QObject for a struct inside a module.
///
/// # Example
///
/// ```ignore
/// #[make_qobject]
/// mod my_object {
///     #[derive(Default)]
///     struct MyObject {
///         property: i32,
///     }
///
///     impl MyObject {
///         fn invokable(&self, a: i32, b: i32) -> i32 {
///             a + b
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn make_qobject(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the TokenStream of a macro
    // this triggers a compile failure if the tokens fail to parse.
    let module = parse_macro_input!(input as ItemMod);

    // Read the C++ namespace prefix set by cxx-qt-build
    let cpp_namespace_prefix = read_cpp_namespace_prefix();

    // Extract and generate the rust code
    extract_and_generate(module, &cpp_namespace_prefix)
}

// Take the module and C++ namespace and generate the rust code
//
// Note that wee need a separate function here, as we need to annotate the lifetimes to allow
// for cpp_namespace_prefix to outlive cpp_namespace_prefix_ref
fn extract_and_generate<'s>(module: ItemMod, cpp_namespace_prefix: &'s [String]) -> TokenStream {
    let cpp_namespace_prefix_ref: Vec<&'s str> =
        cpp_namespace_prefix.iter().map(AsRef::as_ref).collect();

    // Attempt to extract information about a QObject inside the module
    let qobject = match extract_qobject(module, &cpp_namespace_prefix_ref) {
        Ok(o) => o,
        Err(e) => return e.into(),
    };

    // From the extracted QObject, generate the rust code that replaces the original code
    // for the given QObject.
    let gen_result = generate_qobject_rs(&qobject, &cpp_namespace_prefix_ref);
    match gen_result {
        Ok(tokens) => tokens.into(),
        Err(tokens) => tokens.into(),
    }
}
