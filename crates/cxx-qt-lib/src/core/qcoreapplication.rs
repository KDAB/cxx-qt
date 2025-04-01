// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{QByteArray, QString, QStringList, QVector};
use core::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;
        include!("cxx-qt-lib/qvector_QByteArray.h");
        type QVector_QByteArray = crate::QVector<QByteArray>;

        include!("cxx-qt-lib/qcoreapplication.h");
        type QCoreApplication;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_new"]
        fn qcoreapplicationNew(args: &QVector_QByteArray) -> UniquePtr<QCoreApplication>;
    }

    unsafe extern "C++Qt" {
        #[doc(hidden)]
        #[rust_name = "about_to_quit"]
        #[qsignal]
        pub(self) fn aboutToQuit(self: Pin<&mut QCoreApplication>);
    }

    // These are all static, so we need to create bindings until CXX supports statics
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_add_library_path"]
        fn qapplicationAddLibraryPath(app: Pin<&mut QCoreApplication>, path: &QString);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_application_name"]
        fn qapplicationApplicationName(app: &QCoreApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_application_version"]
        fn qapplicationApplicationVersion(app: &QCoreApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_exec"]
        fn qapplicationExec(app: Pin<&mut QCoreApplication>) -> i32;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_library_paths"]
        fn qapplicationLibraryPaths(app: &QCoreApplication) -> QStringList;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_organization_domain"]
        fn qapplicationOrganizationDomain(app: &QCoreApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_organization_name"]
        fn qapplicationOrganizationName(app: &QCoreApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_remove_library_path"]
        fn qapplicationRemoveLibraryPath(app: &QCoreApplication, path: &QString);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_set_application_name"]
        fn qapplicationSetApplicationName(app: Pin<&mut QCoreApplication>, name: &QString);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_set_application_version"]
        fn qapplicationSetApplicationVersion(app: Pin<&mut QCoreApplication>, version: &QString);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_set_library_paths"]
        fn qapplicationSetLibraryPaths(app: Pin<&mut QCoreApplication>, paths: &QStringList);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_set_organization_domain"]
        fn qapplicationSetOrganizationDomain(app: Pin<&mut QCoreApplication>, domain: &QString);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_set_organization_name"]
        fn qapplicationSetOrganizationName(app: Pin<&mut QCoreApplication>, name: &QString);
    }

    // QCoreApplication is not a trivial to CXX and is not relocatable in Qt
    // as the following fails in C++. So we cannot mark it as a trivial type
    // and need to use references or pointers.
    // static_assert(QTypeInfo<QCoreApplication>::isRelocatable);
    impl UniquePtr<QCoreApplication> {}
}

pub use ffi::QCoreApplication;

impl QCoreApplication {
    /// Prepends path to the beginning of the library path list,
    /// ensuring that it is searched for libraries first.
    /// If path is empty or already in the path list, the path list is not changed.
    pub fn add_library_path(self: Pin<&mut Self>, path: &QString) {
        ffi::qcoreapplication_add_library_path(self, path);
    }

    /// The name of this application
    pub fn application_name(&self) -> QString {
        ffi::qcoreapplication_application_name(self)
    }

    /// The version of this application
    pub fn application_version(&self) -> QString {
        ffi::qcoreapplication_application_version(self)
    }

    /// Enters the main event loop and waits until exit() is called,
    /// and then returns the value that was set to exit() (which is 0 if exit() is called via quit()).
    pub fn exec(self: Pin<&mut Self>) -> i32 {
        ffi::qcoreapplication_exec(self)
    }

    /// Returns a list of paths that the application will search when dynamically loading libraries.
    pub fn library_paths(&self) -> QStringList {
        ffi::qcoreapplication_library_paths(self)
    }

    /// Initializes the window system and constructs an application object with command line arguments in args.
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

        ffi::qcoreapplication_new(&vector)
    }

    /// The Internet domain of the organization that wrote this application
    pub fn organization_domain(&self) -> QString {
        ffi::qcoreapplication_organization_domain(self)
    }

    /// The name of the organization that wrote this application
    pub fn organization_name(&self) -> QString {
        ffi::qcoreapplication_organization_name(self)
    }

    /// Removes path from the library path list. If path is empty or not in the path list, the list is not changed.
    pub fn remove_library_path(&self, path: &QString) {
        ffi::qcoreapplication_remove_library_path(self, path)
    }

    /// Set the name of this application
    pub fn set_application_name(self: Pin<&mut Self>, name: &QString) {
        ffi::qcoreapplication_set_application_name(self, name);
    }

    /// Set the version of this application
    pub fn set_application_version(self: Pin<&mut Self>, version: &QString) {
        ffi::qcoreapplication_set_application_version(self, version);
    }

    /// Sets the list of directories to search when loading plugins with QLibrary to paths.
    /// All existing paths will be deleted and the path list will consist of the paths given in paths and the path to the application.
    pub fn set_library_paths(self: Pin<&mut Self>, paths: &QStringList) {
        ffi::qcoreapplication_set_library_paths(self, paths);
    }

    /// Sets the Internet domain of the organization that wrote this application
    pub fn set_organization_domain(self: Pin<&mut Self>, domain: &QString) {
        ffi::qcoreapplication_set_organization_domain(self, domain);
    }

    /// Sets the name of the organization that wrote this application
    pub fn set_organization_name(self: Pin<&mut Self>, name: &QString) {
        ffi::qcoreapplication_set_organization_name(self, name);
    }
}
