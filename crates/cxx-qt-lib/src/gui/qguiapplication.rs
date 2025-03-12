// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{KeyboardModifiers, MouseButtons, QByteArray, QFont, QString, QStringList, QVector};
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
        include!("cxx-qt-lib/qfont.h");
        type QFont = crate::QFont;

        include!("cxx-qt-lib/qcoreapplication.h");
        type QCoreApplication = crate::QCoreApplication;
    }

    #[namespace = "Qt"]
    unsafe extern "C++" {
        type KeyboardModifiers = crate::KeyboardModifiers;
        type MouseButtons = crate::MouseButtons;
    }

    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/qguiapplication.h");
        #[qobject]
        #[base = QCoreApplication]
        type QGuiApplication;
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
        fn qapplicationAddLibraryPath(app: Pin<&mut QGuiApplication>, path: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_application_name"]
        fn qapplicationApplicationName(app: &QGuiApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_remove_library_path"]
        fn qapplicationRemoveLibraryPath(app: &QGuiApplication, path: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_application_version"]
        fn qapplicationApplicationVersion(app: &QGuiApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_exec"]
        fn qapplicationExec(app: Pin<&mut QGuiApplication>) -> i32;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_library_paths"]
        fn qapplicationLibraryPaths(app: &QGuiApplication) -> QStringList;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_organization_domain"]
        fn qapplicationOrganizationDomain(app: &QGuiApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_organization_name"]
        fn qapplicationOrganizationName(app: &QGuiApplication) -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_application_name"]
        fn qapplicationSetApplicationName(app: Pin<&mut QGuiApplication>, name: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_application_version"]
        fn qapplicationSetApplicationVersion(app: Pin<&mut QGuiApplication>, version: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_font"]
        fn qguiapplicationSetFont(app: Pin<&mut QGuiApplication>, font: &QFont);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_font"]
        fn qguiapplicationFont(app: &QGuiApplication) -> QFont;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_library_paths"]
        fn qapplicationSetLibraryPaths(app: Pin<&mut QGuiApplication>, paths: &QStringList);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_organization_domain"]
        fn qapplicationSetOrganizationDomain(app: Pin<&mut QGuiApplication>, domain: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_organization_name"]
        fn qapplicationSetOrganizationName(app: Pin<&mut QGuiApplication>, name: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_set_desktop_file_name"]
        fn qguiapplicationSetDesktopFileName(name: &QString);
        #[doc(hidden)]
        #[rust_name = "qguiapplication_desktop_file_name"]
        fn qguiapplicationDesktopFileName() -> QString;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_keyboard_modifiers"]
        fn qguiapplicationKeyboardModifiers() -> KeyboardModifiers;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_mouse_buttons"]
        fn qguiapplicationMouseButtons() -> MouseButtons;
        #[doc(hidden)]
        #[rust_name = "qguiapplication_query_keyboard_modifiers"]
        fn qguiapplicationQueryKeyboardModifiers() -> KeyboardModifiers;
    }

    // QGuiApplication is not a trivial to CXX and is not relocatable in Qt
    // as the following fails in C++. So we cannot mark it as a trivial type
    // and need to use references or pointers.
    // static_assert(QTypeInfo<QGuiApplication>::isRelocatable);
    impl UniquePtr<QGuiApplication> {}
}

pub use ffi::QGuiApplication;

impl QGuiApplication {
    /// Prepends path to the beginning of the library path list,
    /// ensuring that it is searched for libraries first.
    /// If path is empty or already in the path list, the path list is not changed.
    pub fn add_library_path(self: Pin<&mut Self>, path: &QString) {
        ffi::qguiapplication_add_library_path(self, path);
    }

    /// The name of this application
    pub fn application_name(&self) -> QString {
        ffi::qguiapplication_application_name(self)
    }

    /// The version of this application
    pub fn application_version(&self) -> QString {
        ffi::qguiapplication_application_version(self)
    }

    /// Enters the main event loop and waits until exit() is called,
    /// and then returns the value that was set to exit() (which is 0 if exit() is called via quit()).
    pub fn exec(self: Pin<&mut Self>) -> i32 {
        ffi::qguiapplication_exec(self)
    }

    /// Returns the default application font.
    pub fn font(&self) -> QFont {
        ffi::qguiapplication_font(self)
    }

    /// Returns a list of paths that the application will search when dynamically loading libraries.
    pub fn library_paths(&self) -> QStringList {
        ffi::qguiapplication_library_paths(self)
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
    pub fn organization_domain(&self) -> QString {
        ffi::qguiapplication_organization_domain(self)
    }

    /// The name of the organization that wrote this application
    pub fn organization_name(&self) -> QString {
        ffi::qguiapplication_organization_name(self)
    }

    /// Set the name of this application
    pub fn set_application_name(self: Pin<&mut Self>, name: &QString) {
        ffi::qguiapplication_set_application_name(self, name);
    }

    /// Removes path from the library path list. If path is empty or not in the path list, the list is not changed.
    pub fn remove_library_path(&self, path: &QString) {
        ffi::qguiapplication_remove_library_path(self, path)
    }

    /// Set the version of this application
    pub fn set_application_version(self: Pin<&mut Self>, version: &QString) {
        ffi::qguiapplication_set_application_version(self, version);
    }

    /// Changes the default application font to font.
    pub fn set_application_font(self: Pin<&mut Self>, font: &QFont) {
        ffi::qguiapplication_set_font(self, font);
    }

    /// Sets the list of directories to search when loading plugins with QLibrary to paths.
    /// All existing paths will be deleted and the path list will consist of the paths given in paths and the path to the application.
    pub fn set_library_paths(self: Pin<&mut Self>, paths: &QStringList) {
        ffi::qguiapplication_set_library_paths(self, paths);
    }

    /// Sets the Internet domain of the organization that wrote this application
    pub fn set_organization_domain(self: Pin<&mut Self>, domain: &QString) {
        ffi::qguiapplication_set_organization_domain(self, domain);
    }

    /// Sets the name of the organization that wrote this application
    pub fn set_organization_name(self: Pin<&mut Self>, name: &QString) {
        ffi::qguiapplication_set_organization_name(self, name);
    }

    /// Changes the desktop file name to name.
    pub fn set_desktop_file_name(name: &QString) {
        ffi::qguiapplication_set_desktop_file_name(name);
    }

    /// Returns the application desktop file name.
    pub fn desktop_file_name() -> QString {
        ffi::qguiapplication_desktop_file_name()
    }

    /// Returns the current state of the modifier keys on the keyboard. The current state is updated
    /// synchronously as the event queue is emptied of events that will spontaneously change the
    /// keyboard state (QEvent::KeyPress and QEvent::KeyRelease events).
    ///
    /// It should be noted this may not reflect the actual keys held on the input device at the time
    /// of calling but rather the modifiers as last reported in an event.
    /// If no keys are being held Qt::NoModifier is returned.
    pub fn keyboard_modifiers(&self) -> KeyboardModifiers {
        ffi::qguiapplication_keyboard_modifiers()
    }

    /// Returns the current state of the buttons on the mouse. The current state is updated
    /// synchronously as the event queue is emptied of events that will spontaneously change the
    /// mouse state (QEvent::MouseButtonPress and QEvent::MouseButtonRelease events).
    ///
    /// It should be noted this may not reflect the actual buttons held on the input device at the
    /// time of calling but rather the mouse buttons as last reported in one of the above events.
    /// If no mouse buttons are being held Qt::NoButton is returned.
    pub fn mouse_buttons(&self) -> MouseButtons {
        ffi::qguiapplication_mouse_buttons()
    }

    /// Queries and returns the state of the modifier keys on the keyboard. Unlike
    /// keyboardModifiers, this method returns the actual keys held on the input device at the time
    /// of calling the method.
    ///
    /// It does not rely on the keypress events having been received by this process, which makes it
    /// possible to check the modifiers while moving a window, for instance. Note that in most
    /// cases, you should use keyboardModifiers(), which is faster and more accurate since it
    /// contains the state of the modifiers as they were when the currently processed event was
    /// received.
    pub fn query_keyboard_modifiers(&self) -> KeyboardModifiers {
        ffi::qguiapplication_query_keyboard_modifiers()
    }
}
