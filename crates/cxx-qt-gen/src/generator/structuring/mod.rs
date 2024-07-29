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

use crate::parser::cxxqtdata::ParsedCxxQtData;
use syn::{Error, Result};

/// The list of all structures that could be associated from the parsed data.
/// Most importantly, this includes the list of qobjects.
pub struct Structures<'a> {
    /// The list of qobjects
    pub qobjects: Vec<StructuredQObject<'a>>,
}

impl<'a> Structures<'a> {
    /// Create a new `Structures` object from the given `ParsedCxxQtData`
    /// Returns an error, if any references could not be resolved.
    pub fn new(cxxqtdata: &'a ParsedCxxQtData) -> Result<Self> {
        // Methods in cxxqtdata need to be parsed and associated with their objects
        let mut qobjects: Vec<_> = cxxqtdata
            .qobjects
            .values()
            .map(|qobject| StructuredQObject {
                declaration: qobject,
                qenums: Vec::new(),
                methods: HashMap::new(),
                signals: HashMap::new(),
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

        // Associate each method parsed with its appropriate qobject
        for method in &cxxqtdata.methods {
            if let Some(qobject_ident) = qobjects.iter_mut().find(|qobject| {
                qobject.declaration.name.rust_unqualified() == &method.qobject_ident
                // Potentially refactor qobjects to be a HashMap for faster lookups by Ident
            }) {
                qobject_ident
                    .methods
                    .insert(method.name.rust_unqualified().clone(), method);
            } else {
                return Err(Error::new_spanned(
                    &method.qobject_ident,
                    format!("Unknown QObject: {:?}", &method.qobject_ident),
                ));
            }
        }

        // Associate each signal parsed with its appropriate qobject
        for signal in &cxxqtdata.signals {
            if let Some(qobject_ident) = qobjects.iter_mut().find(|qobject| {
                qobject.declaration.name.rust_unqualified() == &signal.qobject_ident
            }) {
                qobject_ident
                    .signals
                    .insert(signal.name.rust_unqualified().clone(), signal);
            } else {
                return Err(Error::new_spanned(
                    &signal.qobject_ident,
                    format!("Unknown QObject: {:?}", &signal.qobject_ident),
                ));
            }
        }

        Ok(Structures { qobjects })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;
    use quote::format_ident;
    use syn::{parse_quote, ItemMod};

    #[test]
    fn test_structures() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;

                    #[qobject]
                    type MyOtherObject = super::MyOtherObjectRust;
                }

                unsafe extern "RustQt" {
                    #[qinvokable]
                    fn test_fn(self: Pin<&mut MyObject>);

                    #[qinvokable]
                    fn test_fn_two(self: Pin<&mut MyObject>);

                    #[qinvokable]
                    fn test_fn_again(self: Pin<&mut MyOtherObject>);

                    #[qsignal]
                    fn ready(self: Pin<&mut MyOtherObject>);
                }
            }
        };

        let parser = Parser::from(module.clone()).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        assert_eq!(structures.qobjects.len(), 2);
        let my_object = &structures.qobjects[0];
        let my_other_object = &structures.qobjects[1];

        assert_eq!(
            *my_object.declaration.name.rust_unqualified(),
            format_ident!("MyObject")
        );
        assert_eq!(
            *my_other_object.declaration.name.rust_unqualified(),
            format_ident!("MyOtherObject")
        );
        assert_eq!(my_object.methods.len(), 2);
        assert_eq!(my_other_object.methods.len(), 1);

        assert!(my_object.signals.is_empty());
        assert_eq!(my_other_object.signals.len(), 1);

        let test_fn = my_object.methods.get(&format_ident!("test_fn")).unwrap();
        let test_fn_two = my_object
            .methods
            .get(&format_ident!("test_fn_two"))
            .unwrap();
        let test_fn_again = my_other_object
            .methods
            .get(&format_ident!("test_fn_again"))
            .unwrap();
        let ready = my_other_object
            .signals
            .get(&format_ident!("ready"))
            .unwrap();
    }
}
