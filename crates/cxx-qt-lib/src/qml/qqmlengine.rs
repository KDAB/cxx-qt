// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = crate::QUrl;

        include!("cxx-qt-lib/qqmlengine.h");
        type QQmlEngine;

        /// Adds path as a directory where the engine searches for installed modules in a URL-based directory structure.
        #[rust_name = "add_import_path"]
        fn addImportPath(self: Pin<&mut QQmlEngine>, path: &QString);

        /// Adds path as a directory where the engine searches for native plugins for imported modules (referenced in the qmldir file).
        #[rust_name = "add_plugin_path"]
        fn addPluginPath(self: Pin<&mut QQmlEngine>, path: &QString);

        /// Return the base URL for this engine.
        /// The base URL is only used to resolve components when a relative URL is passed to the QQmlComponent constructor.
        #[rust_name = "base_url"]
        fn baseUrl(self: &QQmlEngine) -> QUrl;

        /// Returns the list of directories where the engine searches for installed modules in a URL-based directory structure.
        #[rust_name = "import_path_list"]
        fn importPathList(self: &QQmlEngine) -> QStringList;

        /// This property holds the directory for storing offline user data. Returns the directory where SQL and other offline storage is placed.
        #[rust_name = "offline_storage_path"]
        fn offlineStoragePath(self: &QQmlEngine) -> QString;

        /// Returns true if warning messages will be output to stderr in addition to being emitted by the warnings() signal, otherwise false.
        #[rust_name = "output_warnings_to_standard_error"]
        fn outputWarningsToStandardError(self: &QQmlEngine) -> bool;

        /// Returns the list of directories where the engine searches for native plugins for imported modules (referenced in the qmldir file).
        #[rust_name = "plugin_path_list"]
        fn pluginPathList(self: &QQmlEngine) -> QStringList;

        /// Set the base URL for this engine to url.
        #[rust_name = "set_base_url"]
        fn setBaseUrl(self: Pin<&mut QQmlEngine>, url: &QUrl);

        /// Sets paths as the list of directories where the engine searches for installed modules in a URL-based directory structure.
        #[rust_name = "set_import_path_list"]
        fn setImportPathList(self: Pin<&mut QQmlEngine>, paths: &QStringList);

        /// This property holds the directory for storing offline user data
        #[rust_name = "set_offline_storage_path"]
        fn setOfflineStoragePath(self: Pin<&mut QQmlEngine>, dir: &QString);

        /// Set whether warning messages will be output to stderr to enabled.
        #[rust_name = "set_output_warnings_to_standard_error"]
        fn setOutputWarningsToStandardError(self: Pin<&mut QQmlEngine>, enabled: bool);

        /// Sets the list of directories where the engine searches for native plugins for imported modules (referenced in the qmldir file) to paths.
        #[rust_name = "set_plugin_path_list"]
        fn setPluginPathList(self: Pin<&mut QQmlEngine>, paths: &QStringList);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qqmlengine_new"]
        fn qqmlengineNew() -> UniquePtr<QQmlEngine>;
    }

    // QQmlEngine is not a trivial to CXX and is not relocatable in Qt
    // as the following fails in C++. So we cannot mark it as a trivial type
    // and need to use references or pointers.
    // static_assert(QTypeInfo<QQmlEngine>::isRelocatable);
    impl UniquePtr<QQmlEngine> {}
}

pub use ffi::QQmlEngine;

impl QQmlEngine {
    /// Create a new QQmlEngine
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qqmlengine_new()
    }
}
