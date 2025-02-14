// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = super::QUrl;

        /// Resets the content of the QUrl. After calling this function,
        /// the QUrl is equal to one that has been constructed with the default empty constructor.
        fn clear(self: &mut QUrl);

        /// Returns an error message if the last operation that modified this QUrl object ran into a parsing error.
        /// If no error was detected, this function returns an empty string and isValid() returns true.
        #[rust_name = "error_string"]
        fn errorString(self: &QUrl) -> QString;

        /// Returns true if this URL contains a fragment (i.e., if # was seen on it).
        #[rust_name = "has_fragment"]
        fn hasFragment(self: &QUrl) -> bool;

        /// Returns true if this URL contains a Query (i.e., if ? was seen on it).
        #[rust_name = "has_query"]
        fn hasQuery(self: &QUrl) -> bool;

        /// Returns true if the URL has no data; otherwise returns false.
        #[rust_name = "is_empty"]
        fn isEmpty(self: &QUrl) -> bool;

        /// Returns true if this URL is pointing to a local file path. A URL is a local file path if the scheme is "file".
        #[rust_name = "is_local_file"]
        fn isLocalFile(self: &QUrl) -> bool;

        /// Returns true if this URL is a parent of child_url.
        /// childUrl is a child of this URL if the two URLs share the same scheme and authority,
        /// and this URL's path is a parent of the path of child_url.
        #[rust_name = "is_parent_of"]
        fn isParentOf(self: &QUrl, child_url: &QUrl) -> bool;

        /// Returns true if the URL is relative; otherwise returns false.
        /// A URL is relative reference if its scheme is undefined;
        /// this function is therefore equivalent to calling scheme().is_empty().
        #[rust_name = "is_relative"]
        fn isRelative(self: &QUrl) -> bool;

        /// Returns true if the URL is non-empty and valid; otherwise returns false.
        #[rust_name = "is_valid"]
        fn isValid(self: &QUrl) -> bool;

        /// Returns the port of the URL, or defaultPort if the port is unspecified.
        #[rust_name = "port_or"]
        fn port(self: &QUrl, port: i32) -> i32;

        /// Returns the result of the merge of this URL with relative. This URL is used as a base to convert relative to an absolute URL.
        fn resolved(self: &QUrl, relative: &QUrl) -> QUrl;

        /// Returns the scheme of the URL. If an empty string is returned,
        /// this means the scheme is undefined and the URL is then relative.
        ///
        /// The scheme can only contain US-ASCII letters or digits,
        /// which means it cannot contain any character that would otherwise require encoding
        /// Additionally, schemes are always returned in lowercase form.
        #[rust_name = "scheme_or_default"]
        fn scheme(self: &QUrl) -> QString;

        /// Sets the port of the URL to port.
        ///
        /// port must be between 0 and 65535 inclusive. Setting the port to -1 indicates that the port is unspecified.
        #[rust_name = "set_port"]
        fn setPort(self: &mut QUrl, port: i32);

        /// Returns the path of this URL formatted as a local file path.
        /// The path returned will use forward slashes, even if it was originally created from one with backslashes.
        #[rust_name = "to_local_file_or_default"]
        fn toLocalFile(self: &QUrl) -> QString;
    }

    // Bitwise enums don't work well with Rust and CXX, so lets just use the defaults for now
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[rust_name = "qurl_authority"]
        fn qurlAuthority(url: &QUrl) -> QString;
        #[rust_name = "qurl_file_name"]
        fn qurlFileName(url: &QUrl) -> QString;
        #[rust_name = "qurl_fragment"]
        fn qurlFragment(url: &QUrl) -> QString;
        #[rust_name = "qurl_from_encoded"]
        fn qurlFromEncoded(input: &QByteArray) -> QUrl;
        #[rust_name = "qurl_from_local_file"]
        fn qurlFromLocalFile(local_file: &QString) -> QUrl;
        #[rust_name = "qurl_from_percent_encoding"]
        fn qurlFromPercentEncoding(input: &QByteArray) -> QString;
        #[rust_name = "qurl_from_user_input"]
        fn qurlFromUserInput(user_input: &QString, working_directory: &QString) -> QUrl;
        #[rust_name = "qurl_host"]
        fn qurlHost(url: &QUrl) -> QString;
        #[rust_name = "qurl_idn_whitelist"]
        fn qurlIdnWhitelist() -> QStringList;
        #[rust_name = "qurl_path"]
        fn qurlPath(url: &QUrl) -> QString;
        #[rust_name = "qurl_password"]
        fn qurlPassword(url: &QUrl) -> QString;
        #[rust_name = "qurl_query"]
        fn qurlQuery(url: &QUrl) -> QString;
        #[rust_name = "qurl_set_authority"]
        fn qurlSetAuthority(url: &mut QUrl, authority: &QString);
        #[rust_name = "qurl_set_fragment"]
        fn qurlSetFragment(url: &mut QUrl, fragment: &QString);
        #[rust_name = "qurl_set_host"]
        fn qurlSetHost(url: &mut QUrl, host: &QString);
        #[rust_name = "qurl_set_idn_whitelist"]
        fn qurlSetIdnWhitelist(list: &QStringList);
        #[rust_name = "qurl_set_password"]
        fn qurlSetPassword(url: &mut QUrl, password: &QString);
        #[rust_name = "qurl_set_path"]
        fn qurlSetPath(url: &mut QUrl, path: &QString);
        #[rust_name = "qurl_set_query"]
        fn qurlSetQuery(url: &mut QUrl, query: &QString);
        #[rust_name = "qurl_set_scheme"]
        fn qurlSetScheme(url: &mut QUrl, scheme: &QString);
        #[rust_name = "qurl_set_url"]
        fn qurlSetUrl(url: &mut QUrl, new_url: &QString);
        #[rust_name = "qurl_set_user_info"]
        fn qurlSetUserInfo(url: &mut QUrl, user_info: &QString);
        #[rust_name = "qurl_set_user_name"]
        fn qurlSetUserName(url: &mut QUrl, user_name: &QString);
        #[rust_name = "qurl_to_display_string"]
        fn qurlToDisplayString(url: &QUrl) -> QString;
        #[rust_name = "qurl_to_encoded"]
        fn qurlToEncoded(url: &QUrl) -> QByteArray;
        #[rust_name = "qurl_to_qstring"]
        fn qurlToQString(url: &QUrl) -> QString;
        #[rust_name = "qurl_to_percent_encoding"]
        fn qurlToPercentEncoding(
            input: &QString,
            exclude: &QByteArray,
            include: &QByteArray,
        ) -> QByteArray;
        #[rust_name = "qurl_user_info"]
        fn qurlUserInfo(url: &QUrl) -> QString;
        #[rust_name = "qurl_user_name"]
        fn qurlUserName(url: &QUrl) -> QString;
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
        #[rust_name = "qurl_init_from_qurl"]
        fn construct(url: &QUrl) -> QUrl;

        #[doc(hidden)]
        #[rust_name = "qurl_eq"]
        fn operatorEq(a: &QUrl, b: &QUrl) -> bool;

        #[doc(hidden)]
        #[rust_name = "qurl_to_debug_qstring"]
        fn toDebugQString(url: &QUrl) -> QString;
    }
}

