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
        include!(<QQmlImageProviderBase>);
        type QQmlImageProviderBase;
        include!(<QtQuick/QQuickImageProvider>);
        type QQuickImageProvider;
        include!(<QtQuick/QQuickAsyncImageProvider>);
        type QQuickAsyncImageProvider;

        include!("cxx-qt-lib/qqmlapplicationengine.h");
        type QQmlApplicationEngine;

        #[doc(hidden)]
        #[rust_name = "add_image_provider_internal"]
        unsafe fn addImageProvider(self: Pin<&mut QQmlApplicationEngine>, provider_id: &QString, provider: *mut QQmlImageProviderBase);

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
        #[doc(hidden)]
        #[rust_name = "qqmlapplicationengine_new"]
        fn qqmlapplicationengineNew() -> UniquePtr<QQmlApplicationEngine>;

        #[doc(hidden)]
        #[rust_name = "qqmlapplicationengine_as_qqmlengine"]
        fn qqmlapplicationengineAsQQmlEngine(
            ptr: Pin<&mut QQmlApplicationEngine>,
        ) -> Pin<&mut QQmlEngine>;
    }

    // QQmlApplicationEngine is not a trivial to CXX and is not relocatable in Qt
    // as the following fails in C++. So we cannot mark it as a trivial type
    // and need to use references or pointers.
    // static_assert(QTypeInfo<QQmlApplicationEngine>::isRelocatable);
    impl UniquePtr<QQmlApplicationEngine> {}
}

use crate::QQmlEngine;
use core::pin::Pin;

pub use ffi::QQmlApplicationEngine;
use crate::QString;
use ffi::QQmlImageProviderBase;

#[allow(dead_code)]
pub enum QQmlImageProviderBasePointer {
    QQuickImageProvider(*mut ffi::QQuickImageProvider),
    QQuickAsyncImageProvider(*mut ffi::QQuickAsyncImageProvider)
}

impl QQmlApplicationEngine {
    /// Convert the existing [QQmlApplicationEngine] to a [QQmlEngine]
    pub fn as_qqmlengine<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut QQmlEngine> {
        ffi::qqmlapplicationengine_as_qqmlengine(self)
    }

    /// Create a new QQmlApplicationEngine
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qqmlapplicationengine_new()
    }

    /// Sets the provider to use for images requested via the image: url scheme, with host proveri_id. The QQmlEngine takes ownership of provider.
    pub unsafe fn add_image_provider(self: Pin<&mut QQmlApplicationEngine>, provider_id: &QString, provider: QQmlImageProviderBasePointer) {
        let ptr = match provider {
            QQmlImageProviderBasePointer::QQuickAsyncImageProvider(ptr) => {
                ptr as *mut QQmlImageProviderBase
            },
            QQmlImageProviderBasePointer::QQuickImageProvider(ptr) => {
                ptr as *mut QQmlImageProviderBase
            }
        };
        self.add_image_provider_internal(provider_id, ptr);
    }
}
