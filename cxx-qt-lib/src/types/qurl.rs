// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");

        type QUrl;

        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qurl_to_rust_string"]
        fn qurlToRustString(url: &QUrl) -> String;

        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qurl_init_from_string"]
        fn qurlInitFromString(string: &str) -> UniquePtr<QUrl>;
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qurl_init_from_qurl"]
        fn qurlInitFromQUrl(url: &QUrl) -> UniquePtr<QUrl>;
    }

    impl UniquePtr<QUrl> {}
}

/// The QUrl class provides a convenient interface for working with URLs.
pub type QUrl = ffi::QUrl;

impl QUrl {
    /// Create a new Rust Url from this QUrl.
    /// This is a copy operation so any changes will not propagate to the original QUrl.
    pub fn to_rust(&self) -> Url {
        Url::from_qurl(self)
    }
}

/// The Rust representation of Qt's QUrl
///
/// Internally this holds a UniquePtr to a QUrl which has been constructed on the C++ side.
pub struct Url {
    // Note that once map_qt_value is removed later, this can become private again
    #[doc(hidden)]
    pub(crate) inner: cxx::UniquePtr<QUrl>,
}

impl Url {
    /// Construct a Rust Url from an existing UniquePtr<QUrl> this is a move operation
    ///
    /// This is used in QVariant::value so that we don't need to perform another copy
    pub(crate) fn from_unique_ptr(ptr: cxx::UniquePtr<QUrl>) -> Self {
        Self { inner: ptr }
    }

    /// Construct a Rust Url from an existing QUrl, this is a copy operation.
    pub fn from_qurl(qurl: &QUrl) -> Self {
        Self {
            inner: ffi::qurl_init_from_qurl(qurl),
        }
    }

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

    /// Returns a string representation of the URL.
    pub fn string(&self) -> String {
        if let Some(inner) = self.inner.as_ref() {
            ffi::qurl_to_rust_string(inner)
        } else {
            "".to_owned()
        }
    }
}

impl std::str::FromStr for Url {
    type Err = std::convert::Infallible;

    /// Constructs a URL by parsing the given string.
    fn from_str(string: &str) -> Result<Self, std::convert::Infallible> {
        Ok(Self {
            inner: ffi::qurl_init_from_string(string),
        })
    }
}

impl crate::ToUniquePtr for Url {
    type CppType = QUrl;

    /// Retrieve the UniquePtr to the Qt QUrl of this Rust Url
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QUrl> {
        self.inner
    }
}

impl From<&QUrl> for Url {
    fn from(qurl: &QUrl) -> Self {
        qurl.to_rust()
    }
}
