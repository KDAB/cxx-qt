// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a custom base class and inheritance can be used

/// A CXX-Qt bridge which shows a custom base class and inheritance can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "custom_base_class")]
pub mod qobject {
    // ANCHOR: book_base_include
    unsafe extern "C++" {
        include!(< QAbstractListModel >);
        // ANCHOR_END: book_base_include

        include!("cxx-qt-lib/qhash.h");
        /// QHash<i32, QByteArray> from cxx_qt_lib
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;

        include!("cxx-qt-lib/qvariant.h");
        /// QVariant from cxx_qt_lib
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qmodelindex.h");
        /// QModelIndex from cxx_qt_lib
        type QModelIndex = cxx_qt_lib::QModelIndex;

        include!("cxx-qt-lib/qvector.h");
        /// QVector<i32> from cxx_qt_lib
        type QVector_i32 = cxx_qt_lib::QVector<i32>;
    }

    // ANCHOR: book_inherit_qalm
    // ANCHOR: book_qobject_base
    extern "RustQt" {
        #[cxx_qt::qobject(
            base = "QAbstractListModel",
            qml_uri = "com.kdab.cxx_qt.demo",
            qml_version = "1.0"
        )]
        type CustomBaseClass = super::CustomBaseClassRust;
    }
    // ANCHOR_END: book_qobject_base
    // ANCHOR_END: book_inherit_qalm

    // Enabling threading on the qobject
    impl cxx_qt::Threading for CustomBaseClass {}

    // ANCHOR: book_qsignals_inherit
    unsafe extern "RustQt" {
        /// Inherit the DataChanged signal from the QAbstractListModel base class
        #[inherit]
        #[qsignal]
        fn data_changed(
            self: Pin<&mut CustomBaseClass>,
            top_left: &QModelIndex,
            bottom_right: &QModelIndex,
            roles: &QVector_i32,
        );
    }
    // ANCHOR_END: book_qsignals_inherit

    unsafe extern "RustQt" {
        /// Add a new row to the QAbstractListModel on the current thread
        #[qinvokable]
        fn add(self: Pin<&mut CustomBaseClass>);

        /// On a background thread, add a given number of rows to the QAbstractListModel
        #[qinvokable]
        fn add_on_thread(self: Pin<&mut CustomBaseClass>, mut counter: i32);
    }

    // ANCHOR: book_inherit_clear_signature
    unsafe extern "RustQt" {
        /// Clear the rows in the QAbstractListModel
        #[qinvokable]
        pub fn clear(self: Pin<&mut CustomBaseClass>);
    }
    // ANCHOR_END: book_inherit_clear_signature

    unsafe extern "RustQt" {
        /// Multiply the number in the row with the given index by the given factor
        #[qinvokable]
        pub fn multiply(self: Pin<&mut CustomBaseClass>, index: i32, factor: f64);

        /// Remove the row with the given index
        #[qinvokable]
        pub fn remove(self: Pin<&mut CustomBaseClass>, index: i32);
    }

    // ANCHOR: book_inherit_qalm_impl_unsafe
    // Create Rust bindings for C++ functions of the base class (QAbstractItemModel)
    extern "RustQt" {
        /// Inherited beginInsertRows from the base class
        #[inherit]
        unsafe fn begin_insert_rows(
            self: Pin<&mut CustomBaseClass>,
            parent: &QModelIndex,
            first: i32,
            last: i32,
        );
        /// Inherited endInsertRows from the base class
        #[inherit]
        unsafe fn end_insert_rows(self: Pin<&mut CustomBaseClass>);

        /// Inherited beginRemoveRows from the base class
        #[inherit]
        unsafe fn begin_remove_rows(
            self: Pin<&mut CustomBaseClass>,
            parent: &QModelIndex,
            first: i32,
            last: i32,
        );
        /// Inherited endRemoveRows from the base class
        #[inherit]
        unsafe fn end_remove_rows(self: Pin<&mut CustomBaseClass>);

        /// Inherited beginResetModel from the base class
        #[inherit]
        unsafe fn begin_reset_model(self: Pin<&mut CustomBaseClass>);
        /// Inherited endResetModel from the base class
        #[inherit]
        unsafe fn end_reset_model(self: Pin<&mut CustomBaseClass>);
    }
    // ANCHOR_END: book_inherit_qalm_impl_unsafe

    // ANCHOR: book_inherit_qalm_impl_safe
    unsafe extern "RustQt" {
        /// Inherited canFetchMore from the base class
        #[cxx_name = "canFetchMore"]
        #[inherit]
        fn base_can_fetch_more(self: &CustomBaseClass, parent: &QModelIndex) -> bool;

        /// Inherited index from the base class
        #[inherit]
        fn index(
            self: &CustomBaseClass,
            row: i32,
            column: i32,
            parent: &QModelIndex,
        ) -> QModelIndex;
    }
    // ANCHOR_END: book_inherit_qalm_impl_safe

    // QAbstractListModel implementation
    // ANCHOR: book_inherit_data_signature
    unsafe extern "RustQt" {
        #[qinvokable(cxx_override)]
        fn data(self: &CustomBaseClass, index: &QModelIndex, role: i32) -> QVariant;
    }
    // ANCHOR_END: book_inherit_data_signature

    // ANCHOR: book_inherit_can_fetch_more_signature
    unsafe extern "RustQt" {
        /// Return whether the base class can fetch more
        // Example of overriding a C++ virtual method and calling the base class implementation.
        #[qinvokable(cxx_override)]
        fn can_fetch_more(self: &CustomBaseClass, parent: &QModelIndex) -> bool;
    }
    // ANCHOR_END: book_inherit_can_fetch_more_signature

    unsafe extern "RustQt" {
        /// Return the role names for the QAbstractListModel
        #[qinvokable(cxx_override)]
        fn role_names(self: &CustomBaseClass) -> QHash_i32_QByteArray;

        /// Return the row count for the QAbstractListModel
        #[qinvokable(cxx_override)]
        fn row_count(self: &CustomBaseClass, _parent: &QModelIndex) -> i32;
    }
}

