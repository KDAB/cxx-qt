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
/// This mostly means grouping QObjects with their QEnums, QSignals, etc...
///
/// All resulting structures are listed in the `Structures` struct.
pub mod qobject;

use crate::parser::cxxqtdata::ParsedCxxQtData;
pub use qobject::StructuredQObject;
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
        let mut qobjects: Vec<_> = cxxqtdata
            .qobjects
            .values()
            .map(StructuredQObject::from_qobject)
            .collect();

        for qenum in &cxxqtdata.qenums {
            if let Some(qobject_ident) = &qenum.qobject {
                let qobject = qobjects
                    .iter_mut()
                    .find(|qobject| qobject.has_qobject_name(qobject_ident))
                    .ok_or_else(|| {
                        Error::new_spanned(
                            qobject_ident,
                            format!("Unknown QObject: {qobject_ident}"),
                        )
                    })?;
                qobject.qenums.push(qenum);
            }
        }

        // Associate each method parsed with its appropriate qobject
        for method in &cxxqtdata.methods {
            let qobject = qobjects
                .iter_mut()
                .find(|qobject| qobject.has_qobject_name(&method.qobject_ident))
                .ok_or_else(|| {
                    Error::new_spanned(
                        &method.qobject_ident,
                        format!("Unknown QObject: {:?}", &method.qobject_ident),
                    )
                })?;
            qobject.methods.push(method);
        }

        // Associate each inherited method parsed with its appropriate qobject
        for inherited_method in &cxxqtdata.inherited_methods {
            let qobject = qobjects
                .iter_mut()
                .find(|qobject| qobject.has_qobject_name(&inherited_method.qobject_ident))
                .ok_or_else(|| {
                    Error::new_spanned(
                        &inherited_method.qobject_ident,
                        format!("Unknown QObject: {:?}", &inherited_method.qobject_ident),
                    )
                })?;
            qobject.inherited_methods.push(inherited_method);
        }

        // Associate each signal parsed with its appropriate qobject
        for signal in &cxxqtdata.signals {
            let qobject = qobjects
                .iter_mut()
                .find(|qobject| qobject.has_qobject_name(&signal.qobject_ident))
                .ok_or_else(|| {
                    Error::new_spanned(
                        &signal.qobject_ident,
                        format!("Unknown QObject: {:?}", &signal.qobject_ident),
                    )
                })?;
            qobject.signals.push(signal);
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
    fn test_structuring_unknown_qobject() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }

                unsafe extern "RustQt" {
                    #[qsignal]
                    fn ready(self: Pin<&mut UnknownObject>);
                }
            }
        };
        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data);

        assert!(structures.is_err());
    }

    #[test]
    fn test_module_invalid_qobject() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                #[qenum(MyObject)]
                enum MyEnum {
                    A,
                }
            }
        };

        let parser = Parser::from(module.clone()).unwrap();
        assert!(Structures::new(&parser.cxx_qt_data).is_err());

        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                unsafe extern "RustQt" {
                    #[qinvokable]
                    fn test_fn(self: Pin<&mut MyObject>);
                }
            }
        };

        let parser = Parser::from(module.clone()).unwrap();
        assert!(Structures::new(&parser.cxx_qt_data).is_err());

        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                unsafe extern "RustQt" {
                    #[qsignal]
                    fn test_fn(self: Pin<&mut MyObject>);
                }
            }
        };

        let parser = Parser::from(module.clone()).unwrap();
        assert!(Structures::new(&parser.cxx_qt_data).is_err());

        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                unsafe extern "RustQt" {
                    #[inherit]
                    fn test_fn(self: Pin<&mut MyObject>);
                }
            }
        };

        let parser = Parser::from(module.clone()).unwrap();
        assert!(Structures::new(&parser.cxx_qt_data).is_err());
    }

    #[test]
    fn test_invalid_lookup() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }
            }
        };

        let parser = Parser::from(module.clone()).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        let qobject = structures.qobjects.first().unwrap();
        assert!(qobject.method_lookup(&format_ident!("NotReal")).is_err());
        assert!(qobject
            .signal_lookup(&format_ident!("NotRealEither"))
            .is_err());
    }

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

        // Checking methods were registered
        assert_eq!(
            *my_object.methods[0].name.rust_unqualified(),
            format_ident!("test_fn")
        );
        assert_eq!(
            *my_object.methods[1].name.rust_unqualified(),
            format_ident!("test_fn_two")
        );
        assert_eq!(
            *my_other_object.methods[0].name.rust_unqualified(),
            format_ident!("test_fn_again")
        );
        assert_eq!(
            *my_other_object.signals[0].name.rust_unqualified(),
            format_ident!("ready")
        );
    }
}
