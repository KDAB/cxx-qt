// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This Rust module contains structs for registering QML modules.

pub use qt_build_utils::{PluginType, QmlFile, QmlUri};

/// This is a description of a QML module for building by the [crate::CxxQtBuilder].
///
/// It allows registering QML files that will be included in the QML module.
/// For further resources such as images, these can be added to the Qt resources
/// system via the appropriate CxxQtBuilder functions.
#[must_use = "The QML module only does anything if it is passed to CxxQtBuilder::qml_module"]
pub struct QmlModule {
    pub(crate) uri: QmlUri,
    pub(crate) version_major: usize,
    pub(crate) version_minor: usize,
    pub(crate) qml_files: Vec<QmlFile>,
    pub(crate) depends: Vec<String>,
    pub(crate) plugin_type: PluginType,
}

impl QmlModule {
    /// Create a new [QmlModule] with the given URI.
    ///
    /// The default version is 1.0.
    pub fn new(uri: impl Into<QmlUri>) -> Self {
        Self {
            uri: uri.into(),
            version_major: 1,
            version_minor: 0,
            qml_files: Vec::new(),
            depends: Vec::new(),
            plugin_type: PluginType::Static,
        }
    }

    /// Add a QML module dependency
    pub fn depend(mut self, depend: impl Into<QmlUri>) -> Self {
        self.depends.push(depend.into());
        self
    }

    /// Add multiple QML module dependencies
    pub fn depends<T: Into<QmlUri>>(mut self, depends: impl IntoIterator<Item = T>) -> Self {
        self.depends.extend(depends.into_iter().map(Into::into));
        self
    }

    /// Add a version to the QML module.
    pub fn version(mut self, version_major: usize, version_minor: usize) -> Self {
        self.version_major = version_major;
        self.version_minor = version_minor;
        self
    }

    /// Specify the plugin type for the QML module ([`PluginType::Static`] by default).
    ///
    /// Warning: The following limitations apply to building QML modules with [`PluginType::Dynamic`]:
    ///
    /// ### Crate must be built to cdylib
    ///
    /// Even though it is possible to build both a `staticlib` and `cdylib` from one crate, any
    /// crate that uses [`PluginType::Dynamic`] for the QML module must build to `cdylib`.
    /// The QML module will not work as expected if built into a `staticlib`.
    ///
    /// ### Only One Dynamic Plugin Per Library
    ///
    /// There can only be one dynamic QML module plugin per dynamic library.
    /// This also applies to sub-crates, so no sub-crate in the dependency tree can
    /// build to a dynamic QML module plugin if the main crate is already doing so.
    ///
    /// ### Final binary should be built with CXX-Qt-CMake
    ///
    /// Any binary that loads a dynamic QML module plugin expects a certain file layout of the qmldir
    /// and dynamic library files. The easiest way to ensure this is to build the final binary
    /// with
    /// [CXX-Qt-CMake](https://kdab.github.io/cxx-qt/book/getting-started/5-cmake-integration.html).
    /// CXX-Qt does not currently provide any way to generate the required file layout with pure
    /// Cargo builds. Prefer building static QML module plugins when building with Cargo only.
    ///
    /// ### MSVC Runtime Compatibility on Windows
    ///
    /// As outlined in the CXX-Qt book, Rust always links to the release runtime under MSVC on Windows.
    /// To ensure compatibility when loading dynamic QML module plugins, CXX-Qt will set the
    /// QT_NO_DEBUG definition when compiling the C++ code for dynamic QML module plugins **under
    /// MSVC**.
    /// This will disable debug features in Qt, even when building a debug build of the Rust crate.
    pub fn plugin_type(mut self, plugin_type: PluginType) -> Self {
        self.plugin_type = plugin_type;
        self
    }

    /// Add a single QML file to the module.
    ///
    /// The [crate::CxxQtBuilder] will register the file with the [Qt Resource System](https://doc.qt.io/qt-6/resources.html) in
    /// the [default QML import path](https://doc.qt.io/qt-6/qtqml-syntax-imports.html#qml-import-path) `qrc:/qt/qml/uri/of/module/`.
    ///
    /// When using Qt 6, the [crate::CxxQtBuilder] will [run qmlcachegen](https://doc.qt.io/qt-6/qtqml-qtquick-compiler-tech.html)
    /// to compile the specified `.qml` file ahead-of-time.
    ///
    /// Additional resources such as images can be added to the Qt resources for the QML module by specifying
    /// the `qrc_files` field.
    ///
    /// If the Qml file starts is uppercase, it will be treated as a QML component and registered in the `qmldir` file.
    /// See [qt_build_utils::QmlFile] for more information on configuring the behavior of QML files.
    ///
    /// Note that if no version is specified for the QML file, the version of the QML module will
    /// be used automatically.
    pub fn qml_file(self, file: impl Into<QmlFile>) -> Self {
        self.qml_files([file])
    }

    /// Add multiple QML files to the module, see [Self::qml_file].
    pub fn qml_files(mut self, files: impl IntoIterator<Item = impl Into<QmlFile>>) -> Self {
        self.qml_files.extend(files.into_iter().map(|p| {
            let qml_file = p.into();
            if qml_file.get_version().is_none() {
                qml_file.version(self.version_major, self.version_minor)
            } else {
                qml_file
            }
        }));
        self
    }
}
