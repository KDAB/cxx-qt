// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::generator::structuring::StructuredQObject;
use crate::{
    generator::{
        naming::{namespace::NamespaceName, qobject::QObjectNames},
        rust::{
            constructor, cxxqttype, fragment::GeneratedRustFragment, inherit,
            method::generate_rust_methods, property::generate_rust_properties,
            signals::generate_rust_signals, threading,
        },
    },
    naming::TypeNames,
};
use quote::quote;
use syn::{parse_quote, Attribute, Result};

impl GeneratedRustFragment {
    pub fn from_qobject(
        structured_qobject: &StructuredQObject,
        type_names: &TypeNames,
    ) -> Result<Self> {
        let qobject = structured_qobject.declaration;
        // Create the base object
        let qobject_names = QObjectNames::from_qobject(qobject, type_names)?;
        let namespace_idents = NamespaceName::from(qobject);

        let mut generated = vec![
            generate_qobject_definitions(&qobject_names, &qobject.cfgs)?,
            generate_rust_properties(
                &qobject.properties,
                &qobject_names,
                type_names,
                structured_qobject,
            )?,
            generate_rust_methods(&structured_qobject.methods, &qobject_names)?,
            inherit::generate(&qobject_names, &structured_qobject.inherited_methods)?,
            generate_rust_signals(&structured_qobject.signals, &qobject_names, type_names)?,
        ];

        // If this type is a singleton then we need to add an include
        if let Some(qml_metadata) = &qobject.qml_metadata {
            if qml_metadata.singleton {
                generated.push(GeneratedRustFragment::from_cxx_item(parse_quote! {
                    unsafe extern "C++" {
                        include!(<QtQml/QQmlEngine>);
                    }
                }))
            }
        }

        // If this type has threading enabled then add generation
        if structured_qobject.threading {
            generated.push(threading::generate(
                &qobject_names,
                &namespace_idents,
                type_names,
                &qobject.cfgs,
            )?);
        }

        // Generate casting impl
        generated.push(GeneratedRustFragment::generate_casting_impl(
            &qobject_names,
            type_names,
            &structured_qobject.declaration.name,
            &structured_qobject.declaration.base_class,
        )?);

        generated.extend(vec![
            constructor::generate(
                &structured_qobject.constructors,
                &qobject_names,
                &namespace_idents,
                type_names,
                &qobject.cfgs,
            )?,
            cxxqttype::generate(&qobject_names, type_names, &qobject.cfgs)?,
        ]);

        Ok(GeneratedRustFragment::flatten(generated))
    }
}

/// Generate the C++ and Rust CXX definitions for the QObject
fn generate_qobject_definitions(
    qobject_idents: &QObjectNames,
    cfgs: &[Attribute],
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

    Ok(GeneratedRustFragment {
        cxx_mod_contents: vec![
            parse_quote! {
                unsafe extern "C++" {
                    #[doc = "The C++ type for the QObject "]
                    #[doc = #rust_struct_name_rust_str]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/concepts/generated_qobject.html>"]
                    #namespace
                    #cxx_name
                    #(#cfgs)*
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
                    #(#cfgs)*
                    type #rust_struct_name_rust;
                }
            },
        ],
        cxx_qt_mod_contents: vec![],
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
        assert_eq!(rust.cxx_mod_contents.len(), 7);
        assert_tokens_eq(
            &rust.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc = "The C++ type for the QObject "]
                    #[doc = "MyObjectRust"]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/concepts/generated_qobject.html>"]
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
                extern "C++" {
                    #[doc(hidden)]
                    #[cxx_name = "upcastPtr"]
                    #[namespace = "rust::cxxqt1"]
                    unsafe fn cxx_qt_ffi_MyObject_upcastPtr(thiz: *const MyObject) -> *const QObject;

                    #[doc(hidden)]
                    #[cxx_name = "downcastPtr"]
                    #[namespace = "rust::cxxqt1"]
                    unsafe fn cxx_qt_ffi_MyObject_downcastPtr(base: *const QObject) -> *const MyObject;
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[4],
            quote! {
                extern "Rust" {
                    #[cxx_name = "createRs"]
                    #[namespace = "cxx_qt::cxx_qt_MyObject"]
                    fn create_rs_MyObjectRust() -> Box<MyObjectRust>;
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[5],
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
            &rust.cxx_mod_contents[6],
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
