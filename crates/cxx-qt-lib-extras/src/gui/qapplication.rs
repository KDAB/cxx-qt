// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::pin::Pin;
use cxx_qt_lib::{QByteArray, QFont, QString, QStringList, QVector};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;
        include!("cxx-qt-lib/qvector.h");
        type QVector_QByteArray = cxx_qt_lib::QVector<QByteArray>;
        include!("cxx-qt-lib/qfont.h");
        type QFont = cxx_qt_lib::QFont;

        include!("cxx-qt-lib-extras/qapplication.h");
        /// The `QApplication` class manages the GUI application's control flow and main settings.
        ///
        /// Qt Documentation: [QApplication](https://doc.qt.io/qt/qapplication.html#details)
        type QApplication;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qapplication_new"]
        fn qapplicationNew(args: &QVector_QByteArray) -> UniquePtr<QApplication>;
    }

    // These are all static, so we need to create bindings until CXX supports statics
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        // Reuse the templated versions from QCoreApplication
        include!("cxx-qt-lib/qcoreapplication.h");

        #[doc(hidden)]
        #[rust_name = "qapplication_add_library_path"]
        fn qapplicationAddLibraryPath(app: Pin<&mut QApplication>, path: &QString);
        #[doc(hidden)]
        #[rust_name = "qapplication_application_name"]
        fn qapplicationApplicationName(app: &QApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qapplication_remove_library_path"]
        fn qapplicationRemoveLibraryPath(app: &QApplication, path: &QString);
        #[doc(hidden)]
        #[rust_name = "qapplication_application_version"]
        fn qapplicationApplicationVersion(app: &QApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qapplication_exec"]
        fn qapplicationExec(app: Pin<&mut QApplication>) -> i32;
        #[doc(hidden)]
        #[rust_name = "qapplication_library_paths"]
        fn qapplicationLibraryPaths(app: &QApplication) -> QStringList;
        #[doc(hidden)]
        #[rust_name = "qapplication_organization_domain"]
        fn qapplicationOrganizationDomain(app: &QApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qapplication_organization_name"]
        fn qapplicationOrganizationName(app: &QApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qapplication_set_application_name"]
        fn qapplicationSetApplicationName(app: Pin<&mut QApplication>, name: &QString);
        #[doc(hidden)]
        #[rust_name = "qapplication_set_application_version"]
        fn qapplicationSetApplicationVersion(app: Pin<&mut QApplication>, version: &QString);
        #[doc(hidden)]
        #[rust_name = "qapplication_set_font"]
        fn qapplicationSetFont(app: Pin<&mut QApplication>, font: &QFont);
        #[doc(hidden)]
        #[rust_name = "qapplication_font"]
        fn qapplicationFont(app: &QApplication) -> QFont;
        #[doc(hidden)]
        #[rust_name = "qapplication_set_library_paths"]
        fn qapplicationSetLibraryPaths(app: Pin<&mut QApplication>, paths: &QStringList);
        #[doc(hidden)]
        #[rust_name = "qapplication_set_organization_domain"]
        fn qapplicationSetOrganizationDomain(app: Pin<&mut QApplication>, domain: &QString);
        #[doc(hidden)]
        #[rust_name = "qapplication_set_organization_name"]
        fn qapplicationSetOrganizationName(app: Pin<&mut QApplication>, name: &QString);
    }

    // QApplication is not a trivial to CXX and is not relocatable in Qt
    // as the following fails in C++. So we cannot mark it as a trivial type
    // and need to use references or pointers.
    // static_assert(QTypeInfo<QApplication>::isRelocatable);
    impl UniquePtr<QApplication> {}
}

pub use ffi::QApplication;

impl QApplication {
    /// Prepends `path` to the beginning of the library path list,
    /// ensuring that it is searched for libraries first.
    /// If `path` is empty or already in the path list, the path list is not changed.
    pub fn add_library_path(self: Pin<&mut Self>, path: &QString) {
        ffi::qapplication_add_library_path(self, path);
    }

    /// Returns the name of this application.
    pub fn application_name(&self) -> QString {
        ffi::qapplication_application_name(self)
    }

    /// Returns the version of this application.
    pub fn application_version(&self) -> QString {
        ffi::qapplication_application_version(self)
    }

    /// Enters the main event loop and waits until [exit]\() is called,
    /// and then returns the value that was set to [exit]\() (which is 0 if [exit]\() is called via [quit]\()).
    ///
    /// [exit]: https://doc.qt.io/qt/qcoreapplication.html#exit
    /// [quit]: https://doc.qt.io/qt/qcoreapplication.html#quit
    pub fn exec(self: Pin<&mut Self>) -> i32 {
        ffi::qapplication_exec(self)
    }

    /// Returns the default application font.
    pub fn font(&self) -> QFont {
        ffi::qapplication_font(self)
    }

    /// Returns a list of paths that the application will search when dynamically loading libraries.
    pub fn library_paths(&self) -> QStringList {
        ffi::qapplication_library_paths(self)
    }

    /// Initializes the window system and constructs an application object.
    /// Standard [Qt command line arguments](https://doc.qt.io/qt/qguiapplication.html#supported-command-line-options) are handled automatically.
    pub fn new() -> cxx::UniquePtr<Self> {
        let mut vector = QVector::<QByteArray>::default();

        // Construct an owned QVector of the args
        // as we need the args_os data to outlive this method
        // so we pass a QVector to C++ which is then stored
        for arg in std::env::args_os() {
            // Unix OsStrings can be directly converted to bytes.
            #[cfg(unix)]
            use std::os::unix::ffi::OsStrExt;

            // Windows OsStrings are WTF-8 encoded, so they need to be
            // converted to UTF-8 Strings before being converted to bytes.
            // https://simonsapin.github.io/wtf-8/
            #[cfg(windows)]
            let arg = arg.to_string_lossy();

            vector.append(QByteArray::from(arg.as_bytes()));
        }

        ffi::qapplication_new(&vector)
    }

    /// Returns the Internet domain of the organization that wrote this application.
    pub fn organization_domain(&self) -> QString {
        ffi::qapplication_organization_domain(self)
    }

    /// Returns the name of the organization that wrote this application.
    pub fn organization_name(&self) -> QString {
        ffi::qapplication_organization_name(self)
    }

    /// Set the `name` of this application.
    pub fn set_application_name(self: Pin<&mut Self>, name: &QString) {
        ffi::qapplication_set_application_name(self, name);
    }

    /// Removes `path` from the library path list. If `path` is empty or not in the path list, the list is not changed.
    pub fn remove_library_path(&self, path: &QString) {
        ffi::qapplication_remove_library_path(self, path)
    }

    /// Set the `version` of this application.
    pub fn set_application_version(self: Pin<&mut Self>, version: &QString) {
        ffi::qapplication_set_application_version(self, version);
    }

    /// Changes the default application font to `font`.
    pub fn set_application_font(self: Pin<&mut Self>, font: &QFont) {
        ffi::qapplication_set_font(self, font);
    }

    /// Sets the list of directories to search when loading plugins with [QLibrary](https://doc.qt.io/qt/qlibrary.html) to `paths`.
    /// All existing paths will be deleted and the path list will consist of the paths given in `paths` and the path to the application.
    pub fn set_library_paths(self: Pin<&mut Self>, paths: &QStringList) {
        ffi::qapplication_set_library_paths(self, paths);
    }

    /// Sets the Internet `domain` of the organization that wrote this application.
    pub fn set_organization_domain(self: Pin<&mut Self>, domain: &QString) {
        ffi::qapplication_set_organization_domain(self, domain);
    }

    /// Sets the `name` of the organization that wrote this application.
    pub fn set_organization_name(self: Pin<&mut Self>, name: &QString) {
        ffi::qapplication_set_organization_name(self, name);
    }
}
