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
pub type QUrl = ffi::QUrl;

impl QUrl {
    /// Constrct a default null QUrl
    pub fn null() -> cxx::UniquePtr<Self> {
        ffi::qurl_init()
    }

    /// Construct a Rust QUrl from an existing QUrlCpp, this is a copy operation.
    pub fn from_ref(qurl: &QUrl) -> cxx::UniquePtr<Self> {
        ffi::qurl_init_from_qurl(qurl)
    }

    pub fn from_str(str: &str) -> cxx::UniquePtr<Self> {
        ffi::qurl_init_from_string(str)
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
        ffi::qurl_to_rust_string(self)
    }
}

impl From<&QUrl> for cxx::UniquePtr<QUrl> {
    fn from(value: &QUrl) -> cxx::UniquePtr<QUrl> {
        QUrl::from_ref(value)
    }
}
