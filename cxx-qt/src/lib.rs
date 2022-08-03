// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemMod};

use cxx_qt_gen::{extract_qobject, generate_qobject_rs};

/// A procedural macro which generates a QObject for a struct inside a module.
///
/// # Example
///
/// ```ignore
/// #[cxx_qt::bridge]
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
pub fn bridge(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the TokenStream of a macro
    // this triggers a compile failure if the tokens fail to parse.
    let module = parse_macro_input!(input as ItemMod);

    // TODO: for now we use a fixed namespace, later this will come from the macro definition
    let cpp_namespace_prefix: Vec<String> = vec!["cxx_qt".to_owned()];

    // Extract and generate the rust code
    extract_and_generate(module, &cpp_namespace_prefix)
}

/// A macro which describes that an enum defines the signals for a QObject.
///
/// It should not be used by itself and instead should be used inside a cxx_qt::bridge definition.
///
/// # Example
///
/// ```ignore
/// #[cxx_qt::bridge]
/// mod my_object {
///     #[cxx_qt::signals(MyObject)]
///     enum MySignals {
///         Ready,
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn signals(_args: TokenStream, _input: TokenStream) -> TokenStream {
    unreachable!("cxx_qt::signals should not be used as a macro by itself. Instead it should be used within a cxx_qt::bridge definition")
}

// Take the module and C++ namespace and generate the rust code
//
// Note that wee need a separate function here, as we need to annotate the lifetimes to allow
// for cpp_namespace_prefix to outlive cpp_namespace_prefix_ref
fn extract_and_generate<'s>(module: ItemMod, cpp_namespace_prefix: &'s [String]) -> TokenStream {
    let cpp_namespace_prefix_ref: Vec<&'s str> =
        cpp_namespace_prefix.iter().map(AsRef::as_ref).collect();

    // Attempt to extract information about a QObject inside the module
    let qobject = match extract_qobject(&module, &cpp_namespace_prefix_ref) {
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
