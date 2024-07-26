// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// This module contains the "structuring" phase of C++ generation.
///
/// As proposed in [#937](https://github.com/KDAB/cxx-qt/issues/937), we aim to refactor
/// cxx-qt-gen, especially to simplify parsing.
/// This module is responsible for structuring the parsed data into a form that is easier to work
/// with when generating C++ code.
/// This mostly means grouping QObjects with their QEnums, QSignals, etc..
///
/// All resulting structures are listed in the `Structures` struct.
pub mod qobject;

pub use qobject::StructuredQObject;
use std::collections::HashMap;

use crate::naming::Name;
use crate::parser::cxxqtdata::ParsedCxxQtData;
use crate::parser::method::ParsedMethod;
use syn::{Error, Result};

/// The list of all structures that could be associated from the parsed data.
/// Most importantly, this includes the list of qobjects.
pub struct Structures<'a> {
    /// The list of qobjects
    pub qobjects: Vec<qobject::StructuredQObject<'a>>,
}

impl<'a> Structures<'a> {
    /// Create a new `Structures` object from the given `ParsedCxxQtData`
    /// Returns an error, if any references could not be resolved.
    pub fn new(cxxqtdata: &'a ParsedCxxQtData) -> Result<Self> {
        let mut qobjects: Vec<_> = cxxqtdata
            .qobjects
            .values()
            .map(|qobject| StructuredQObject {
                declaration: qobject,
                qenums: Vec::new(),
                methods: HashMap::new(), // TODO: Add in hashed method pairs
            })
            .collect();

        for qenum in &cxxqtdata.qenums {
            if let Some(qobject_ident) = &qenum.qobject {
                if let Some(qobject) = qobjects
                    .iter_mut()
                    .find(|qobject| qobject.declaration.name.rust_unqualified() == qobject_ident)
                {
                    qobject.qenums.push(qenum);
                } else {
                    return Err(Error::new_spanned(
                        qobject_ident,
                        format!("Unknown QObject: {qobject_ident}"),
                    ));
                }
            }
        }

        Ok(Structures { qobjects })
    }
}
