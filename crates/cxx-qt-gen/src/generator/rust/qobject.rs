// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::generator::structuring::StructuredQObject;
use crate::{
    generator::{
        naming::{namespace::NamespaceName, qobject::QObjectNames},
        rust::{
            constructor, cxxqttype,
            fragment::GeneratedRustFragment,
            inherit,
            method::generate_rust_methods,
            property::generate_rust_properties,
            signals::generate_rust_signals,
            threading,
        },
    },
    naming::TypeNames,
};
use quote::quote;
use syn::{parse_quote, Ident, Result};

impl GeneratedRustFragment {
    // Might need to be refactored to use a StructuredQObject instead (confirm with Leon)
    pub fn from_qobject(
        structured_qobject: &StructuredQObject,
        type_names: &TypeNames,
    ) -> Result<Self> {
        let qobject = structured_qobject.declaration;
        // Create the base object
        let qobject_names = QObjectNames::from_qobject(qobject, type_names)?;
        let namespace_idents = NamespaceName::from(qobject);
        let mut generated = Self::default();

        generated.append(&mut generate_qobject_definitions(
            &qobject_names,
            qobject.base_class.clone(),
            type_names,
        )?);

        // Generate methods for the properties, invokables, signals
        generated.append(&mut generate_rust_properties(
            &qobject.properties,
            &qobject_names,
            type_names,
            structured_qobject,
        )?);
        generated.append(&mut generate_rust_methods(
            &structured_qobject.methods,
            &qobject_names,
        )?);
        generated.append(&mut inherit::generate(
            &qobject_names,
            &structured_qobject.inherited_methods,
        )?);
        generated.append(&mut generate_rust_signals(
            &structured_qobject.signals,
            &qobject_names,
            type_names,
        )?);

        // If this type is a singleton then we need to add an include
        if let Some(qml_metadata) = &qobject.qml_metadata {
            if qml_metadata.singleton {
                generated.append(&mut GeneratedRustFragment {
                    cxx_mod_contents: vec![parse_quote! {
                        unsafe extern "C++" {
                            include!(<QtQml/QQmlEngine>);
                        }
                    }],
                    cxx_qt_mod_contents: vec![],
                })
            }
        }

        // If this type has threading enabled then add generation
        if structured_qobject.threading {
            generated.append(&mut threading::generate(
                &qobject_names,
                &namespace_idents,
                type_names,
            )?);
        }

        generated.append(&mut constructor::generate(
            &structured_qobject.constructors,
            &qobject_names,
            &namespace_idents,
            type_names,
        )?);

        generated.append(&mut cxxqttype::generate(&qobject_names, type_names)?);

        Ok(generated)
    }
}

/// Generate the C++ and Rust CXX definitions for the QObject
fn generate_qobject_definitions(
    qobject_idents: &QObjectNames,
    base: Option<Ident>,
    type_names: &TypeNames,
) -> Result<GeneratedRustFragment> {
    let cpp_class_name_rust = &qobject_idents.name.rust_unqualified();
    let cpp_class_name_cpp = &qobject_idents.name.cxx_unqualified();

    let rust_struct_name_rust = &qobject_idents.rust_struct.rust_unqualified();
    let rust_struct_name_rust_str = rust_struct_name_rust.to_string();
    let namespace = qobject_idents.namespace_tokens();
    let cxx_name = if cpp_class_name_rust.to_string() == *cpp_class_name_cpp {
        quote! {}
    } else {
        let cpp_class_name_cpp = cpp_class_name_cpp.to_string();
        quote! {
            #[doc = "\n\nNote: The C++ name of this QObject is: "]
            #[doc = #cpp_class_name_cpp]
            #[cxx_name = #cpp_class_name_cpp]
        }
    };

    let cpp_struct_qualified = &qobject_idents.name.rust_qualified();

    let base_upcast = if let Some(base) = base {
        let base_name = type_names.lookup(&base)?.rust_qualified();
        vec![
            parse_quote! { impl cxx_qt::Upcast<#base_name> for #cpp_struct_qualified {} },
            // Until we can actually implement the Upcast trait properly, we just need to silence
            // the warning that the base class is otherwise unused.
            // This can be done with an unnamed import and the right attributes
            parse_quote! {
                #[allow(unused_imports)]
                #[allow(dead_code)]
                use #base_name as _;
            },
        ]
    } else {
        vec![]
    };

    Ok(GeneratedRustFragment {
        cxx_mod_contents: vec![
            parse_quote! {
                unsafe extern "C++" {
                    #[doc = "The C++ type for the QObject "]
                    #[doc = #rust_struct_name_rust_str]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
                    #namespace
                    #cxx_name
                    type #cpp_class_name_rust;
                }
            },
            parse_quote! {
                extern "Rust" {
                    // Needed for QObjects to have a namespace on their type or extern block
                    //
                    // A Namespace from cxx_qt::bridge would be automatically applied to all children
                    // but to apply it to only certain types, it is needed here too
                    #namespace
                    type #rust_struct_name_rust;
                }
            },
        ],
        cxx_qt_mod_contents: base_upcast
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::mock_qml_singleton;
    use crate::generator::structuring::Structures;
    use crate::parser::Parser;
    use crate::tests::assert_tokens_eq;
    use syn::{parse_quote, ItemMod};

    #[test]
    fn test_generated_rust_qobject_blocks_non_singleton() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    #[qml_element]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        assert!(GeneratedRustFragment::from_qobject(
            structures.qobjects.first().unwrap(),
            &parser.type_names,
        )
        .is_ok());
    }

    #[test]
    fn test_generated_rust_qobject_blocks_singleton() {
        let module = mock_qml_singleton();
        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        let rust = GeneratedRustFragment::from_qobject(
            structures.qobjects.first().unwrap(),
            &parser.type_names,
        )
        .unwrap();
        assert_eq!(rust.cxx_mod_contents.len(), 6);
        assert_tokens_eq(
            &rust.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc = "The C++ type for the QObject "]
                    #[doc = "MyObjectRust"]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
                    #[namespace = "cxx_qt"]
                    type MyObject;
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[1],
            quote! {
                extern "Rust" {
                    #[namespace = "cxx_qt"]
                    type MyObjectRust;
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[2],
            quote! {
                unsafe extern "C++" {
                    include!(<QtQml/QQmlEngine>);
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[3],
            quote! {
                extern "Rust" {
                    #[cxx_name = "createRs"]
                    #[namespace = "cxx_qt::cxx_qt_MyObject"]
                    fn create_rs_MyObjectRust() -> Box<MyObjectRust>;
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[4],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[cxx_name = "unsafeRust"]
                    #[namespace = "rust::cxxqt1"]
                    fn cxx_qt_ffi_MyObject_unsafeRust(outer: &MyObject) -> &MyObjectRust;
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[5],
            quote! {
                unsafe extern "C++" {
                    #[doc(hidden)]
                    #[cxx_name = "unsafeRustMut"]
                    #[namespace = "rust::cxxqt1"]
                    fn cxx_qt_ffi_MyObject_unsafeRustMut(outer: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
                }
            },
        );
    }
}
