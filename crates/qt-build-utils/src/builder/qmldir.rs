// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::builder::qmluri::QmlUriBuilder;

/// QML module definition files builder
///
/// A qmldir file is a plain-text file that contains the commands
pub struct QmlDirBuilder {
    class_name: Option<String>,
    plugin: Option<(bool, String)>,
    type_info: Option<String>,
    uri: QmlUriBuilder,
}

impl QmlDirBuilder {
    /// Construct a [QmlDirBuilder] using the give [QmlUriBuilder] for the
    /// module identifier
    pub fn new(uri: QmlUriBuilder) -> Self {
        Self {
            class_name: None,
            plugin: None,
            type_info: None,
            uri,
        }
    }

    /// Build the resultant qmldir text file contents
    pub fn build(self) -> String {
        // Module is mandatory
        let module = format!("module {}", self.uri.as_dots());

        // Plugin, classname, and typeinfo are optional
        let plugin = self
            .plugin
            .map(|(optional, name)| {
                if optional {
                    format!("optional plugin {name}")
                } else {
                    format!("plugin {name}")
                }
            })
            .unwrap_or_default();
        let class_name = self
            .class_name
            .map(|name| format!("classname {name}"))
            .unwrap_or_default();
        let type_info = self
            .type_info
            .map(|file| format!("typeinfo {file}"))
            .unwrap_or_default();
        let prefer = format!("prefer :/qt/qml/{}/", self.uri.as_dirs());

        format!(
            "{module}
{plugin}
{class_name}
{type_info}
{prefer}
"
        )
    }

    /// Provides the class name of the C++ plugin used by the module.
    ///
    /// This information is required for all the QML modules that depend on a
    /// C++ plugin for additional functionality. Qt Quick applications built
    /// with static linking cannot resolve the module imports without this
    /// information.
    //
    // TODO: is required for C++ plugins, is it required when plugin?
    pub fn class_name(mut self, class_name: String) -> Self {
        self.class_name = Some(class_name);
        self
    }

    /// Declares a plugin to be made available by the module.
    ///
    /// optional denotes that the plugin itself does not contain any relevant code
    /// and only serves to load a library it links to. If given, and if any types
    /// for the module are already available, indicating that the library has been
    /// loaded by some other means, QML will not load the plugin.
    ///
    /// name is the plugin library name. This is usually not the same as the file
    /// name of the plugin binary, which is platform dependent. For example, the
    /// library MyAppTypes would produce libMyAppTypes.so on Linux and MyAppTypes.dll
    /// on Windows.
    ///
    /// Zero or more C++ plugin declarations may exist in the qmldir file.
    /// However, since plugin loading is a relatively expensive operation, clients
    /// are advised to specify at most a single plugin.
    //
    // TODO: Zero or more but only one is recommended so is enforcing one ok for now?
    pub fn plugin(mut self, name: String, optional: bool) -> Self {
        self.plugin = Some((optional, name));
        self
    }

    /// Declares a type description file for the module that can be read by QML
    /// tools such as Qt Creator to access information about the types defined
    /// by the module's plugins. File is the (relative) file name of a
    /// .qmltypes file.
    pub fn type_info(mut self, file: String) -> Self {
        self.type_info = Some(file);
        self
    }

    // TODO: add further optional entries
    // object type declaration
    // internal object type declaration
    // javascript resource definition
    // module dependencies declaration
    // module import declaration
    // designer support declaration
}
