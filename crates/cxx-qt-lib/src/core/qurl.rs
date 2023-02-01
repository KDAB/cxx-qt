// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;

use crate::QString;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        include!("cxx-qt-lib/qstring.h");

        type QString = crate::QString;
        type QUrl = super::QUrl;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qurl_drop"]
        fn drop(url: &mut QUrl);

        #[doc(hidden)]
        #[rust_name = "qurl_init_default"]
        fn construct() -> QUrl;
        #[doc(hidden)]
        #[rust_name = "qurl_init_from_qstring"]
        fn construct(string: &QString) -> QUrl;
        #[doc(hidden)]
        #[rust_name = "qurl_init_from_string"]
        fn qurlInitFromString(string: &str) -> QUrl;
        #[doc(hidden)]
        #[rust_name = "qurl_init_from_qurl"]
        fn construct(url: &QUrl) -> QUrl;

        #[doc(hidden)]
        #[rust_name = "qurl_eq"]
        fn operatorEq(a: &QUrl, b: &QUrl) -> bool;

        #[doc(hidden)]
        #[rust_name = "qurl_to_rust_string"]
        fn qurlToRustString(url: &QUrl) -> String;
        #[doc(hidden)]
        #[rust_name = "qurl_to_qstring"]
        fn qurlToQString(url: &QUrl) -> QString;

        #[doc(hidden)]
        #[rust_name = "qurl_debug"]
        fn toQString(url: &QUrl) -> QString;
    }
}

/// The QUrl class provides a convenient interface for working with URLs.
#[repr(C)]
pub struct QUrl {
    _space: MaybeUninit<usize>,
}

impl QUrl {
    // TODO: other QUrl methods
    //
    // fragment: Option<String>,
    // host: Option<String>,
    // password: Option<String>,
    // path: Option<String>,
    // port: Option<u16>,
    // query: Option<String>,
    // scheme: Option<String>,
    // userName: Option<String>,

    /// Returns a QString representation of the URL.
    pub fn to_qstring(&self) -> QString {
        ffi::qurl_to_qstring(self)
    }
}

impl Clone for QUrl {
    /// Constructs a copy of other.
    fn clone(&self) -> Self {
        ffi::qurl_init_from_qurl(self)
    }
}

impl Default for QUrl {
    /// Constructs an empty QUrl object.
    fn default() -> Self {
        ffi::qurl_init_default()
    }
}

impl std::cmp::PartialEq for QUrl {
    fn eq(&self, other: &Self) -> bool {
        ffi::qurl_eq(self, other)
    }
}

impl std::cmp::Eq for QUrl {}

impl fmt::Display for QUrl {
    /// Convert the QUrl to a Rust string
    ///
    /// Note that this converts from UTF-16 to UTF-8
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", ffi::qurl_to_rust_string(self))
    }
}

impl fmt::Debug for QUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qurl_debug(self))
    }
}

impl Drop for QUrl {
    /// Destructor; called immediately before the object is deleted.
    fn drop(&mut self) {
        ffi::qurl_drop(self)
    }
}

impl From<&QString> for QUrl {
    /// Constructs a QUrl from a QString
    fn from(str: &QString) -> Self {
        ffi::qurl_init_from_qstring(str)
    }
}

impl From<&str> for QUrl {
    /// Constructs a QUrl from a Rust string
    ///
    /// Note that this converts from UTF-8 to UTF-16
    fn from(str: &str) -> Self {
        ffi::qurl_init_from_string(str)
    }
}

impl From<&String> for QUrl {
    /// Constructs a QUrl from a Rust string
    ///
    /// Note that this converts from UTF-8 to UTF-16
    fn from(str: &String) -> Self {
        ffi::qurl_init_from_string(str)
    }
}

#[cfg(feature = "http")]
impl From<&http::Uri> for QUrl {
    fn from(value: &http::Uri) -> Self {
        QUrl::from(&value.to_string())
    }
}

#[cfg(feature = "http")]
impl TryFrom<&QUrl> for http::Uri {
    type Error = http::uri::InvalidUri;

    fn try_from(value: &QUrl) -> Result<Self, Self::Error> {
        value.to_string().parse::<http::Uri>()
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QUrl {
    type Id = type_id!("QUrl");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "http")]
    use super::*;

    #[cfg(feature = "http")]
    #[test]
    fn test_http() {
        let uri = "https://github.com/kdab/cxx-qt"
            .parse::<http::Uri>()
            .unwrap();
        let qurl = QUrl::from(&uri);
        assert_eq!(uri.to_string(), qurl.to_string());

        let http_uri = http::Uri::try_from(&qurl).unwrap();
        assert_eq!(http_uri, uri);
    }
}
