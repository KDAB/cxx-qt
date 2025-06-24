// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We explicitly allow missing docs for the macros in this crate, as they should be documented in
// cxx-qt itself.
#![allow(missing_docs)]
//! The cxx-qt-macro crate provides the procedural attribute macros which are used with cxx-qt.
//!
//! See the [cxx-qt crate docs](https://docs.rs/cxx-qt/latest/) for documentation of the macros
//! inside this crate.

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemMod};

use cxx_qt_gen::{self_inlining::qualify_self_types, write_rust, GeneratedRustBlocks, Parser};

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
    module.attrs = attrs.into_iter().chain(module.attrs).collect();

    // Extract and generate the rust code
    extract_and_generate(module)
}

#[proc_macro_attribute]
pub fn qobject(_args: TokenStream, _input: TokenStream) -> TokenStream {
    unreachable!("qobject should not be used as a macro by itself. Instead it should be used within a cxx_qt::bridge definition")
}

#[proc_macro]
pub fn init_crate(args: TokenStream) -> TokenStream {
    let crate_name = syn::parse_macro_input!(args as syn::Ident);
    let function_name = quote::format_ident!("cxx_qt_init_crate_{crate_name}");
    quote::quote! {
        extern "C" {
            fn #function_name() -> bool;
        }
        unsafe { #function_name(); }
    }
    .into()
}

#[proc_macro]
pub fn init_qml_module(args: TokenStream) -> TokenStream {
    let module_uri = syn::parse_macro_input!(args as syn::LitStr);
    let module_name = syn::Ident::new(&module_uri.value().replace('.', "_"), module_uri.span());

    let function_name = quote::format_ident!("cxx_qt_init_qml_module_{module_name}");
    quote::quote! {
        extern "C" {
            fn #function_name() -> bool;
        }
        unsafe { #function_name(); }
    }
    .into()
}

// Take the module and C++ namespace and generate the rust code
fn extract_and_generate(module: ItemMod) -> TokenStream {
    Parser::from(module)
        .and_then(|mut parser| {
            qualify_self_types(&mut parser)?;
            Ok(parser)
        })
        .and_then(|parser| GeneratedRustBlocks::from(&parser))
        .map(|generated_rust| write_rust(&generated_rust, None))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
