// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemMod};

use cxx_qt_gen::{extract_qobject, generate_qobject_rs};

#[proc_macro_attribute]
pub fn make_qobject(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as ItemMod);

    let qobject;
    match extract_qobject(module) {
        Ok(o) => qobject = o,
        Err(e) => return e.into(),
    }

    let gen_result = generate_qobject_rs(&qobject);
    match gen_result {
        Ok(tokens) => tokens.into(),
        Err(tokens) => tokens.into(),
    }
}
