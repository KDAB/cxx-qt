// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{QList, QString};
use core::mem::MaybeUninit;
use cxx::{type_id, ExternType};
use cxx_qt::casting::Upcast;
use std::fmt;
use std::ops::{Deref, DerefMut};

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type CaseSensitivity = crate::CaseSensitivity;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qlist_QString.h");
        type QList_QString = crate::QList<QString>;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = super::QStringList;

        /// Returns true if the list contains the string str; otherwise returns false.
        fn contains(self: &QStringList, str: &QString, cs: CaseSensitivity) -> bool;

        /// Returns a list of all the strings containing the substring str.
        fn filter(self: &QStringList, str: &QString, cs: CaseSensitivity) -> QStringList;

        /// Joins all the string list's strings into a single string with each element
        /// separated by the given separator (which can be an empty string).
        fn join(self: &QStringList, separator: &QString) -> QString;

        /// Sorts the list of strings in ascending order.
        fn sort(self: &mut QStringList, cs: CaseSensitivity);

        /// Returns a string list where every string has had the before text replaced with the after text wherever the before text is found.
        /// The before text is matched case-sensitively or not depending on the cs flag.
        #[rust_name = "replace_in_strings"]
        fn replaceInStrings(
            self: &mut QStringList,
            before: &QString,
            after: &QString,
            cs: CaseSensitivity,
        ) -> &mut QStringList;
    }

    #[namespace = "rust::cxxqt1"]
    unsafe extern "C++" {
        include!("cxx-qt/casting.h");

        #[doc(hidden)]
        #[rust_name = "upcast_qstringlist"]
        unsafe fn upcastPtr(thiz: *const QStringList) -> *const QList_QString;

        #[doc(hidden)]
        #[rust_name = "downcast_qlist_qstring"]
        unsafe fn downcastPtrStatic(base: *const QList_QString) -> *const QStringList;
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
        #[rust_name = "qstringlist_eq"]
        fn operatorEq(a: &QStringList, b: &QStringList) -> bool;

        #[doc(hidden)]
        #[rust_name = "qstringlist_to_debug_qstring"]
        fn toDebugQString(value: &QStringList) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qstringlist_from_qlist_qstring"]
        fn qstringlistFromQListQString(list: &QList_QString) -> QStringList;
        #[doc(hidden)]
        #[rust_name = "qstringlist_as_qlist_qstring"]
        fn qstringlistAsQListQString(list: &QStringList) -> QList_QString;
        #[doc(hidden)]
        #[rust_name = "qstringlist_remove_duplicates"]
        fn qstringlistRemoveDuplicates(list: &mut QStringList) -> isize;
    }
}

/// The QStringList class provides a list of strings.
#[repr(C)]
pub struct QStringList {
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QStringList has one pointer as a member
    /// Qt6 QStringList has one member, which contains two pointers and a size_t
    #[cfg(cxxqt_qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(cxxqt_qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
}

impl QStringList {
    /// This function removes duplicate entries from a list.
    /// The entries do not have to be sorted. They will retain their original order.
    pub fn remove_duplicates(&mut self) -> isize {
        ffi::qstringlist_remove_duplicates(self)
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

impl std::cmp::Eq for QStringList {}

impl fmt::Display for QStringList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qstringlist_to_debug_qstring(self))
    }
}

impl fmt::Debug for QStringList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", **self)
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
    /// Converts a `QList<QString>` into QStringList.
    fn from(list: &QList<QString>) -> Self {
        ffi::qstringlist_from_qlist_qstring(list)
    }
}

impl From<&QStringList> for QList<QString> {
    /// Converts a QStringList into a `QList<QString>`
    fn from(list: &QStringList) -> Self {
        ffi::qstringlist_as_qlist_qstring(list)
    }
}

impl Deref for QStringList {
    type Target = QList<QString>;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl DerefMut for QStringList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.upcast_mut()
    }
}

unsafe impl Upcast<QList<QString>> for QStringList {
    unsafe fn upcast_ptr(this: *const Self) -> *const QList<QString> {
        ffi::upcast_qstringlist(this)
    }

    unsafe fn from_base_ptr(base: *const QList<QString>) -> *const Self {
        ffi::downcast_qlist_qstring(base)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QStringList {
    type Id = type_id!("QStringList");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deref() {
        let mut list = QStringList::default();
        list.append(QString::from("element"));
        assert_eq!(
            list.get(0).map(|s| s.to_string()),
            Some("element".to_owned())
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn qstringlist_serde() {
        let mut qstringlist = QStringList::default();
        qstringlist.append(QString::from("element 1"));
        qstringlist.append(QString::from("element 2"));
        assert_eq!(crate::serde_impl::roundtrip(&qstringlist), qstringlist)
    }
}
