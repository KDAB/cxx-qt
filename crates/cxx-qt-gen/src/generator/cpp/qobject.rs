// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::{
    cpp::{
        constructor, fragment::CppFragment, inherit, invokable::generate_cpp_invokables,
        property::generate_cpp_properties, signal::generate_cpp_signals, threading,
    },
    naming::{namespace::NamespaceName, qobject::QObjectName},
};
use crate::parser::{cxxqtdata::ParsedCxxMappings, qobject::ParsedQObject};
use syn::Result;

#[derive(Default)]
pub struct GeneratedCppQObjectBlocks {
    /// List of forward declares before the class and include of the generated CXX header
    pub forward_declares: Vec<String>,
    /// List of Qt Meta Object items (eg Q_PROPERTY)
    pub metaobjects: Vec<String>,
    /// List of public methods for the QObject
    pub methods: Vec<CppFragment>,
    /// List of private methods for the QObject
    pub private_methods: Vec<CppFragment>,
    /// List of members for the QObject
    pub members: Vec<String>,
    /// List of deconstructor source
    pub deconstructors: Vec<String>,
}

impl GeneratedCppQObjectBlocks {
    pub fn append(&mut self, other: &mut Self) {
        self.forward_declares.append(&mut other.forward_declares);
        self.metaobjects.append(&mut other.metaobjects);
        self.methods.append(&mut other.methods);
        self.private_methods.append(&mut other.private_methods);
        self.members.append(&mut other.members);
        self.deconstructors.append(&mut other.deconstructors);
    }

