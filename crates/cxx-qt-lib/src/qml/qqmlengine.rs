// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/qqmlengine.h");
        /// The `QQmlEngine` class provides an environment for instantiating QML components.
        ///
        /// Qt Documentation: [QQmlEngine](https://doc.qt.io/qt/qqmlengine.html#details)
        #[qobject]
        type QQmlEngine;

        /// This signal is emitted when the QML loaded by the engine would like to exit from the event loop with the specified return code `ret_code`.
        #[qsignal]
        fn exit(self: Pin<&mut QQmlEngine>, ret_code: i32);

        /// This signal is emitted when the QML loaded by the engine would like to quit.
        #[qsignal]
        fn quit(self: Pin<&mut QQmlEngine>);
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = crate::QUrl;

        /// Adds `path` as a directory where the engine searches for installed modules in a URL-based directory structure.
        ///
        /// The path may be a local filesystem directory, a Qt Resource path (`:/imports`), a Qt Resource url (`qrc:/imports`) or a URL.
        ///
        /// The path will be converted into canonical form before it is added to the import path list.
        ///
        /// The newly added `path` will be first in the [`import_path_list`](Self::import_path_list).
        #[rust_name = "add_import_path"]
        fn addImportPath(self: Pin<&mut QQmlEngine>, path: &QString);

        /// Adds `path` as a directory where the engine searches for native plugins for imported modules (referenced in the `qmldir` file).
        ///
        /// The newly added `path` will be first in the [`plugin_path_list`](Self::plugin_path_list).
        #[rust_name = "add_plugin_path"]
        fn addPluginPath(self: Pin<&mut QQmlEngine>, path: &QString);

        /// Return the base URL for this engine.
        /// The base URL is only used to resolve components when a relative URL is passed to the [QQmlComponent](https://doc.qt.io/qt/qqmlcomponent.html) constructor.
        #[rust_name = "base_url"]
        fn baseUrl(self: &QQmlEngine) -> QUrl;

        /// Returns the list of directories where the engine searches for installed modules in a URL-based directory structure.
        #[rust_name = "import_path_list"]
        fn importPathList(self: &QQmlEngine) -> QStringList;

        /// Returns the directory for storing offline user data. Returns the directory where SQL and other offline storage is placed.
        ///
        /// The SQL databases created with [openDatabaseSync](https://doc.qt.io/qt/qtquick-localstorage-qmlmodule.html#opendatabasesync)() are stored here.
        ///
        /// The default is QML/OfflineStorage in the platform-standard user application data directory.
        ///
        /// Note that the path may not currently exist on the filesystem, so callers wanting to create new files at this location should create it first.
        #[rust_name = "offline_storage_path"]
        fn offlineStoragePath(self: &QQmlEngine) -> QString;

        /// Returns `true` if warning messages will be output to stderr in addition to being emitted by the [warnings](https://doc.qt.io/qt/qqmlengine.html#warnings)() signal, otherwise `false`.
        #[rust_name = "output_warnings_to_standard_error"]
        fn outputWarningsToStandardError(self: &QQmlEngine) -> bool;

        /// Returns the list of directories where the engine searches for native plugins for imported modules (referenced in the `qmldir` file).
        ///
        /// By default, the list contains only `.`, i.e. the engine searches in the directory of the `qmldir` file itself.
        #[rust_name = "plugin_path_list"]
        fn pluginPathList(self: &QQmlEngine) -> QStringList;

        /// Set the base URL for this engine to `url`.
        #[rust_name = "set_base_url"]
        fn setBaseUrl(self: Pin<&mut QQmlEngine>, url: &QUrl);

        /// Sets `paths` as the list of directories where the engine searches for installed modules in a URL-based directory structure.
        ///
        /// By default, this list contains the paths mentioned in [QML Import Path](https://doc.qt.io/qt/qtqml-syntax-imports.html#qml-import-path).
        ///
        /// **Warning:** Calling this function does not preserve the default import paths.
        #[rust_name = "set_import_path_list"]
        fn setImportPathList(self: Pin<&mut QQmlEngine>, paths: &QStringList);

        /// Returns the directory for storing offline user data.
        #[rust_name = "set_offline_storage_path"]
        fn setOfflineStoragePath(self: Pin<&mut QQmlEngine>, dir: &QString);

        /// Set whether warning messages will be output to stderr to `enabled`.
        #[rust_name = "set_output_warnings_to_standard_error"]
        fn setOutputWarningsToStandardError(self: Pin<&mut QQmlEngine>, enabled: bool);

        /// Sets the list of directories where the engine searches for native plugins for imported modules (referenced in the `qmldir` file) to `paths`.
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
    /// Create a new `QQmlEngine`.
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qqmlengine_new()
    }
}
