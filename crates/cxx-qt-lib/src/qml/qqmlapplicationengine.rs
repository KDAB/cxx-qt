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

        include!("cxx-qt-lib/qqmlapplicationengine.h");
        type QQmlApplicationEngine;

        /// Adds path as a directory where the engine searches for installed modules in a URL-based directory structure.
        #[rust_name = "add_import_path"]
        fn addImportPath(self: Pin<&mut QQmlApplicationEngine>, path: &QString);

        /// Adds path as a directory where the engine searches for native plugins for imported modules (referenced in the qmldir file).
        #[rust_name = "add_plugin_path"]
        fn addPluginPath(self: Pin<&mut QQmlApplicationEngine>, path: &QString);

        /// Return the base URL for this engine.
        /// The base URL is only used to resolve components when a relative URL is passed to the QQmlComponent constructor.
        #[rust_name = "base_url"]
        fn baseUrl(self: &QQmlApplicationEngine) -> QUrl;

        /// Returns the list of directories where the engine searches for installed modules in a URL-based directory structure.
        #[rust_name = "import_path_list"]
        fn importPathList(self: &QQmlApplicationEngine) -> QStringList;

        /// Loads the root QML file located at url.
        fn load(self: Pin<&mut QQmlApplicationEngine>, url: &QUrl);

        /// This property holds the directory for storing offline user data
        #[rust_name = "offline_storage_path"]
        fn offlineStoragePath(self: &QQmlApplicationEngine) -> QString;

        /// Returns the list of directories where the engine searches for native plugins for imported modules (referenced in the qmldir file).
        #[rust_name = "plugin_path_list"]
        fn pluginPathList(self: &QQmlApplicationEngine) -> QStringList;

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

    unsafe extern "C++" {
        include!("cxx-qt-lib/qqmlengine.h");
        type QQmlEngine = crate::QQmlEngine;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        type c_void = crate::c_void;

        #[doc(hidden)]
        #[rust_name = "qqmlapplicationengine_new"]
        fn qqmlapplicationengineNew() -> UniquePtr<QQmlApplicationEngine>;

        #[doc(hidden)]
        #[rust_name = "qqmlapplicationengine_as_qqmlengine"]
        fn qqmlapplicationengineAsQQmlEngine(
            ptr: Pin<&mut QQmlApplicationEngine>,
        ) -> Pin<&mut QQmlEngine>;
    }

    #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_5))]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qanystringview.h");
        type QAnyStringView<'a> = crate::QAnyStringView<'a>;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qqmlapplicationengine_singleton_instance"]
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_5))]
        fn qqmlapplicationengineSingletonInstance(
            ptr: Pin<&mut QQmlApplicationEngine>,
            uri: QAnyStringView,
            typeName: QAnyStringView,
        ) -> *mut c_void;
    }

    // QQmlApplicationEngine is not a trivial to CXX and is not relocatable in Qt
    // as the following fails in C++. So we cannot mark it as a trivial type
    // and need to use references or pointers.
    // static_assert(QTypeInfo<QQmlApplicationEngine>::isRelocatable);
    impl UniquePtr<QQmlApplicationEngine> {}
}

#[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_5))]
use crate::QAnyStringView;
use crate::QQmlEngine;
use core::pin::Pin;

pub use ffi::QQmlApplicationEngine;

impl QQmlApplicationEngine {
    /// Convert the existing [QQmlApplicationEngine] to a [QQmlEngine]
    pub fn as_qqmlengine<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut QQmlEngine> {
        ffi::qqmlapplicationengine_as_qqmlengine(self)
    }

    /// Create a new QQmlApplicationEngine
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qqmlapplicationengine_new()
    }

    /// Returns the instance of a singleton type named typeName from the module specified by uri.
    /// This is inherently unsafe as it does not perform any type checks.
    /// This function was introduced in Qt 6.5.
    #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_5))]
    pub unsafe fn singleton_instance<'a, T>(
        self: Pin<&'a mut Self>,
        uri: QAnyStringView,
        type_name: QAnyStringView,
    ) -> Option<Pin<&'a mut T>> {
        let ptr = ffi::qqmlapplicationengine_singleton_instance(self, uri, type_name);
        if ptr.is_null() {
            return None;
        }
        Some(Pin::new_unchecked(&mut *(ptr as *mut T)))
    }
}
