// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{QList, QString};
use core::mem::MaybeUninit;
use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qlist.h");
        type QList_QString = crate::QList<QString>;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = super::QStringList;

        /// Joins all the string list's strings into a single string with each element
        /// separated by the given separator (which can be an empty string).
        fn join(self: &QStringList, separator: &QString) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qstringlist_clone"]
        fn construct(list: &QStringList) -> QStringList;

        #[doc(hidden)]
        #[rust_name = "qstringlist_drop"]
        fn drop(url: &mut QStringList);

        #[doc(hidden)]
        #[rust_name = "qstringlist_default"]
        fn construct() -> QStringList;

        #[doc(hidden)]
        #[rust_name = "qstringlist_from_qstring"]
        fn construct(string: &QString) -> QStringList;

        #[doc(hidden)]
        #[rust_name = "qstringlist_from_qlist_qstring"]
        fn qstringlistFromQListQString(list: &QList_QString) -> QStringList;
        #[doc(hidden)]
        #[rust_name = "qstringlist_as_qlist_qstring"]
        fn qstringlistAsQListQString(list: &QStringList) -> QList_QString;

        #[doc(hidden)]
        #[rust_name = "qstringlist_contains"]
        fn qstringlistContains(list: &QStringList, string: &QString) -> bool;

        #[doc(hidden)]
        #[rust_name = "qstringlist_eq"]
        fn operatorEq(a: &QStringList, b: &QStringList) -> bool;

        #[doc(hidden)]
        #[rust_name = "qstringlist_to_qstring"]
        fn toQString(value: &QStringList) -> QString;
    }
}

/// The QStringList class provides a list of strings.
#[repr(C)]
pub struct QStringList {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QStringList has one pointer as a member
    /// Qt6 QStringList has one member, which contains two pointers and a size_t
    #[cfg(qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
}

impl QStringList {
    /// Returns true if the list contains the string str; otherwise returns false
    pub fn contains(&self, string: &QString) -> bool {
        ffi::qstringlist_contains(self, string)
    }
}

impl Clone for QStringList {
    /// Constructs a copy of other.
    fn clone(&self) -> Self {
        ffi::qstringlist_clone(self)
    }
}

impl Default for QStringList {
    /// Constructs an empty list.
    fn default() -> Self {
        ffi::qstringlist_default()
    }
}

impl std::cmp::PartialEq for QStringList {
    fn eq(&self, other: &Self) -> bool {
        ffi::qstringlist_eq(self, other)
    }
}

impl std::fmt::Display for QStringList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", ffi::qstringlist_to_qstring(self))
    }
}

impl std::fmt::Debug for QStringList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Drop for QStringList {
    /// Destroys the list.
    fn drop(&mut self) {
        ffi::qstringlist_drop(self);
    }
}

impl From<&QString> for QStringList {
    /// Constructs a string list that contains the given string
    fn from(string: &QString) -> Self {
        ffi::qstringlist_from_qstring(string)
    }
}

impl From<&QList<QString>> for QStringList {
    /// Converts a QList<QString> into QStringList.
    fn from(list: &QList<QString>) -> Self {
        ffi::qstringlist_from_qlist_qstring(list)
    }
}

impl From<&QStringList> for QList<QString> {
    /// Converts a QStringList into a QList<QString>
    fn from(list: &QStringList) -> Self {
        ffi::qstringlist_as_qlist_qstring(list)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QStringList {
    type Id = type_id!("QStringList");
    type Kind = cxx::kind::Trivial;
}
