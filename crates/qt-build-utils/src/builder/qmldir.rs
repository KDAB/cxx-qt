// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::builder::qmluri::QmlUri;

use std::io;

/// QML module definition files builder
///
/// A qmldir file is a plain-text file that contains the commands
pub struct QmlDirBuilder {
    class_name: Option<String>,
    plugin: Option<(bool, String)>,
    type_info: Option<String>,
    uri: QmlUri,
}

impl QmlDirBuilder {
    /// Construct a [QmlDirBuilder] using the give [QmlUriBuilder] for the
    /// module identifier
    pub fn new(uri: QmlUri) -> Self {
        Self {
            class_name: None,
            plugin: None,
            type_info: None,
            uri,
        }
    }

    /// Writer the resultant qmldir text file contents
    pub fn write(self, writer: &mut impl io::Write) -> io::Result<()> {
        // Module is mandatory
        writeln!(writer, "module {}", self.uri.as_dots())?;

        // Plugin, classname, and typeinfo are optional
        if let Some((optional, name)) = self.plugin {
            if optional {
                writeln!(writer, "optional plugin {name}")?;
            } else {
                writeln!(writer, "plugin {name}")?;
            }
        }

        if let Some(name) = self.class_name {
            writeln!(writer, "classname {name}")?;
        }

        if let Some(file) = self.type_info {
            writeln!(writer, "typeinfo {file}")?;
        }

        // Prefer is always specified for now
        writeln!(writer, "prefer :/qt/qml/{}/", self.uri.as_dirs())
    }

    /// Provides the class name of the C++ plugin used by the module.
    ///
    /// This information is required for all the QML modules that depend on a
    /// C++ plugin for additional functionality. Qt Quick applications built
    /// with static linking cannot resolve the module imports without this
    /// information.
    //
    // TODO: is required for C++ plugins, is it required when plugin?
    pub fn class_name(mut self, class_name: impl Into<String>) -> Self {
        self.class_name = Some(class_name.into());
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
    /// Only zero or one plugin is supported, otherwise a panic will occur.
    pub fn plugin(mut self, name: impl Into<String>, optional: bool) -> Self {
        // Only support zero or one plugin for now
        // it is not recommended to have more than one anyway
        if self.plugin.is_some() {
            panic!("Only zero or one plugin is supported currently");
        }

        self.plugin = Some((optional, name.into()));
        self
    }

    /// Declares a type description file for the module that can be read by QML
    /// tools such as Qt Creator to access information about the types defined
    /// by the module's plugins. File is the (relative) file name of a
    /// .qmltypes file.
    pub fn type_info(mut self, file: impl Into<String>) -> Self {
        self.type_info = Some(file.into());
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn qml_dir() {
        let mut result = Vec::new();
        QmlDirBuilder::new(QmlUri::new(["com", "kdab"]))
            .class_name("C")
            .plugin("P", true)
            .type_info("T")
            .write(&mut result)
            .unwrap();
        assert_eq!(
            String::from_utf8(result).unwrap(),
            "module com.kdab
optional plugin P
classname C
typeinfo T
prefer :/qt/qml/com/kdab/
"
        );
    }
}
