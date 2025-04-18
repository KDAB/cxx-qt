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

use crate::parser::{
    cxxqtdata::ParsedCxxQtData,
    trait_impl::{TraitImpl, TraitKind},
};
pub use qobject::StructuredQObject;
use syn::{Error, Ident, Result};

/// The list of all structures that could be associated from the parsed data.
/// Most importantly, this includes the list of qobjects.
pub struct Structures<'a> {
    /// The list of qobjects
    pub qobjects: Vec<StructuredQObject<'a>>,
}

/// Error for looking up against a QObject which isn't registered in Structures
fn unknown_qobject(id: &Ident) -> Error {
    not_found_error("QObject", id)
}

fn not_found_error(obj: &str, id: &Ident) -> Error {
    Error::new_spanned(id, format!("{obj} with name `{id}` not found!"))
}

fn find_qobject<'a, 'b>(
    qobjects: &'b mut [StructuredQObject<'a>],
    ident: &Ident,
) -> Result<&'b mut StructuredQObject<'a>> {
    qobjects
        .iter_mut()
        .find(|qobject| qobject.has_qobject_name(ident))
        .ok_or_else(|| unknown_qobject(ident))
}

impl<'a> Structures<'a> {
    fn structure_trait_impls(
        qobjects: &mut [StructuredQObject<'a>],
        trait_impls: &'a [TraitImpl],
    ) -> Result<()> {
        // Associate each trait impl with its appropriate qobject
        for imp in trait_impls {
            let qobject = find_qobject(qobjects, &imp.qobject)?;
            match imp.kind {
                TraitKind::Threading => {
                    if qobject.threading {
                        return Err(Error::new_spanned(
                            &imp.declaration,
                            format!(
                                "Threading already enabled on QObject {qobject}!",
                                qobject = imp.qobject
                            ),
                        ));
                    }
                    qobject.threading = true;
                }
                // TODO: Check for duplicate declarations?
                TraitKind::Constructor(ref constructor) => qobject.constructors.push(constructor),
            }
        }
        Ok(())
    }

    /// Create a new `Structures` object from the given `ParsedCxxQtData`
    /// Returns an error, if any references could not be resolved.
    pub fn new(cxxqtdata: &'a ParsedCxxQtData) -> Result<Self> {
        let mut qobjects: Vec<_> = cxxqtdata
            .qobjects()
            .map(StructuredQObject::from_qobject)
            .collect();

        for qenum in &cxxqtdata.qenums {
            if let Some(qobject_ident) = &qenum.qobject {
                let qobject = find_qobject(&mut qobjects, qobject_ident)?;
                qobject.qenums.push(qenum);
            }
        }

        // Associate each method parsed with its appropriate qobject
        for method in cxxqtdata.methods() {
            let qobject = find_qobject(&mut qobjects, &method.qobject_ident)?;
            qobject.methods.push(method);
        }

        // Associate each inherited method parsed with its appropriate qobject
        for inherited_method in cxxqtdata.inherited_methods() {
            let qobject = find_qobject(&mut qobjects, &inherited_method.qobject_ident)?;
            qobject.inherited_methods.push(inherited_method);
        }

        // Associate each signal parsed with its appropriate qobject
        for signal in cxxqtdata.signals() {
            let qobject = find_qobject(&mut qobjects, &signal.qobject_ident)?;
            qobject.signals.push(signal);
        }

        Self::structure_trait_impls(&mut qobjects, &cxxqtdata.trait_impls)?;

        Ok(Structures { qobjects })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_errors;
    use crate::Parser;
    use quote::format_ident;
    use syn::{parse_quote, ItemMod};

    #[test]
    fn test_invalid_lookup() {
        let module = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }
            }
        };

        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        let qobject = structures.qobjects.first().unwrap();
        assert!(qobject.method_lookup(&format_ident!("NotReal")).is_err());
        assert!(qobject
            .signal_lookup(&format_ident!("NotRealEither"))
            .is_err());
    }

    #[test]
    fn test_inherited_lookup() {
        let module = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }

                unsafe extern "RustQt" {
                    #[qinvokable]
                    #[inherit]
                    fn test_fn(self: Pin<&mut MyObject>);
                }
            }
        };

        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        let qobject = structures.qobjects.first().unwrap();
        assert!(qobject.method_lookup(&format_ident!("test_fn")).is_ok());
    }

    #[test]
    fn test_structures() {
        let module = parse_quote! {
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

        let parser = Parser::from(module).unwrap();
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

    fn mock_bridge() -> ItemMod {
        parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    type MyObject = super::MyObjectRust;
                }
            }
        }
    }

    #[test]
    fn test_incompatible_trait_impl() {
        let mut bridge = mock_bridge();
        bridge.content.as_mut().unwrap().1.extend([
            parse_quote! {impl cxx_qt::Threading for MyObject {}},
            parse_quote! {impl cxx_qt::Threading for MyObject {}},
        ]);
        let parser = Parser::from(bridge).unwrap();
        assert!(Structures::new(&parser.cxx_qt_data).is_err());
    }

    #[test]
    fn test_create_invalid_structures() {
        assert_parse_errors! {
            |module| {
                Structures::new(&Parser::from(module).unwrap().cxx_qt_data).map(|_| ())
            } =>

            {
                // Unknown QObject
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
            }

            {
                // Invalid QObject for QEnum
                #[cxx_qt::bridge]
                mod ffi {
                    #[qenum(MyObject)]
                    enum MyEnum {
                        A,
                    }
                }
            }

            {
                // Undeclared QObject for method
                #[cxx_qt::bridge]
                mod ffi {
                    unsafe extern "RustQt" {
                        #[qinvokable]
                        fn test_fn(self: Pin<&mut MyObject>);
                    }
                }
            }

            {
                // Undeclared QObject for signal
                #[cxx_qt::bridge]
                mod ffi {
                    unsafe extern "RustQt" {
                        #[qsignal]
                        fn test_fn(self: Pin<&mut MyObject>);
                    }
                }
            }

            {
                // Undeclared QObject for inherited method
                #[cxx_qt::bridge]
                mod ffi {
                    unsafe extern "RustQt" {
                        #[inherit]
                        fn test_fn(self: Pin<&mut MyObject>);
                    }
                }
            }
        }
    }
}
