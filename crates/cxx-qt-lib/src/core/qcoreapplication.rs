// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{QByteArray, QString, QStringList, QVector};

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

        include!("cxx-qt-lib/qcoreapplication.h");
        type QCoreApplication;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_new"]
        fn qcoreapplicationNew(args: &QVector_QByteArray) -> UniquePtr<QCoreApplication>;
    }

    // These are all static, so we need to create bindings until CXX supports statics
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_add_library_path"]
        fn qcoreapplicationAddLibraryPath(path: &QString);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_application_name"]
        fn qcoreapplicationApplicationName() -> QString;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_application_version"]
        fn qcoreapplicationApplicationVersion() -> QString;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_exec"]
        fn qcoreapplicationExec() -> i32;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_library_paths"]
        fn qcoreapplicationLibraryPaths() -> QStringList;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_organization_domain"]
        fn qcoreapplicationOrganizationDomain() -> QString;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_organization_name"]
        fn qcoreapplicationOrganizationName() -> QString;
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_remove_library_path"]
        fn qcoreapplicationRemoveLibraryPath(path: &QString);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_set_application_name"]
        fn qcoreapplicationSetApplicationName(name: &QString);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_set_application_version"]
        fn qcoreapplicationSetApplicationVersion(version: &QString);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_set_library_paths"]
        fn qcoreapplicationSetLibraryPaths(paths: &QStringList);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_set_organization_domain"]
        fn qcoreapplicationSetOrganizationDomain(domain: &QString);
        #[doc(hidden)]
        #[rust_name = "qcoreapplication_set_organization_name"]
        fn qcoreapplicationSetOrganizationName(name: &QString);
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
    pub fn add_library_path(path: &QString) {
        ffi::qcoreapplication_add_library_path(path);
    }

    /// The name of this application
    pub fn application_name() -> QString {
        ffi::qcoreapplication_application_name()
    }

    /// The version of this application
    pub fn application_version() -> QString {
        ffi::qcoreapplication_application_version()
    }

    /// Enters the main event loop and waits until exit() is called,
    /// and then returns the value that was set to exit() (which is 0 if exit() is called via quit()).
    pub fn exec(&self) -> i32 {
        ffi::qcoreapplication_exec()
    }

    /// Returns a list of paths that the application will search when dynamically loading libraries.
    pub fn library_paths() -> QStringList {
        ffi::qcoreapplication_library_paths()
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
    pub fn organization_domain() -> QString {
        ffi::qcoreapplication_organization_domain()
    }

    /// The name of the organization that wrote this application
    pub fn organization_name() -> QString {
        ffi::qcoreapplication_organization_name()
    }

    /// Removes path from the library path list. If path is empty or not in the path list, the list is not changed.
    pub fn remove_library_path(path: &QString) {
        ffi::qcoreapplication_remove_library_path(path)
    }

    /// Set the name of this application
    pub fn set_application_name(name: &QString) {
        ffi::qcoreapplication_set_application_name(name);
    }

    /// Set the version of this application
    pub fn set_application_version(version: &QString) {
        ffi::qcoreapplication_set_application_version(version);
    }

    /// Sets the list of directories to search when loading plugins with QLibrary to paths.
    /// All existing paths will be deleted and the path list will consist of the paths given in paths and the path to the application.
    pub fn set_library_paths(paths: &QStringList) {
        ffi::qcoreapplication_set_library_paths(paths);
    }

    /// Sets the Internet domain of the organization that wrote this application
    pub fn set_organization_domain(domain: &QString) {
        ffi::qcoreapplication_set_organization_domain(domain);
    }

    /// Sets the name of the organization that wrote this application
    pub fn set_organization_name(name: &QString) {
        ffi::qcoreapplication_set_organization_name(name);
    }
}
