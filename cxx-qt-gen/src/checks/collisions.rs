// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::Parser;
use syn::{Error, Ident, Result};

/// For two lists of [syn::Ident] fail if any names are the same
pub fn contains_colliding_names(first: &[&Ident], second: &[&Ident]) -> Result<()> {
    for item in first {
        if second.contains(item) {
            return Err(Error::new(
                item.span(),
                format!("Name ({}) collides another name", item.to_string().as_str()),
            ));
        }
    }

    Ok(())
}

/// Check for colliding names in the Parser components
pub fn check_for_colliding_names(parser: &Parser) -> Result<()> {
    for qobject in parser.cxx_qt_data.qobjects.values() {
        // Collect the list of names
        let invokables = qobject
            .invokables
            .iter()
            .map(|invokable| &invokable.sig.ident)
            .collect::<Vec<&Ident>>();
        let properties = qobject
            .properties
            .iter()
            .map(|property| &property.ident)
            .collect::<Vec<&Ident>>();
        let signals = if let Some(signals) = &qobject.signals {
            signals.signals.iter().map(|signal| &signal.ident).collect()
        } else {
            vec![]
        };
        // TODO: we also need to consider generated names, like emit{signal}
        // TODO: need to check for *_wrapper and *Wrapper being used
        // TODO: we also need to consider different cases
        // TODO: need to consider get/set/changed for properties with signals
        //
        // TODO: do we have helpers in generator for converting names that we can share

        // Check for collisions
        contains_colliding_names(&invokables, &properties)?;
        contains_colliding_names(&invokables, &signals)?;
        contains_colliding_names(&properties, &signals)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO
}
