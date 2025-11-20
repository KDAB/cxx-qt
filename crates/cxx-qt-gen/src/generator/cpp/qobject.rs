// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        cpp::{
            constructor, cxxqttype, fragment::CppFragment, inherit, method::generate_cpp_methods,
            property::generate_cpp_properties, qenum, signal::generate_cpp_signals, threading,
        },
        naming::{namespace::NamespaceName, qobject::QObjectNames},
        structuring::StructuredQObject,
    },
    naming::Name,
    GeneratedOpt,
};
use crate::{naming::TypeNames, parser::qobject::ParsedQObject};
use std::collections::BTreeSet;
use syn::Result;

#[derive(Default)]
pub struct GeneratedCppQObjectBlocks {
    /// List of includes
    pub includes: BTreeSet<String>,
    /// List of forward declares before the class and include of the generated CXX header
    ///
    /// For now these are not namespaced
    pub forward_declares: Vec<String>,
    /// List of forward declares before the class and include of the generated CXX header
    //
    // TODO: later combine these into forward_declares
    // once we have solved how to handle namespacing
    pub forward_declares_namespaced: Vec<String>,
    /// List of fragments which are outside of the QObject namespace
    pub fragments: Vec<CppFragment>,
    /// List of fragments which are outside of the QObject namespace and after the class
    pub post_fragments: Vec<CppFragment>,
    /// Base class of the QObject
    pub base_classes: Vec<String>,
    /// List of Qt Meta Object items (eg Q_PROPERTY)
    pub metaobjects: Vec<String>,
    /// List of public methods for the QObject
    pub methods: Vec<CppFragment>,
    /// List of private methods for the QObject
    pub private_methods: Vec<CppFragment>,
}

impl GeneratedCppQObjectBlocks {
    pub fn append(&mut self, other: &mut Self) {
        self.includes.append(&mut other.includes);
        self.forward_declares.append(&mut other.forward_declares);
        self.forward_declares_namespaced
            .append(&mut other.forward_declares_namespaced);
        self.fragments.append(&mut other.fragments);
        self.post_fragments.append(&mut other.post_fragments);
        self.base_classes.append(&mut other.base_classes);
        self.metaobjects.append(&mut other.metaobjects);
        self.methods.append(&mut other.methods);
        self.private_methods.append(&mut other.private_methods);
    }

    pub fn from(qobject: &ParsedQObject) -> GeneratedCppQObjectBlocks {
        let mut qml_specifiers = Vec::new();
        if let Some(qml_metadata) = &qobject.qml_metadata {
            // Note ensure that the header moc processes has QtQml/QQmlEngine
            // this is done via generator/rust/qobject

            qml_specifiers.push(format!("QML_NAMED_ELEMENT({})", qml_metadata.name));

            if qml_metadata.uncreatable {
                qml_specifiers
                    .push("QML_UNCREATABLE(\"Type cannot be created in QML.\")".to_owned());
            }

            if qml_metadata.singleton {
                qml_specifiers.push("QML_SINGLETON".to_owned());
            }
        }
        GeneratedCppQObjectBlocks {
            metaobjects: qml_specifiers,
            ..Default::default()
        }
    }
}

pub struct GeneratedCppQObject {
    /// Name of the QObject, with associated namespace, cxx_name, etc.
    pub name: Name,
    /// Name of the Rust struct that this QObject is associated with
    pub rust_struct: Name,
    /// Ident of the namespace for CXX-Qt internals of the QObject
    pub namespace_internals: String,
    /// The blocks of the QObject
    pub blocks: GeneratedCppQObjectBlocks,
    /// Whether this type has a #[qobject] / Q_OBJECT macro
    pub has_qobject_macro: bool,
}