    pub fn from(qobject: &ParsedQObject) -> GeneratedCppQObjectBlocks {
        let mut qml_specifiers = Vec::new();
        if let Some(qml_metadata) = &qobject.qml_metadata {
            // Somehow moc doesn't include the info in metatypes.json that qmltyperegistrar needs
            // when using the QML_ELEMENT/QML_NAMED_ELEMENT macros, but moc works when using what
            // those macros expand to.
            qml_specifiers.push(format!(
                "Q_CLASSINFO(\"QML.Element\", \"{}\")",
                qml_metadata.name
            ));

            if qml_metadata.uncreatable {
                qml_specifiers.push("Q_CLASSINFO(\"QML.Creatable\", \"false\")".to_owned());
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

#[derive(Default)]
pub struct GeneratedCppQObject {
    /// Ident of the C++ QObject
    pub ident: String,
    /// Ident of the Rust object
    pub rust_ident: String,
    /// Ident of the namespace for CXX-Qt internals of the QObject
    pub namespace_internals: String,
    /// Base class of the QObject
    pub base_class: String,
    /// The blocks of the QObject
    pub blocks: GeneratedCppQObjectBlocks,
    /// Whether locking is enabled
    pub locking: bool,
}

impl GeneratedCppQObject {
    pub fn from(
        qobject: &ParsedQObject,
        cxx_mappings: &ParsedCxxMappings,
    ) -> Result<GeneratedCppQObject> {
        // Create the base object
        let qobject_idents = QObjectName::from(qobject);
        let namespace_idents = NamespaceName::from(qobject);
        let mut generated = GeneratedCppQObject {
            ident: qobject_idents.cpp_class.cpp.to_string(),
            rust_ident: qobject_idents.rust_struct.cpp.to_string(),
            namespace_internals: namespace_idents.internal,
            base_class: qobject
                .base_class
                .clone()
                .unwrap_or_else(|| "QObject".to_string()),
            blocks: GeneratedCppQObjectBlocks::from(qobject),
            locking: qobject.locking,
        };
        let lock_guard = if qobject.locking {
            Some("const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);")
        } else {
            None
        };

        // Generate methods for the properties, invokables, signals
        generated.blocks.append(&mut generate_cpp_properties(
            &qobject.properties,
            &qobject_idents,
            cxx_mappings,
            lock_guard,
        )?);
        generated.blocks.append(&mut generate_cpp_invokables(
            &qobject.invokables,
            &qobject_idents,
            cxx_mappings,
            lock_guard,
        )?);
        generated.blocks.append(&mut generate_cpp_signals(
            &qobject.signals,
            &qobject_idents,
            cxx_mappings,
            lock_guard,
        )?);
        generated.blocks.append(&mut inherit::generate(
            &qobject.inherited_methods,
            &qobject.base_class,
            cxx_mappings,
        )?);

        let mut member_initializers = if qobject.locking {
            vec!["m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())".to_string()]
        } else {
            vec![]
        };
        // If this type has threading enabled then add generation
        if qobject.threading {
            let (initializer, mut blocks) = threading::generate(&qobject_idents)?;
            generated.blocks.append(&mut blocks);
            member_initializers.push(initializer);
        }
        generated.blocks.append(&mut constructor::generate(
            &generated,
            &qobject.constructors,
            &member_initializers,
            cxx_mappings,
        )?);

        Ok(generated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::Parser;
    use syn::{parse_quote, ItemMod};

    #[test]
    fn test_generated_cpp_qobject_blocks() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge]
            mod ffi {
                extern "RustQt" {
                    #[cxx_qt::qobject]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();

        let cpp = GeneratedCppQObject::from(
            parser.cxx_qt_data.qobjects.values().next().unwrap(),
            &ParsedCxxMappings::default(),
        )
        .unwrap();
        assert_eq!(cpp.ident, "MyObject");
        assert_eq!(cpp.rust_ident, "MyObjectRust");
        assert_eq!(cpp.namespace_internals, "cxx_qt_my_object");
        assert_eq!(cpp.base_class, "QObject");
        assert_eq!(cpp.blocks.metaobjects.len(), 0);
    }

    #[test]
    fn test_generated_cpp_qobject_blocks_base_and_namespace() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[cxx_qt::qobject(base = "QStringListModel")]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();

        let cpp = GeneratedCppQObject::from(
            parser.cxx_qt_data.qobjects.values().next().unwrap(),
            &ParsedCxxMappings::default(),
        )
        .unwrap();
        assert_eq!(cpp.namespace_internals, "cxx_qt::cxx_qt_my_object");
        assert_eq!(cpp.base_class, "QStringListModel");
        assert_eq!(cpp.blocks.metaobjects.len(), 0);
    }

    #[test]
    fn test_generated_cpp_qobject_named() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[cxx_qt::qobject(qml_element = "MyQmlElement")]
                    type MyNamedObject = super::MyNamedObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();

        let cpp = GeneratedCppQObject::from(
            parser.cxx_qt_data.qobjects.values().next().unwrap(),
            &ParsedCxxMappings::default(),
        )
        .unwrap();
        assert_eq!(cpp.ident, "MyNamedObject");
        assert_eq!(cpp.blocks.metaobjects.len(), 1);
        assert_eq!(
            cpp.blocks.metaobjects[0],
            "Q_CLASSINFO(\"QML.Element\", \"MyQmlElement\")"
        );
    }

    #[test]
    fn test_generated_cpp_qobject_singleton() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[cxx_qt::qobject(qml_element, qml_singleton)]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();

        let cpp = GeneratedCppQObject::from(
            parser.cxx_qt_data.qobjects.values().next().unwrap(),
            &ParsedCxxMappings::default(),
        )
        .unwrap();
        assert_eq!(cpp.ident, "MyObject");
        assert_eq!(cpp.blocks.metaobjects.len(), 2);
        assert_eq!(
            cpp.blocks.metaobjects[0],
            "Q_CLASSINFO(\"QML.Element\", \"MyObject\")"
        );
        assert_eq!(cpp.blocks.metaobjects[1], "QML_SINGLETON");
    }

    #[test]
    fn test_generated_cpp_qobject_uncreatable() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[cxx_qt::qobject(qml_element, qml_uncreatable)]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();

        let cpp = GeneratedCppQObject::from(
            parser.cxx_qt_data.qobjects.values().next().unwrap(),
            &ParsedCxxMappings::default(),
        )
        .unwrap();
        assert_eq!(cpp.ident, "MyObject");
        assert_eq!(cpp.blocks.metaobjects.len(), 2);
        assert_eq!(
            cpp.blocks.metaobjects[0],
            "Q_CLASSINFO(\"QML.Element\", \"MyObject\")"
        );
        assert_eq!(
            cpp.blocks.metaobjects[1],
            "Q_CLASSINFO(\"QML.Creatable\", \"false\")"
        );
    }
}
