// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::QQmlEngine;
use cxx_qt::Upcast;
use std::ops::Deref;

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
        type QQmlEngine = crate::QQmlEngine;

        include!("cxx-qt-lib/qqmlapplicationengine.h");
        type QQmlApplicationEngine;

        /// Adds path as a directory where the engine searches for installed modules in a URL-based directory structure.
        #[rust_name = "add_import_path"]
        fn addImportPath(self: Pin<&mut QQmlApplicationEngine>, path: &QString);

        /// Adds path as a directory where the engine searches for native plugins for imported modules (referenced in the qmldir file).
        #[rust_name = "add_plugin_path"]
        fn addPluginPath(self: Pin<&mut QQmlApplicationEngine>, path: &QString);

        /// Loads the root QML file located at url.
        fn load(self: Pin<&mut QQmlApplicationEngine>, url: &QUrl);

        /// Set the base URL for this engine to url.
        #[rust_name = "set_base_url"]
        fn setBaseUrl(self: Pin<&mut QQmlApplicationEngine>, url: &QUrl);

        /// Sets paths as the list of directories where the engine searches for installed modules in a URL-based directory structure.
        #[rust_name = "set_import_path_list"]
        fn setImportPathList(self: Pin<&mut QQmlApplicationEngine>, paths: &QStringList);

        /// Sets the list of directories where the engine searches for native plugins for imported modules (referenced in the qmldir file) to paths.
        #[rust_name = "set_plugin_path_list"]
        fn setPluginPathList(self: Pin<&mut QQmlApplicationEngine>, paths: &QStringList);

        /// Sets path as string for storing offline user data.
        #[rust_name = "set_offline_storage_path"]
        fn setOfflineStoragePath(self: Pin<&mut QQmlApplicationEngine>, dir: &QString);
    }

    #[namespace = "rust::cxxqt1"]
    unsafe extern "C++" {
        include!("cxx-qt/casting.h");

        #[doc(hidden)]
        #[rust_name = "upcast_qqmlapplication_engine"]
        unsafe fn upcastPtr(thiz: *const QQmlApplicationEngine) -> *const QQmlEngine;

        #[doc(hidden)]
        #[rust_name = "downcast_qqml_engine"]
        unsafe fn downcastPtr(base: *const QQmlEngine) -> *const QQmlApplicationEngine;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qqmlapplicationengine_new"]
        fn qqmlapplicationengineNew() -> UniquePtr<QQmlApplicationEngine>;
    }

    // QQmlApplicationEngine is not a trivial to CXX and is not relocatable in Qt
    // as the following fails in C++. So we cannot mark it as a trivial type
    // and need to use references or pointers.
    // static_assert(QTypeInfo<QQmlApplicationEngine>::isRelocatable);
    impl UniquePtr<QQmlApplicationEngine> {}
}

pub use ffi::QQmlApplicationEngine;

impl Deref for QQmlApplicationEngine {
    type Target = QQmlEngine;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl Upcast<QQmlEngine> for QQmlApplicationEngine {
    unsafe fn upcast_ptr(this: *const Self) -> *const QQmlEngine {
        ffi::upcast_qqmlapplication_engine(this)
    }

    unsafe fn from_base_ptr(base: *const QQmlEngine) -> *const Self {
        ffi::downcast_qqml_engine(base)
    }
}

impl QQmlApplicationEngine {
    /// Create a new QQmlApplicationEngine
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qqmlapplicationengine_new()
    }
}