/// The QUrl class provides a convenient interface for working with URLs.
#[repr(C)]
pub struct QUrl {
    _space: MaybeUninit<usize>,
}

impl QUrl {
    /// Returns the authority of the URL if it is defined; otherwise an empty string is returned.
    pub fn authority_or_default(&self) -> ffi::QString {
        ffi::qurl_authority(self)
    }

    /// Returns the name of the file, excluding the directory path.
    ///
    /// Note that, if this QUrl object is given a path ending in a slash, the name of the file is considered empty.
    ///
    /// If the path doesn't contain any slash, it is fully returned as the fileName.
    pub fn file_name(&self) -> ffi::QString {
        ffi::qurl_file_name(self)
    }

    /// Returns the fragment of the URL.
    pub fn fragment(&self) -> Option<ffi::QString> {
        if self.has_fragment() {
            Some(self.fragment_or_default())
        } else {
            None
        }
    }

    /// Returns the fragment of the URL if it is defined; otherwise an empty string is returned.
    pub fn fragment_or_default(&self) -> ffi::QString {
        ffi::qurl_fragment(self)
    }

    /// Parses input and returns the corresponding QUrl. input is assumed to be in encoded form, containing only ASCII characters.
    pub fn from_encoded(input: &ffi::QByteArray) -> Self {
        ffi::qurl_from_encoded(input)
    }

