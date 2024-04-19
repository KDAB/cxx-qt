// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::generator::naming::CombinedIdent;
use crate::parser::qobject::ParsedQObject;
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

/// Names for parts of a Q_OBJECT
pub struct QObjectName {
    // Store the ident so that cxx_qt_thread_method can use it later
    ident: Ident,
    /// The name of the C++ class
    pub cpp_class: CombinedIdent,
    /// The name of the Rust struct
    pub rust_struct: CombinedIdent,
    /// The name of the CxxQtThread
    pub cxx_qt_thread_class: Ident,
    /// The name of the Rust closure wrapper to be passed in to CxxQtThread
    pub cxx_qt_thread_queued_fn_struct: Ident,
}

impl From<&ParsedQObject> for QObjectName {
    fn from(qobject: &ParsedQObject) -> Self {
        Self::from_idents(
            qobject.name.rust_unqualified().clone(),
            qobject.qobject_ty.ident_right.clone(),
        )
    }
}

impl QObjectName {
    pub fn from_idents(ident_left: Ident, ident_right: Ident) -> Self {
        Self {
            cpp_class: CombinedIdent::from_ident(ident_left.clone()),
            rust_struct: CombinedIdent::from_ident(ident_right),
            cxx_qt_thread_class: cxx_qt_thread_class_from_ident(&ident_left),
            cxx_qt_thread_queued_fn_struct: cxx_qt_thread_queued_fn_struct_from_ident(&ident_left),
            ident: ident_left,
        }
    }

    /// For a given ident generate the mangled threading suffix ident
    pub fn cxx_qt_thread_method(&self, suffix: &str) -> Ident {
        format_ident!(
            "cxx_qt_ffi_{ident}_{suffix}",
            ident = self.ident.to_string().to_case(Case::Snake)
        )
    }
}

/// For a given ident generate the CxxQtThread ident
fn cxx_qt_thread_class_from_ident(ident: &Ident) -> Ident {
    format_ident!("{ident}CxxQtThread")
}

/// For a given ident generate the CxxQtThreadQueuedFn ident
fn cxx_qt_thread_queued_fn_struct_from_ident(ident: &Ident) -> Ident {
    format_ident!("{ident}CxxQtThreadQueuedFn")
}

impl CombinedIdent {
    /// For a given ident generate the Rust and C++ names
    fn from_ident(ident: Ident) -> Self {
        Self {
            cpp: ident.clone(),
            rust: ident,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::parser::qobject::tests::create_parsed_qobject;

    pub fn create_qobjectname() -> QObjectName {
        QObjectName::from(&create_parsed_qobject())
    }

    #[test]
    fn test_parsed_property() {
        let names = QObjectName::from(&create_parsed_qobject());
        assert_eq!(names.cpp_class.cpp, format_ident!("MyObject"));
        assert_eq!(names.cpp_class.rust, format_ident!("MyObject"));
        assert_eq!(names.rust_struct.cpp, format_ident!("MyObjectRust"));
        assert_eq!(names.rust_struct.rust, format_ident!("MyObjectRust"));
        assert_eq!(
            names.cxx_qt_thread_class,
            format_ident!("MyObjectCxxQtThread")
        );
        assert_eq!(
            names.cxx_qt_thread_queued_fn_struct,
            format_ident!("MyObjectCxxQtThreadQueuedFn")
        );

        assert_eq!(
            names.cxx_qt_thread_method("threading_clone"),
            "cxx_qt_ffi_my_object_threading_clone"
        );
        assert_eq!(
            names.cxx_qt_thread_method("threading_drop"),
            "cxx_qt_ffi_my_object_threading_drop"
        );
        assert_eq!(
            names.cxx_qt_thread_method("queue_boxed_fn"),
            "cxx_qt_ffi_my_object_queue_boxed_fn"
        );
    }
}
