// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{
    naming::{Name, TypeNames},
    parser::qobject::ParsedQObject,
};
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::{Ident, Result};

/// Names for parts of a Q_OBJECT
pub struct QObjectNames {
    /// The name of the QObject itself.
    pub name: Name,
    /// The name of the inner Rust struct
    pub rust_struct: Name,
    /// The name of the CxxQtThread
    pub cxx_qt_thread_class: Ident,
    /// The name of the Rust closure wrapper to be passed in to CxxQtThread
    pub cxx_qt_thread_queued_fn_struct: Ident,
}

impl QObjectNames {
    /// For a given QObject, create the names associated with it for generation.
    pub fn from_qobject(qobject: &ParsedQObject, type_names: &TypeNames) -> Result<Self> {
        Self::from_name_and_ident(&qobject.name, &qobject.rust_type, type_names)
    }

    /// From the QObject name and Rust struct ident, create the names needed for generation.
    pub fn from_name_and_ident(
        qobject_name: &Name,
        ident_right: &Ident,
        type_names: &TypeNames,
    ) -> Result<Self> {
        Ok(Self {
            name: qobject_name.clone(),
            rust_struct: type_names.lookup(ident_right)?.clone(),
            cxx_qt_thread_class: cxx_qt_thread_class_from_ident(qobject_name.rust_unqualified()),
            cxx_qt_thread_queued_fn_struct: cxx_qt_thread_queued_fn_struct_from_ident(
                qobject_name.rust_unqualified(),
            ),
        })
    }

    // Only for mocking in tests
    #[cfg(test)]
    pub fn from_idents(ident_left: Ident, ident_right: Ident) -> Self {
        Self {
            name: Name::mock(&ident_left.to_string()),
            rust_struct: Name::mock(&ident_right.to_string()),
            cxx_qt_thread_class: cxx_qt_thread_class_from_ident(&ident_left),
            cxx_qt_thread_queued_fn_struct: cxx_qt_thread_queued_fn_struct_from_ident(&ident_left),
        }
    }

    /// For a given ident generate the mangled threading suffix ident
    pub fn cxx_qt_ffi_method(&self, suffix: &str) -> Ident {
        format_ident!(
            "cxx_qt_ffi_{ident}_{suffix}",
            ident = self.name.cxx_unqualified().to_case(Case::Snake)
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

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::parser::qobject::tests::create_parsed_qobject;

    pub fn create_qobjectname() -> QObjectNames {
        QObjectNames::from_qobject(&create_parsed_qobject(), &TypeNames::mock()).unwrap()
    }

    #[test]
    fn test_parsed_property() {
        let names =
            QObjectNames::from_qobject(&create_parsed_qobject(), &TypeNames::mock()).unwrap();
        assert_eq!(names.name.cxx_unqualified(), "MyObject");
        assert_eq!(names.name.rust_unqualified(), &format_ident!("MyObject"));
        assert_eq!(names.rust_struct.cxx_unqualified(), "MyObjectRust");
        assert_eq!(
            names.rust_struct.rust_unqualified(),
            &format_ident!("MyObjectRust")
        );
        assert_eq!(
            names.cxx_qt_thread_class,
            format_ident!("MyObjectCxxQtThread")
        );
        assert_eq!(
            names.cxx_qt_thread_queued_fn_struct,
            format_ident!("MyObjectCxxQtThreadQueuedFn")
        );

        assert_eq!(
            names.cxx_qt_ffi_method("threading_clone"),
            "cxx_qt_ffi_my_object_threading_clone"
        );
        assert_eq!(
            names.cxx_qt_ffi_method("threading_drop"),
            "cxx_qt_ffi_my_object_threading_drop"
        );
        assert_eq!(
            names.cxx_qt_ffi_method("queue_boxed_fn"),
            "cxx_qt_ffi_my_object_queue_boxed_fn"
        );
    }
}