    /// Returns a QUrl representation of localFile, interpreted as a local file.
    /// This function accepts paths separated by slashes as well as the native separator for this platform.
    pub fn from_local_file(local_file: &ffi::QString) -> Self {
        ffi::qurl_from_local_file(local_file)
    }

    /// Returns a decoded copy of input. input is first decoded from percent encoding,
    /// then converted from UTF-8 to unicode.
    pub fn from_percent_encoding(input: &ffi::QByteArray) -> ffi::QString {
        ffi::qurl_from_percent_encoding(input)
    }

    /// Returns a valid URL from a user supplied userInput string if one can be deduced.
    /// In the case that is not possible, an invalid QUrl() is returned.
    pub fn from_user_input(user_input: &ffi::QString, working_directory: &ffi::QString) -> Self {
        ffi::qurl_from_user_input(user_input, working_directory)
    }

    /// Returns the host of the URL if it is defined; otherwise an empty string is returned.
    pub fn host_or_default(&self) -> ffi::QString {
        ffi::qurl_host(self)
    }

    /// Returns the current whitelist of top-level domains that are allowed to have non-ASCII characters in their compositions.
    pub fn idn_whitelist() -> ffi::QStringList {
        ffi::qurl_idn_whitelist()
    }

    /// Returns the password of the URL if it is defined; otherwise an empty string is returned.
    pub fn password_or_default(&self) -> ffi::QString {
        ffi::qurl_password(self)
    }

    /// Returns the path of the URL.
    pub fn path(&self) -> ffi::QString {
        ffi::qurl_path(self)
    }

    /// Returns the query string of the URL if there's a query string
    pub fn query(&self) -> Option<ffi::QString> {
        if self.has_query() {
            Some(self.query_or_default())
        } else {
            None
        }
    }

    /// Returns the query string of the URL if it is defined; otherwise an empty string is returned.
    pub fn query_or_default(&self) -> ffi::QString {
        ffi::qurl_query(self)
    }

    /// Returns the scheme of the URL. If the Option is None,
    /// this means the scheme is undefined and the URL is then relative.
    ///
    /// The scheme can only contain US-ASCII letters or digits,
    /// which means it cannot contain any character that would otherwise require encoding
    /// Additionally, schemes are always returned in lowercase form.
    pub fn scheme(&self) -> Option<ffi::QString> {
        let scheme = self.scheme_or_default();
        if scheme.is_empty() {
            None
        } else {
            Some(scheme)
        }
    }

    /// Sets the authority of the URL to authority.
    pub fn set_authority(&mut self, authority: &ffi::QString) {
        ffi::qurl_set_authority(self, authority)
    }

    /// Sets the fragment of the URL to fragment.
    /// The fragment is the last part of the URL, represented by a '#' followed by a string of characters.
    pub fn set_fragment(&mut self, fragment: &ffi::QString) {
        ffi::qurl_set_fragment(self, fragment)
    }

    /// Sets the host of the URL to host. The host is part of the authority.
    pub fn set_host(&mut self, host: &ffi::QString) {
        ffi::qurl_set_host(self, host)
    }

    /// Sets the whitelist of Top-Level Domains (TLDs) that are allowed to have non-ASCII characters in domains to the value of list.
    pub fn set_idn_whitelist(list: &ffi::QStringList) {
        ffi::qurl_set_idn_whitelist(list)
    }

    /// Sets the URL's password to password.
    pub fn set_password(&mut self, password: &ffi::QString) {
        ffi::qurl_set_password(self, password)
    }

    /// Sets the path of the URL to path.
    /// The path is the part of the URL that comes after the authority but before the query string.
    pub fn set_path(&mut self, path: &ffi::QString) {
        ffi::qurl_set_path(self, path)
    }

    /// Sets the query string of the URL to query.
    pub fn set_query(&mut self, query: &ffi::QString) {
        ffi::qurl_set_query(self, query)
    }

    /// Sets the scheme of the URL to scheme. As a scheme can only contain ASCII characters,
    /// no conversion or decoding is done on the input. It must also start with an ASCII letter.
    pub fn set_scheme(&mut self, scheme: &ffi::QString) {
        ffi::qurl_set_scheme(self, scheme)
    }

