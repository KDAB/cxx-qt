// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qt_types.h");

        type QUrl;

        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qurl_to_rust_string"]
        fn qurlToRustString(url: &QUrl) -> String;

        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qurl_init"]
        fn qurlInit() -> UniquePtr<QUrl>;
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qurl_init_from_string"]
        fn qurlInitFromString(string: &str) -> UniquePtr<QUrl>;
        #[namespace = "rust::cxxqtlib1"]
        #[rust_name = "qurl_init_from_qurl"]
        fn qurlInitFromQUrl(url: &QUrl) -> UniquePtr<QUrl>;
    }

    impl UniquePtr<QUrl> {}
}

/// The QUrlCpp class provides a convenient interface for working with URLs.
///
/// Note that this is the C++ representation and QUrl should be used in Rust.
pub type QUrlCpp = ffi::QUrl;

impl QUrlCpp {
    /// Create a new Rust QUrl from this QUrlCpp.
    /// This is a copy operation so any changes will not propagate to the original QUrlCpp.
    pub fn to_rust(&self) -> QUrl {
        QUrl::from_qurl(self)
    }
}

/// The Rust representation of Qt's QUrl
///
/// Internally this holds a UniquePtr to a QUrlCpp which has been constructed on the C++ side.
pub struct QUrl {
    inner: cxx::UniquePtr<QUrlCpp>,
}

impl Default for QUrl {
    fn default() -> Self {
        QUrl::from_unique_ptr(ffi::qurl_init())
    }
}

impl QUrl {
    /// Construct a Rust QUrl from an existing UniquePtr<QUrlCpp> this is a move operation
    ///
    /// This is used in QVariant::value so that we don't need to perform another copy
    pub(crate) fn from_unique_ptr(ptr: cxx::UniquePtr<QUrlCpp>) -> Self {
        Self { inner: ptr }
    }

    /// Construct a Rust QUrl from an existing QUrlCpp, this is a copy operation.
    pub fn from_qurl(qurl: &QUrlCpp) -> Self {
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

impl std::str::FromStr for QUrl {
    type Err = std::convert::Infallible;

    /// Constructs a URL by parsing the given string.
    fn from_str(string: &str) -> Result<Self, std::convert::Infallible> {
        Ok(Self {
            inner: ffi::qurl_init_from_string(string),
        })
    }
}

impl crate::ToUniquePtr for QUrl {
    type CppType = QUrlCpp;

    /// Retrieve the UniquePtr to the Qt QUrlCpp of this Rust QUrl
    /// so that this object can be passed back to C++.
    fn to_unique_ptr(self) -> cxx::UniquePtr<QUrlCpp> {
        self.inner
    }
}

impl From<&QUrlCpp> for QUrl {
    fn from(qurl: &QUrlCpp) -> Self {
        qurl.to_rust()
    }
}
