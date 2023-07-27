// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]
//! The cxx-qt-macro crate provides the procedural attribute macros which are used with cxx-qt.

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemMod};

use cxx_qt_gen::{write_rust, GeneratedRustBlocks, Parser};

/// A procedural macro which generates a QObject for a struct inside a module.
///
/// # Example
///
/// ```rust
/// #[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
/// mod qobject {
///     unsafe extern "RustQt" {
///         #[qobject]
///         # // Note that we can't use properties as this confuses the linker on Windows
///         type MyObject = super::MyObjectRust;
///
///         #[qinvokable]
///         fn invokable(self: &MyObject, a: i32, b: i32) -> i32;
///     }
/// }
///
/// #[derive(Default)]
/// pub struct MyObjectRust;
///
/// impl qobject::MyObject {
///     fn invokable(&self, a: i32, b: i32) -> i32 {
///         a + b
///     }
/// }
///
/// # // Note that we need a fake main for doc tests to build
/// # fn main() {}
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
    let args_input = format!("#[cxx_qt::bridge({args})] mod dummy;");
    let attrs = syn::parse_str::<ItemMod>(&args_input).unwrap().attrs;
    module.attrs = attrs.into_iter().chain(module.attrs.into_iter()).collect();

    // Extract and generate the rust code
    extract_and_generate(module)
}

/// A macro which describes that a struct should be made into a QObject.
///
/// It should not be used by itself and instead should be used inside a cxx_qt::bridge definition.
///
/// # Example
///
/// ```rust
/// #[cxx_qt::bridge]
/// mod my_object {
///     extern "RustQt" {
///         #[qobject]
///         # // Note that we can't use properties as this confuses the linker on Windows
///         type MyObject = super::MyObjectRust;
///     }
/// }
///
/// #[derive(Default)]
/// pub struct MyObjectRust;
///
/// # // Note that we need a fake main for doc tests to build
/// # fn main() {}
/// ```
///
/// You can also specify a custom base class by using `#[base = "QStringListModel"]`, you must then use CXX to add any includes needed.
///
/// # Example
///
/// ```rust
/// #[cxx_qt::bridge]
/// mod my_object {
///     extern "RustQt" {
///         #[qobject]
///         #[base = "QStringListModel"]
///         # // Note that we can't use properties as this confuses the linker on Windows
///         type MyModel = super::MyModelRust;
///     }
///
///     unsafe extern "C++" {
///         include!(<QtCore/QStringListModel>);
///     }
/// }
///
/// #[derive(Default)]
/// pub struct MyModelRust;
///
/// # // Note that we need a fake main for doc tests to build
/// # fn main() {}
/// ```
#[proc_macro_attribute]
pub fn qobject(_args: TokenStream, _input: TokenStream) -> TokenStream {
    unreachable!("qobject should not be used as a macro by itself. Instead it should be used within a cxx_qt::bridge definition")
}

// Take the module and C++ namespace and generate the rust code
fn extract_and_generate(module: ItemMod) -> TokenStream {
    Parser::from(module)
        .and_then(|parser| GeneratedRustBlocks::from(&parser))
        .map(|generated_rust| write_rust(&generated_rust))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