    /// Parses url and sets this object to that value.
    /// QUrl will automatically percent encode all characters that are not allowed in a URL
    /// and decode the percent-encoded sequences that represent an unreserved character
    /// (letters, digits, hyphens, underscores, dots and tildes).
    /// All other characters are left in their original forms.
    pub fn set_url(&mut self, url: &ffi::QString) {
        ffi::qurl_set_url(self, url)
    }

    /// Sets the user info of the URL to userInfo.
    pub fn set_user_info(&mut self, user_info: &ffi::QString) {
        ffi::qurl_set_user_info(self, user_info)
    }

    /// Sets the URL's user name to userName.
    pub fn set_user_name(&mut self, user_name: &ffi::QString) {
        ffi::qurl_set_user_name(self, user_name)
    }

    /// Returns a human-displayable string representation of the URL.
    /// The option RemovePassword is always enabled, since passwords should never be shown back to users.
    pub fn to_display_string(&self) -> ffi::QString {
        ffi::qurl_to_display_string(self)
    }

    /// Returns the encoded representation of the URL if it's valid; otherwise an empty QByteArray is returned.
    pub fn to_encoded(&self) -> ffi::QByteArray {
        ffi::qurl_to_encoded(self)
    }

    /// Returns the path of this URL formatted as a local file path.
    /// The path returned will use forward slashes, even if it was originally created from one with backslashes.
    pub fn to_local_file(&self) -> Option<ffi::QString> {
        if self.is_local_file() {
            Some(self.to_local_file_or_default())
        } else {
            None
        }
    }

    /// Returns an encoded copy of input. input is first converted to UTF-8,
    /// and all ASCII-characters that are not in the unreserved group are percent encoded.
    /// To prevent characters from being percent encoded pass them to exclude.
    /// To force characters to be percent encoded pass them to include.
    pub fn to_percent_encoding(
        input: &ffi::QString,
        exclude: &ffi::QByteArray,
        include: &ffi::QByteArray,
    ) -> ffi::QByteArray {
        ffi::qurl_to_percent_encoding(input, exclude, include)
    }

    /// Returns a QString representation of the URL.
    pub fn to_qstring(&self) -> ffi::QString {
        ffi::qurl_to_qstring(self)
    }

    /// Returns the user info of the URL, or an empty string if the user info is undefined.
    pub fn user_info_or_default(&self) -> ffi::QString {
        ffi::qurl_user_info(self)
    }

    /// Returns the user name of the URL if it is defined; otherwise an empty string is returned.
    pub fn user_name_or_default(&self) -> ffi::QString {
        ffi::qurl_user_name(self)
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
        write!(f, "{}", ffi::qurl_to_display_string(self))
    }
}

impl fmt::Debug for QUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qurl_to_debug_qstring(self))
    }
}

impl Drop for QUrl {
    /// Destructor; called immediately before the object is deleted.
    fn drop(&mut self) {
        ffi::qurl_drop(self)
    }
}

impl From<&ffi::QString> for QUrl {
    /// Constructs a QUrl from a QString
    fn from(str: &ffi::QString) -> Self {
        ffi::qurl_init_from_qstring(str)
    }
}

impl From<&str> for QUrl {
    /// Constructs a QUrl from a Rust string
    ///
    /// Note that this converts from UTF-8 to UTF-16
    fn from(str: &str) -> Self {
        Self::from(&ffi::QString::from(str))
    }
}

impl From<&String> for QUrl {
    /// Constructs a QUrl from a Rust string
    ///
    /// Note that this converts from UTF-8 to UTF-16
    fn from(str: &String) -> Self {
        Self::from(str.as_str())
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

#[cfg(feature = "url")]
impl From<&url::Url> for QUrl {
    fn from(value: &url::Url) -> Self {
        QUrl::from(&value.to_string())
    }
}

#[cfg(feature = "url")]
impl TryFrom<&QUrl> for url::Url {
    type Error = url::ParseError;

    fn try_from(value: &QUrl) -> Result<Self, Self::Error> {
        url::Url::parse(value.to_string().as_str())
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
    #[cfg(any(feature = "http", feature = "url"))]
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

    #[cfg(feature = "url")]
    #[test]
    fn test_url() {
        let url = url::Url::parse("https://github.com/kdab/cxx-qt").unwrap();
        let qurl = QUrl::from(&url);
        assert_eq!(url.to_string(), qurl.to_string());

        let url_url = url::Url::try_from(&qurl).unwrap();
        assert_eq!(url_url, url);
    }
}
