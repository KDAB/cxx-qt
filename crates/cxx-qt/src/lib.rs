// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemMod};

use cxx_qt_gen::{write_rust, GeneratedRustBlocks, Parser};

/// A procedural macro which generates a QObject for a struct inside a module.
///
/// # Example
///
/// ```ignore
/// #[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
/// mod my_object {
///     #[cxx_qt::qobject]
///     #[derive(Default)]
///     struct RustObj {
///         #[qproperty]
///         property: i32,
///     }
///
///     impl qobject::RustObj {
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
    let args_input = format!("#[cxx_qt::bridge({args})] mod dummy;");
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
///     #[cxx_qt::qsignals(MyObject)]
///     enum MySignals {
///         Ready,
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn qsignals(_args: TokenStream, _input: TokenStream) -> TokenStream {
    unreachable!("cxx_qt::qsignals should not be used as a macro by itself. Instead it should be used within a cxx_qt::bridge definition")
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
///
/// You can also specify a custom base class by using `#[cxx_qt::qobject(base = "QStringListModel")]`, you must then use CXX to add any includes needed.
///
/// # Example
///
/// ```ignore
/// #[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
/// mod my_object {
///     #[cxx_qt::qobject(base = "QStringListModel")]
///     struct MyModel;
///
///     unsafe extern "C++" {
///         include!(<QtCore/QStringListModel>);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn qobject(_args: TokenStream, _input: TokenStream) -> TokenStream {
    unreachable!("cxx_qt::qobject should not be used as a macro by itself. Instead it should be used within a cxx_qt::bridge definition")
}

/// A macro which allows you to access base class methods from within Rust.
///
/// It should not be used by itself and instead should be used inside a cxx_qt::bridge definition.
/// Furthermore, the macro must be placed within the `impl` block of a `qobject::T`.
/// See [the book page](https://kdab.github.io/cxx-qt/book/concepts/inheritance.html) for more
/// details.
///
/// # Example
/// ``` ignore
/// #[cxx_qt::bridge]
/// mod my_object {
///     #[cxx_qt::qobject(base="QAbstractListModel")]
///     #[derive(Default)]
///     struct MyObject;
///
///     #[cxx_qt::inherit]
///     extern "C++" {
///         // Unsafe to call
///         unsafe fn begin_insert_rows(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
///     }
///
///     #[cxx_qt::inherit]
///     unsafe extern "C++" {
///         // Safe to call - you are responsible to ensure this is true!
///         fn end_insert_rows(self: Pin<&mut Self>);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn inherit(_args: TokenStream, _input: TokenStream) -> TokenStream {
    unreachable!("cxx_qt::inherit should not be used as a macro by itself. Instead it should be used within a cxx_qt::bridge definition")
}

// Take the module and C++ namespace and generate the rust code
fn extract_and_generate(module: ItemMod) -> TokenStream {
    let parser = Parser::from(module).unwrap();
    let generated_rust = GeneratedRustBlocks::from(&parser).unwrap();
    write_rust(&generated_rust).into()
}