use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QByteArray, QHash, QHashPair_i32_QByteArray, QModelIndex, QVariant, QVector};

/// A struct which inherits from QAbstractListModel
#[derive(Default)]
pub struct CustomBaseClassRust {
    pub(crate) id: u32,
    pub(crate) vector: Vec<(u32, f64)>,
}

impl qobject::CustomBaseClass {
    /// Add a new row to the QAbstractListModel on the current thread
    pub fn add(self: Pin<&mut Self>) {
        self.add_cpp_context();
    }

    /// On a background thread, add a given number of rows to the QAbstractListModel
    pub fn add_on_thread(self: Pin<&mut Self>, mut counter: i32) {
        let qt_thread = self.qt_thread();

        std::thread::spawn(move || {
            while counter > 0 {
                counter -= 1;
                std::thread::sleep(std::time::Duration::from_millis(250));

                // Use our add helper to add a row on the Qt event loop
                // as seen in the threading demo channels could be used to pass info
                qt_thread
                    .queue(|custom_base_class| {
                        custom_base_class.add_cpp_context();
                    })
                    .unwrap();
            }
        });
    }

    fn add_cpp_context(mut self: Pin<&mut Self>) {
        let count = self.as_ref().rust().vector.len();
        unsafe {
            self.as_mut()
                .begin_insert_rows(&QModelIndex::default(), count as i32, count as i32);
            let id = self.as_ref().rust().id;
            self.as_mut().rust_mut().id = id + 1;
            self.as_mut()
                .rust_mut()
                .vector
                .push((id, (id as f64) / 3.0));
            self.as_mut().end_insert_rows();
        }
    }
}

// ANCHOR: book_inherit_clear
impl qobject::CustomBaseClass {
    /// Clear the rows in the QAbstractListModel
    pub fn clear(mut self: Pin<&mut Self>) {
        unsafe {
            self.as_mut().begin_reset_model();
            self.as_mut().rust_mut().id = 0;
            self.as_mut().rust_mut().vector.clear();
            self.as_mut().end_reset_model();
        }
    }
}
// ANCHOR_END: book_inherit_clear

impl qobject::CustomBaseClass {
    /// Multiply the number in the row with the given index by the given factor
    pub fn multiply(mut self: Pin<&mut Self>, index: i32, factor: f64) {
        if let Some((_, value)) = self.as_mut().rust_mut().vector.get_mut(index as usize) {
            *value *= factor;

            // Emit dataChanged for the index and value role
            let model_index = self.index(index, 0, &QModelIndex::default());
            let mut vector_roles = QVector::<i32>::default();
            vector_roles.append(1);
            self.as_mut()
                .data_changed(&model_index, &model_index, &vector_roles);
        }
    }

    /// Remove the row with the given index
    pub fn remove(mut self: Pin<&mut Self>, index: i32) {
        if index < 0 || (index as usize) >= self.as_ref().rust().vector.len() {
            return;
        }

        unsafe {
            self.as_mut()
                .begin_remove_rows(&QModelIndex::default(), index, index);
            self.as_mut().rust_mut().vector.remove(index as usize);
            self.as_mut().end_remove_rows();
        }
    }
}

// QAbstractListModel implementation
//
// ANCHOR: book_inherit_data
impl qobject::CustomBaseClass {
    /// i32 representing the id role
    pub const ID_ROLE: i32 = 0;
    /// i32 representing the value role
    pub const VALUE_ROLE: i32 = 1;

    fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        if let Some((id, value)) = self.rust().vector.get(index.row() as usize) {
            return match role {
                Self::ID_ROLE => QVariant::from(id),
                Self::VALUE_ROLE => QVariant::from(value),
                _ => QVariant::default(),
            };
        }

        QVariant::default()
    }
}
// ANCHOR_END: book_inherit_data

// ANCHOR: book_inherit_can_fetch_more
impl qobject::CustomBaseClass {
    /// Return whether the base class can fetch more
    // Example of overriding a C++ virtual method and calling the base class implementation.
    pub fn can_fetch_more(&self, parent: &QModelIndex) -> bool {
        self.base_can_fetch_more(parent)
    }
}
// ANCHOR_END: book_inherit_can_fetch_more

impl qobject::CustomBaseClass {
    /// Return the role names for the QAbstractListModel
    pub fn role_names(&self) -> QHash<QHashPair_i32_QByteArray> {
        let mut roles = QHash::<QHashPair_i32_QByteArray>::default();
        roles.insert(Self::ID_ROLE, QByteArray::from("id"));
        roles.insert(Self::VALUE_ROLE, QByteArray::from("value"));
        roles
    }

    /// Return the row count for the QAbstractListModel
    pub fn row_count(&self, _parent: &QModelIndex) -> i32 {
        self.rust().vector.len() as i32
    }
}
// ANCHOR_END: book_macro_code
