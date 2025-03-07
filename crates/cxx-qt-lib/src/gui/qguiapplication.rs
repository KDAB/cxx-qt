// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{QByteArray, QFont, QString, QStringList, QVector};
use cxx_qt::Upcast;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;
        include!("cxx-qt-lib/qvector.h");
        type QVector_QByteArray = crate::QVector<QByteArray>;
        include!("cxx-qt-lib/qfont.h");
        type QFont = crate::QFont;

        include!("cxx-qt-lib/qguiapplication.h");
        type QGuiApplication;

        include!("cxx-qt-lib/qcoreapplication.h");
        type QCoreApplication;

        include!("cxx-qt/casting.h");

        #[doc(hidden)]
        #[rust_name = "upcast_qguiapplication"]
        #[cxx_name = "upcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn upcast(thiz: *const QGuiApplication) -> *const QCoreApplication;

        #[doc(hidden)]
        #[rust_name = "downcast_qcoreapplication"]
        #[cxx_name = "downcastPtr"]
        #[namespace = "rust::cxxqt1"]
        unsafe fn downcast(base: *const QCoreApplication) -> *const QGuiApplication;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qguiapplication_new"]
        fn qguiapplicationNew(args: &QVector_QByteArray) -> UniquePtr<QGuiApplication>;
    }

    // These are all static, so we need to create bindings until CXX supports statics
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        // Reuse the templated versions from QCoreApplication
        include!("cxx-qt-lib/qcoreapplication.h");

        #[doc(hidden)]
        #[rust_name = "qguiapplication_add_library_path"]
        fn qguiapplicationAddLibraryPath(path: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_application_name"]
        fn qguiapplicationApplicationName() -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_remove_library_path"]
        fn qguiapplicationRemoveLibraryPath(path: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_application_version"]
        fn qguiapplicationApplicationVersion() -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_exec"]
        fn qguiapplicationExec() -> i32;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_library_paths"]
        fn qguiapplicationLibraryPaths() -> QStringList;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_organization_domain"]
        fn qguiapplicationOrganizationDomain() -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_organization_name"]
        fn qguiapplicationOrganizationName() -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_application_name"]
        fn qguiapplicationSetApplicationName(name: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_application_version"]
        fn qguiapplicationSetApplicationVersion(version: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_font"]
        fn qguiapplicationSetFont(font: &QFont);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_font"]
        fn qguiapplicationFont() -> QFont;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_library_paths"]
        fn qguiapplicationSetLibraryPaths(paths: &QStringList);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_organization_domain"]
        fn qguiapplicationSetOrganizationDomain(domain: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_organization_name"]
        fn qguiapplicationSetOrganizationName(name: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_desktop_file_name"]
        fn qguiapplicationSetDesktopFileName(name: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_desktop_file_name"]
        fn qguiapplicationDesktopFileName() -> QString;
    }

    // QGuiApplication is not a trivial to CXX and is not relocatable in Qt
    // as the following fails in C++. So we cannot mark it as a trivial type
    // and need to use references or pointers.
    // static_assert(QTypeInfo<QGuiApplication>::isRelocatable);
    impl UniquePtr<QGuiApplication> {}
}

pub use ffi::{
    downcast_qcoreapplication, upcast_qguiapplication, QCoreApplication, QGuiApplication,
};

impl Upcast<QCoreApplication> for QGuiApplication {
    unsafe fn upcast_ptr(this: *const Self) -> *const QCoreApplication {
        upcast_qguiapplication(this)
    }

    unsafe fn from_base_ptr(base: *const QCoreApplication) -> *const Self {
        downcast_qcoreapplication(base)
    }
}

impl QGuiApplication {
    /// Prepends path to the beginning of the library path list,
    /// ensuring that it is searched for libraries first.
    /// If path is empty or already in the path list, the path list is not changed.
    pub fn add_library_path(path: &QString) {
        ffi::qguiapplication_add_library_path(path);
    }

    /// The name of this application
    pub fn application_name() -> QString {
        ffi::qguiapplication_application_name()
    }

    /// The version of this application
    pub fn application_version() -> QString {
        ffi::qguiapplication_application_version()
    }

    /// Enters the main event loop and waits until exit() is called,
    /// and then returns the value that was set to exit() (which is 0 if exit() is called via quit()).
    pub fn exec(&self) -> i32 {
        ffi::qguiapplication_exec()
    }

    /// Returns the default application font.
    pub fn font() -> QFont {
        ffi::qguiapplication_font()
    }

    /// Returns a list of paths that the application will search when dynamically loading libraries.
    pub fn library_paths() -> QStringList {
        ffi::qguiapplication_library_paths()
    }

    /// Initializes the window system and constructs an application object.
    /// Standard [Qt command line arguments](https://doc.qt.io/qt-6/qguiapplication.html#supported-command-line-options) are handled automatically.
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

        ffi::qguiapplication_new(&vector)
    }

    /// The Internet domain of the organization that wrote this application
    pub fn organization_domain() -> QString {
        ffi::qguiapplication_organization_domain()
    }

    /// The name of the organization that wrote this application
    pub fn organization_name() -> QString {
        ffi::qguiapplication_organization_name()
    }

    /// Set the name of this application
    pub fn set_application_name(name: &QString) {
        ffi::qguiapplication_set_application_name(name);
    }

    /// Removes path from the library path list. If path is empty or not in the path list, the list is not changed.
    pub fn remove_library_path(path: &QString) {
        ffi::qguiapplication_remove_library_path(path)
    }

    /// Set the version of this application
    pub fn set_application_version(version: &QString) {
        ffi::qguiapplication_set_application_version(version);
    }

    /// Changes the default application font to font.
    pub fn set_application_font(font: &QFont) {
        ffi::qguiapplication_set_font(font);
    }

    /// Sets the list of directories to search when loading plugins with QLibrary to paths.
    /// All existing paths will be deleted and the path list will consist of the paths given in paths and the path to the application.
    pub fn set_library_paths(paths: &QStringList) {
        ffi::qguiapplication_set_library_paths(paths);
    }

    /// Sets the Internet domain of the organization that wrote this application
    pub fn set_organization_domain(domain: &QString) {
        ffi::qguiapplication_set_organization_domain(domain);
    }

    /// Sets the name of the organization that wrote this application
    pub fn set_organization_name(name: &QString) {
        ffi::qguiapplication_set_organization_name(name);
    }

    /// Changes the desktop file name to name.
    pub fn set_desktop_file_name(name: &QString) {
        ffi::qguiapplication_set_desktop_file_name(name);
    }

    /// Returns the application desktop file name.
    pub fn desktop_file_name() -> QString {
        ffi::qguiapplication_desktop_file_name()
    }
}
