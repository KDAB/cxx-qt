// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::Parser;
use syn::{Error, Ident, Result};

/// Is the given [syn::Ident] reserved
fn is_reserved_name(ident: &Ident) -> bool {
    matches!(
        ident.to_string().as_str(),
        // Cannot use unsafe or unsafeRustMut
        "unsafeRust" | "unsafeRustMut" | "unsafe_rust" | "unsafe_rust_mut" |
        // Cannot use handler names
        "updateRequester" | "updateState" | "update_requester" | "update_state" |
        // Cannot use internal member names
        "m_rustObj" | "m_rustObjMutex" | "m_initialised"
    )
}

/// For a given list of [syn::Ident] fail if a reserved name is found
fn contains_reserved_name(items: Vec<&Ident>, context: &str) -> Result<()> {
    for item in items {
        if is_reserved_name(item) {
            return Err(Error::new(
                item.span(),
                format!(
                    "Reserved name ({}) by CXX-Qt cannot be used in a {}",
                    item.to_string().as_str(),
                    context
                ),
            ));
        }
    }

    Ok(())
}

/// Check for reserved names in the Parser components
pub fn check_for_reserved_names(parser: &Parser) -> Result<()> {
    for qobject in parser.cxx_qt_data.qobjects.values() {
        // Ensure that invokable names are valid
        contains_reserved_name(
            qobject
                .invokables
                .iter()
                .map(|invokable| &invokable.sig.ident)
                .collect(),
            "invokable",
        )?;

        // Ensure that property names are valid
        contains_reserved_name(
            qobject
                .properties
                .iter()
                .map(|property| &property.ident)
                .collect(),
            "property",
        )?;

        // Ensure that signal names are valid
        if let Some(signals) = &qobject.signals {
            contains_reserved_name(
                signals.signals.iter().map(|signal| &signal.ident).collect(),
                "signal",
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO
}
