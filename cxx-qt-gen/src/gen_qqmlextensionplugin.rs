// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use clang_format::clang_format;
use convert_case::{Case, Casing};
use indoc::formatdoc;

use crate::extract::QObject;

/// Describes a C++ type that is being registered to QML with a specific version
struct QMLType {
    /// The name of C++ type which is being registered
    cpp_type: String,
    /// The name to use for QML when registering the type
    qml_name: String,
    // FIXME: can versions be negative? C++ type is just int?
    // TODO: what happens with Qt 6 here?
    /// The major version to register the QML type as
    version_major: u32,
    /// The minor version to register the QML type as
    version_minor: u32,
}

impl QMLType {
    /// Returns the include line for this QMLType
    fn as_include(&self) -> String {
        format!(
            "#include \"cxx-qt-gen/include/{ident_snake}.h\"",
            ident_snake = self.cpp_type.to_case(Case::Snake)
        )
    }

    /// Returns the C++ qmlRegisterType line for this QMLType
    fn as_qml_register_type(&self) -> String {
        format!(
            "qmlRegisterType<{cpp_type}>(uri, {version_major}, {version_minor}, \"{qml_name}\");",
            cpp_type = self.cpp_type,
            qml_name = self.qml_name,
            version_major = self.version_major,
            version_minor = self.version_minor
        )
    }
}

/// Holds meta data relating to QQmlExtensionPlugin
pub struct QQmlExtensionPluginData {
    /// Module identifier of the plugin that matches the folder it is in
    module_identifier: &'static str,
    /// The name of the library file (eg .dll or .so)
    cpp_plugin_name: &'static str,
    /// The name of the C++ class in the library file
    cpp_class_name: String,
    /// The list of QML types to register in this module
    register_types: Vec<QMLType>,
    // TODO: add support for register_singleton_types
}

impl QQmlExtensionPluginData {
    /// Constructs a QQmlExtensionPluginData with the given module identifier and C++ plugin name
    pub fn new(module_identifier: &'static str, cpp_plugin_name: &'static str) -> Self {
        Self {
            cpp_plugin_name,
            // Generate the class name from the plugin name to attempt to avoid collisions
            cpp_class_name: format!("{}Plugin", cpp_plugin_name.to_case(Case::Pascal)),
            module_identifier,
            register_types: vec![],
        }
    }

    /// Adds a given QObject as a QML type that will be registered in the QQmlExtensionPlugin
    pub fn push_type(&mut self, object: &QObject) {
        self.register_types.push(QMLType {
            cpp_type: object.ident.to_string(),
            qml_name: object.ident.to_string(),
            // TODO: these should be optionally read from the macro attributes
            // eg #[make_qobject(version_major=1, version_minor=0)]
            version_major: 1,
            version_minor: 0,
        });
    }

    /// Generate the contents of the qmldir for this QQmlExtensionPluginData
    pub fn gen_qmldir(&self) -> String {
        formatdoc! {
            r#"
            module {module_identifier}
            plugin {cpp_plugin_name}
            class_name {cpp_class_name}
            "#,
            module_identifier = self.module_identifier,
            cpp_plugin_name = self.cpp_plugin_name,
            cpp_class_name = self.cpp_class_name,
        }
    }

    /// Generate the contents of the plugin.cpp for this QQmlExtensionPluginData
    pub fn gen_qqmlextensionplugin(&self) -> String {
        let plugin = formatdoc! {
            r#"
            #include <QQmlEngine>
            #include <QQmlExtensionPlugin>

            {type_includes}

            class {cpp_class_name} : public QQmlExtensionPlugin
            {{
                Q_OBJECT
                Q_PLUGIN_METADATA(IID QQmlExtensionInterface_iid)

            public:
                void registerTypes(const char *uri) override
                {{
                    {qml_register_types}
                }}
            }};

            #include "plugin.moc"
            "#,
            cpp_class_name = self.cpp_class_name,
            qml_register_types = self.register_types.iter().map(|data| data.as_qml_register_type()).collect::<Vec<String>>().join("\n"),
            type_includes = self.register_types.iter().map(|data| data.as_include()).collect::<Vec<String>>().join("\n"),
        };

        clang_format(&plugin).unwrap_or(plugin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::extract_qobject;

    use pretty_assertions::assert_eq;
    use syn::ItemMod;

    #[test]
    fn generates_basic_qmldir() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_qmldir = include_str!("../test_outputs/basic_qmldir");

        let mut plugin = QQmlExtensionPluginData::new("MODULE_IDENT", "CPP_PLUGIN_NAME");
        plugin.push_type(&qobject);
        let actual_qmldir = plugin.gen_qmldir();

        assert_eq!(actual_qmldir, expected_qmldir);
    }

    #[test]
    fn generates_basic_qqmlextensionplugin() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_source = clang_format(include_str!(
            "../test_outputs/basic_qqmlextensionplugin.cpp"
        ))
        .unwrap();

        let mut plugin = QQmlExtensionPluginData::new("MODULE_IDENT", "CPP_PLUGIN_NAME");
        plugin.push_type(&qobject);
        let actual_qqmlextensionplugin = plugin.gen_qqmlextensionplugin();

        assert_eq!(actual_qqmlextensionplugin, expected_source);
    }
}