impl GeneratedCppQObject {
    pub fn from(
        structured_qobject: &StructuredQObject,
        type_names: &TypeNames,
        opt: &GeneratedOpt,
    ) -> Result<GeneratedCppQObject> {
        let qobject = structured_qobject.declaration;

        // Create the base object
        let qobject_idents = QObjectNames::from_qobject(qobject, type_names)?;
        let namespace_idents = NamespaceName::from(qobject);
        let mut generated = GeneratedCppQObject {
            name: qobject.name.clone(),
            rust_struct: type_names.lookup(&qobject.rust_type)?.clone(),
            namespace_internals: namespace_idents.internal,
            blocks: GeneratedCppQObjectBlocks::from(qobject),
            has_qobject_macro: qobject.has_qobject_macro,
        };

        let base_class = if let Some(ident) = &qobject.base_class {
            type_names.lookup(ident)?.cxx_qualified()
        } else if qobject.has_qobject_macro {
            "QObject".to_owned()
        } else {
            // CODECOV_EXCLUDE_START
            unreachable!("Cannot have an empty #[base] attribute  with no #[qobject] attribute");
            // CODECOV_EXCLUDE_STOP
        };

        generated
            .blocks
            .base_classes
            .push(format!("public {base_class}"));

        // Add the CxxQtType rust and rust_mut methods
        generated
            .blocks
            .append(&mut cxxqttype::generate(&qobject_idents)?);

        // Generate methods for the properties, invokables, signals
        generated.blocks.append(&mut generate_cpp_properties(
            &qobject.properties,
            &qobject_idents,
            type_names,
            structured_qobject,
            opt,
        )?);
        generated.blocks.append(&mut generate_cpp_methods(
            &structured_qobject.methods,
            type_names,
            opt,
        )?);
        generated.blocks.append(&mut generate_cpp_signals(
            &structured_qobject.signals,
            &qobject_idents,
            type_names,
            opt,
        )?);

        generated.blocks.append(&mut inherit::generate(
            &structured_qobject.inherited_methods,
            &qobject.base_class.as_ref().map(|ident| ident.to_string()),
            type_names,
            opt,
        )?);
        generated.blocks.append(&mut qenum::generate_on_qobject(
            structured_qobject.qenums.iter().cloned(),
            &generated.name,
            opt,
        )?);

        let mut class_initializers = vec![];

        // If this type has threading enabled then add generation
        //
        // Note that threading also includes locking C++ generation
        if structured_qobject.threading {
            let (initializer, mut blocks) = threading::generate(&qobject_idents)?;
            generated.blocks.append(&mut blocks);
            class_initializers.push(initializer);
        }

        // Include casting header
        let mut result = GeneratedCppQObjectBlocks::default();
        result.includes.insert("#include <cxx-qt/casting.h>".into());

        generated.blocks.append(&mut result);

        generated.blocks.append(&mut constructor::generate(
            &generated,
            &structured_qobject.constructors,
            base_class,
            &class_initializers,
            type_names,
        )?);

        Ok(generated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::mock_qml_singleton;
    use crate::{generator::structuring::Structures, parser::Parser};
    use quote::format_ident;
    use syn::{parse_quote, ItemMod};

    #[test]
    fn test_generated_cpp_qobject_blocks() {
        let module: ItemMod = parse_quote! {
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

        let cpp = GeneratedCppQObject::from(
            structures.qobjects.first().unwrap(),
            &TypeNames::mock(),
            &GeneratedOpt::default(),
        )
        .unwrap();
        assert_eq!(cpp.name.cxx_unqualified(), "MyObject");
        assert_eq!(cpp.rust_struct.cxx_unqualified(), "MyObjectRust");
        assert_eq!(cpp.namespace_internals, "cxx_qt_MyObject");

        assert_eq!(cpp.blocks.base_classes.len(), 2);
        assert_eq!(cpp.blocks.base_classes[0], "public QObject");
        assert_eq!(
            cpp.blocks.base_classes[1],
            "private ::rust::cxxqt1::CxxQtType<MyObjectRust>"
        );
        assert_eq!(cpp.blocks.metaobjects.len(), 0);
    }

    #[test]
    fn test_generated_cpp_qobject_blocks_base_and_namespace() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    #[base = QStringListModel]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();
        let mut type_names = TypeNames::mock();

        type_names.mock_insert(
            "QStringListModel",
            Some(format_ident!("qobject")),
            None,
            None,
        );

        let cpp = GeneratedCppQObject::from(
            structures.qobjects.first().unwrap(),
            &type_names,
            &GeneratedOpt::default(),
        )
        .unwrap();
        assert_eq!(cpp.namespace_internals, "cxx_qt::cxx_qt_MyObject");
        assert_eq!(cpp.blocks.base_classes.len(), 2);
        assert_eq!(cpp.blocks.base_classes[0], "public QStringListModel");
        assert_eq!(
            cpp.blocks.base_classes[1],
            "private ::rust::cxxqt1::CxxQtType<MyObjectRust>"
        );
        assert_eq!(cpp.blocks.metaobjects.len(), 0);
    }

    #[test]
    fn test_generated_cpp_qobject_named() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    #[qml_element = "MyQmlElement"]
                    type MyNamedObject = super::MyNamedObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        let mut type_names = TypeNames::default();
        type_names.mock_insert("MyNamedObject", None, None, None);
        type_names.mock_insert("MyNamedObjectRust", None, None, None);
        let cpp = GeneratedCppQObject::from(
            structures.qobjects.first().unwrap(),
            &type_names,
            &GeneratedOpt::default(),
        )
        .unwrap();
        assert_eq!(cpp.name.cxx_unqualified(), "MyNamedObject");
        assert_eq!(cpp.blocks.metaobjects.len(), 1);
        assert_eq!(cpp.blocks.metaobjects[0], "QML_NAMED_ELEMENT(MyQmlElement)");
    }

    #[test]
    fn test_generated_cpp_qobject_singleton() {
        let module = mock_qml_singleton();
        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        let cpp = GeneratedCppQObject::from(
            structures.qobjects.first().unwrap(),
            &TypeNames::mock(),
            &GeneratedOpt::default(),
        )
        .unwrap();
        assert_eq!(cpp.name.cxx_unqualified(), "MyObject");
        assert_eq!(cpp.blocks.metaobjects.len(), 2);
        assert_eq!(cpp.blocks.metaobjects[0], "QML_NAMED_ELEMENT(MyObject)");
        assert_eq!(cpp.blocks.metaobjects[1], "QML_SINGLETON");
    }

    #[test]
    fn test_generated_cpp_qobject_uncreatable() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    #[qml_element]
                    #[qml_uncreatable]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();
        let structures = Structures::new(&parser.cxx_qt_data).unwrap();

        let cpp = GeneratedCppQObject::from(
            structures.qobjects.first().unwrap(),
            &TypeNames::mock(),
            &GeneratedOpt::default(),
        )
        .unwrap();
        assert_eq!(cpp.name.cxx_unqualified(), "MyObject");
        assert_eq!(cpp.blocks.metaobjects.len(), 2);
        assert_eq!(cpp.blocks.metaobjects[0], "QML_NAMED_ELEMENT(MyObject)");
        assert_eq!(
            cpp.blocks.metaobjects[1],
            "QML_UNCREATABLE(\"Type cannot be created in QML.\")"
        );
    }
}
