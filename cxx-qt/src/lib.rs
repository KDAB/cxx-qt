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
/// #[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
/// mod my_object {
///     #[derive(Default)]
///     struct Data {
///         property: i32,
///     }
///
///     #[derive(Default)]
///     struct RustObj;
///
///     impl cxx_qt::QObject<RustObj> {
///         #[qinvokable]
///         fn invokable(&self, a: i32, b: i32) -> i32 {
///             a + b
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn bridge(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the TokenStream of a macro
    // this triggers a compile failure if the tokens fail to parse.
    let mut module = parse_macro_input!(input as ItemMod);

    // Macros do not typically need to do anything with their own attribute name,
    // so rustc does not include that in the `args` or `input` TokenStreams.
    //
    // However, other code paths that use the parser do not enter from a macro invocation,
    // so they rely on parsing the `cxx_qt::bridge` attribute to identify where to start parsing.
    //
    // To keep the inputs to the parser consistent for all code paths,
    // add the attribute to the module before giving it to the parser.
    let args_input = format!("#[cxx_qt::bridge({})] mod dummy;", args);
    let attrs = syn::parse_str::<ItemMod>(&args_input).unwrap().attrs;
    module.attrs = attrs.into_iter().chain(module.attrs.into_iter()).collect();

    // Extract and generate the rust code
    extract_and_generate(module)
}

/// A macro which describes that an enum defines the signals for a QObject.
///
/// It should not be used by itself and instead should be used inside a cxx_qt::bridge definition.
///
/// # Example
///
/// ```ignore
/// #[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
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

/// A macro which describes that a struct should be made into a QObject.
///
/// It should not be used by itself and instead should be used inside a cxx_qt::bridge definition.
///
/// # Example
///
/// ```ignore
/// #[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
/// mod my_object {
///     #[cxx_qt::qobject]
///     struct MyObject;
/// }
/// ```
#[proc_macro_attribute]
pub fn qobject(_args: TokenStream, _input: TokenStream) -> TokenStream {
    unreachable!("cxx_qt::qobject should not be used as a macro by itself. Instead it should be used within a cxx_qt::bridge definition")
}

/// A macro which describes that the inner methods should be implemented on the C++ QObject.
/// This allows for defining C++ methods which are Q_INVOKABLE for QML in Rust.
///
/// It should not be used by itself and instead should be used inside a cxx_qt::bridge definition.
///
/// # Example
///
/// ```ignore
/// #[cxx_qt::bridge]
/// mod my_object {
///     #[derive(Default)]
///     struct Data {
///         property: i32,
///     }
///
///     #[derive(Default)]
///     struct RustObj;
///
///     impl cxx_qt::QObject<RustObj> {
///         #[qinvokable]
///         fn invokable(&self, a: i32, b: i32) -> i32 {
///             a + b
///         }
///     }
/// }
/// ```
#[proc_macro]
#[allow(non_snake_case)]
pub fn QObject(_input: TokenStream) -> TokenStream {
    unreachable!("cxx_qt::QObject should not be used as a macro by itself. Instead it should be used within a cxx_qt::bridge definition")
}

// Take the module and C++ namespace and generate the rust code
fn extract_and_generate(module: ItemMod) -> TokenStream {
    // Attempt to extract information about a QObject inside the module
    let qobject = match extract_qobject(&module) {
        Ok(o) => o,
        Err(e) => return e.into(),
    };

    // From the extracted QObject, generate the rust code that replaces the original code
    // for the given QObject.
    let gen_result = generate_qobject_rs(&qobject);
    match gen_result {
        Ok(tokens) => tokens.into(),
        Err(tokens) => tokens.into(),
    }
}
