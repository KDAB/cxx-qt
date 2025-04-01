// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::parser::externqobject::ParsedExternQObject;
use crate::{
    naming::{Name, TypeNames},
    parser::qobject::ParsedQObject,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
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

    /// For a given ExternQObject, create the names associated with it for generation.
    pub fn from_extern_qobject(
        qobject: &ParsedExternQObject,
        type_names: &TypeNames,
    ) -> Result<Self> {
        Self::from_name_and_ident(&qobject.name, qobject.name.rust_unqualified(), type_names)
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

    /// For a given C++ function, generate a free function name specific to this class.
    /// This is then used can be used to wrap the free function as a new member function through
    /// CXX.
    pub fn cxx_qt_ffi_method(&self, cxx_name: &str) -> Name {
        let ident = format_ident!(
            "cxx_qt_ffi_{ident}_{cxx_name}",
            ident = self.name.cxx_unqualified(),
        );
        let mut name = Name::new(ident);
        if let Some(module) = self.name.module() {
            name = name.with_module(module.clone());
        }
        name.with_namespace("rust::cxxqt1".to_owned())
            .with_cxx_name(cxx_name.to_owned())
        // Could potentially add the rust name here, with an automatic conversion or provided
    }

    /// Returns the tokens of the namespace attribute to be added to a rust line, or no tokens if this instance has no namespace
    /// attribute looks like `#[namespace = "namespace::here"]`
    pub fn namespace_tokens(&self) -> TokenStream {
        if let Some(namespace) = self.name.namespace() {
            quote! { #[namespace = #namespace ] }
        } else {
            quote! {}
        }
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
        assert_eq!(names.name.rust_unqualified(), "MyObject");
        assert_eq!(names.rust_struct.cxx_unqualified(), "MyObjectRust");
        assert_eq!(names.rust_struct.rust_unqualified(), "MyObjectRust");
        assert_eq!(names.cxx_qt_thread_class, "MyObjectCxxQtThread");
        assert_eq!(
            names.cxx_qt_thread_queued_fn_struct,
            "MyObjectCxxQtThreadQueuedFn"
        );

        assert_eq!(
            names.cxx_qt_ffi_method("threading_clone").into_cxx_parts(),
            (
                format_ident!("cxx_qt_ffi_MyObject_threading_clone"),
                vec![
                    syn::parse_quote! { #[cxx_name="threading_clone"] },
                    syn::parse_quote! { #[namespace="rust::cxxqt1"] },
                ],
                syn::parse_quote! { qobject::cxx_qt_ffi_MyObject_threading_clone}
            )
        );
        // These have the same values for namespace, and Rust module, so no need to
        // assert those again
        assert_eq!(
            names.cxx_qt_ffi_method("threading_drop").rust_unqualified(),
            "cxx_qt_ffi_MyObject_threading_drop"
        );
        assert_eq!(
            names.cxx_qt_ffi_method("queue_boxed_fn").rust_unqualified(),
            "cxx_qt_ffi_MyObject_queue_boxed_fn"
        );
    }
}
