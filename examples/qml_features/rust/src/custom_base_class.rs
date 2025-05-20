// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a custom base class and inheritance can be used

/// A CXX-Qt bridge which shows a custom base class and inheritance can be used
// ANCHOR: book_macro_code
#[cxx_qt::bridge]
pub mod qobject {
    // ANCHOR: book_base_include
    unsafe extern "C++Qt" {
        include!(<QtCore/QAbstractListModel>);
        /// Base for Qt type
        #[qobject]
        type QAbstractListModel;
    }
    // ANCHOR_END: book_base_include

    unsafe extern "C++" {
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

    #[qenum(CustomBaseClass)]
    /// Roles for the CustomBaseClass list model
    enum Roles {
        /// The index of the row
        Id,
        /// The value of the row
        Value,
    }

    // ANCHOR: book_qenum_in_qobject
    #[qenum(CustomBaseClass)]
    /// State of the CustomBaseClass list model
    enum State {
        /// Another item is being added in the background
        Running,
        /// No items are being added in the background
        Idle,
    }
    // ANCHOR_END: book_qenum_in_qobject

    // ANCHOR: book_inherit_qalm
    // ANCHOR: book_qobject_base
    extern "RustQt" {
        #[qobject]
        #[base = QAbstractListModel]
        type AbstractBaseClass = super::AbstractBaseClassRust;

        #[qobject]
        #[base = AbstractBaseClass]
        #[qml_element]
        #[qproperty(State, state)]
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
        #[cxx_name = "dataChanged"]
        fn data_changed(
            self: Pin<&mut CustomBaseClass>,
            top_left: &QModelIndex,
            bottom_right: &QModelIndex,
            roles: &QVector_i32,
        );
    }
    // ANCHOR_END: book_qsignals_inherit

    extern "RustQt" {
        /// Log the state of the abstract class
        #[qinvokable]
        #[cxx_virtual]
        #[cxx_pure]
        fn log(self: &AbstractBaseClass);

        /// Override to Log the state of the custom base class
        #[qinvokable]
        #[cxx_override]
        fn log(self: &CustomBaseClass);

        /// Add a new row to the QAbstractListModel on the current thread
        #[qinvokable]
        fn add(self: Pin<&mut CustomBaseClass>);

        #[cxx_name = "addOnThreadDelayed"]
        #[qinvokable]
        /// On a background thread, add a given number of rows to the QAbstractListModel with a
        /// configurable delay
        fn add_on_thread_delayed(self: Pin<&mut CustomBaseClass>, counter: i32, delay_ms: u64);

        /// On a background thread, add a given number of rows to the QAbstractListModel
        /// Use a standard delay of 250ms per item
        #[qinvokable]
        #[cxx_virtual]
        #[cxx_name = "addOnThread"]
        fn add_on_thread(self: Pin<&mut CustomBaseClass>, counter: i32);
    }

    // ANCHOR: book_inherit_clear_signature
    extern "RustQt" {
        /// Clear the rows in the QAbstractListModel
        #[qinvokable]
        pub fn clear(self: Pin<&mut CustomBaseClass>);
    }
    // ANCHOR_END: book_inherit_clear_signature

    extern "RustQt" {
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
        /// # Safety
        ///
        /// Inherited beginInsertRows from the base class.
        /// If you call begin_insert_rows, it is your responsibility to ensure end_insert_rows is called
        #[inherit]
        #[cxx_name = "beginInsertRows"]
        unsafe fn begin_insert_rows(
            self: Pin<&mut CustomBaseClass>,
            parent: &QModelIndex,
            first: i32,
            last: i32,
        );
        /// # Safety
        ///
        /// Inherited endInsertRows from the base class.
        /// If you call `begin_insert_rows`, it is your responsibility to ensure `end_insert_rows` is called
        #[inherit]
        #[cxx_name = "endInsertRows"]
        unsafe fn end_insert_rows(self: Pin<&mut CustomBaseClass>);

        /// # Safety
        ///
        /// Inherited beginRemoveRows from the base class.
        /// If you call `begin_remove_rows`, it is your responsibility to ensure `end_remove_rows` is called
        #[inherit]
        #[cxx_name = "beginRemoveRows"]
        unsafe fn begin_remove_rows(
            self: Pin<&mut CustomBaseClass>,
            parent: &QModelIndex,
            first: i32,
            last: i32,
        );
        /// # Safety
        ///
        /// Inherited endRemoveRows from the base class.
        /// If you call `begin_remove_rows`, it is your responsibility to ensure `end_remove_rows` is called
        #[inherit]
        #[cxx_name = "endRemoveRows"]
        unsafe fn end_remove_rows(self: Pin<&mut CustomBaseClass>);

        /// # Safety
        ///
        /// Inherited beginResetModel from the base class.
        /// If you call `begin_reset_model`, it is your responsibility to ensure `end_reset_model` is called
        #[inherit]
        #[cxx_name = "beginResetModel"]
        unsafe fn begin_reset_model(self: Pin<&mut CustomBaseClass>);
        /// # Safety
        ///
        /// Inherited endResetModel from the base class.
        /// If you call `begin_reset_model`, it is your responsibility to ensure `end_reset_model` is called
        #[inherit]
        #[cxx_name = "endResetModel"]
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
    extern "RustQt" {
        #[qinvokable]
        #[cxx_override]
        fn data(self: &CustomBaseClass, index: &QModelIndex, role: i32) -> QVariant;
    }
    // ANCHOR_END: book_inherit_data_signature

    // ANCHOR: book_inherit_can_fetch_more_signature
    extern "RustQt" {
        /// Return whether the base class can fetch more
        // Example of overriding a C++ virtual method and calling the base class implementation.
        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "canFetchMore"]
        fn can_fetch_more(self: &CustomBaseClass, parent: &QModelIndex) -> bool;
    }
    // ANCHOR_END: book_inherit_can_fetch_more_signature

    extern "RustQt" {
        /// Return the role names for the QAbstractListModel
        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "roleNames"]
        fn role_names(self: &CustomBaseClass) -> QHash_i32_QByteArray;

        /// Return the row count for the QAbstractListModel
        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "rowCount"]
        fn row_count(self: &CustomBaseClass, _parent: &QModelIndex) -> i32;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = CustomBaseClass]
        type TransitiveInheritance = super::TransitiveInheritanceRust;

        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "addOnThread"]
        fn add_on_thread(self: Pin<&mut TransitiveInheritance>, counter: i32);

        #[cxx_name = "addOnThreadDelayed"]
        #[inherit]
        /// On a background thread, add a given number of rows to the QAbstractListModel with a
        /// configurable delay
        fn add_on_thread_delayed(
            self: Pin<&mut TransitiveInheritance>,
            counter: i32,
            delay_ms: u64,
        );
    }
}

use crate::custom_base_class::qobject::{
    AbstractBaseClass, CustomBaseClass, QAbstractListModel, QObject,
};
use core::pin::Pin;
use cxx_qt::{casting::Upcast, impl_transitive_cast, CxxQtType, Threading};
use cxx_qt_lib::{QByteArray, QHash, QHashPair_i32_QByteArray, QModelIndex, QVariant, QVector};

impl_transitive_cast!(
    CustomBaseClass,
    AbstractBaseClass,
    QAbstractListModel,
    QObject
);

impl Default for qobject::State {
    fn default() -> Self {
        Self::Idle
    }
}

/// A struct which inherits from QAbstractListModel
#[derive(Default)]
pub struct AbstractBaseClassRust {}

/// A struct which inherits from our custom abstract parent
#[derive(Default)]
pub struct CustomBaseClassRust {
    state: qobject::State,
    pending_adds: i32,

    pub(crate) id: u32,
    pub(crate) vector: Vec<(u32, f64)>,
}

impl qobject::CustomBaseClass {
    /// Virtual method for logging type
    pub fn log(self: &CustomBaseClass) {
        println!(
            "state: {}\npending adds: {}\nid: {}\nvector: {:?}\n",
            self.state.repr, self.pending_adds, self.id, self.vector
        );
    }

    /// Add a new row to the QAbstractListModel on the current thread
    pub fn add(self: Pin<&mut Self>) {
        self.add_cpp_context();
    }

    /// On a background thread, add a given number of rows to the QAbstractListModel
    /// Use delay_ms to add a delay between adding each row
    pub fn add_on_thread_delayed(mut self: Pin<&mut Self>, mut counter: i32, delay_ms: u64) {
        let qt_thread = self.qt_thread();

        self.as_mut().rust_mut().pending_adds += counter;
        self.as_mut().set_state(qobject::State::Running);

        std::thread::spawn(move || {
            while counter > 0 {
                counter -= 1;
                if delay_ms > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(delay_ms));
                }

                // Use our add helper to add a row on the Qt event loop
                // as seen in the threading demo channels could be used to pass info
                qt_thread
                    .queue(|mut this| {
                        this.as_mut().add_cpp_context();
                        this.as_mut().rust_mut().pending_adds -= 1;
                        if this.pending_adds == 0 {
                            this.set_state(qobject::State::Idle);
                        }
                    })
                    .unwrap();
            }
        });
    }

    /// On a background thread, add a given number of rows to the QAbstractListModel
    /// Use a standard delay of 250 ms per item
    pub fn add_on_thread(self: Pin<&mut Self>, counter: i32) {
        self.add_on_thread_delayed(counter, 250);
    }

    fn add_cpp_context(mut self: Pin<&mut Self>) {
        let count = self.vector.len();
        unsafe {
            self.as_mut()
                .begin_insert_rows(&QModelIndex::default(), count as i32, count as i32);
            let id = self.id;
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
        if index < 0 || (index as usize) >= self.vector.len() {
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
    /// Retrieve the data for a given index and role
    pub fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let role = qobject::Roles { repr: role };
        if let Some((id, value)) = self.vector.get(index.row() as usize) {
            return match role {
                qobject::Roles::Id => QVariant::from(id),
                qobject::Roles::Value => QVariant::from(value),
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
        roles.insert(qobject::Roles::Id.repr, QByteArray::from("id"));
        roles.insert(qobject::Roles::Value.repr, QByteArray::from("value"));
        roles
    }

    /// Return the row count for the QAbstractListModel
    pub fn row_count(&self, _parent: &QModelIndex) -> i32 {
        self.vector.len() as i32
    }
}
// ANCHOR_END: book_macro_code

/// This struct demonstrates that CXX-Qt QObjects can derive from other CXX-Qt QObjects
/// It derives from CustomBaseClass and overrides the length of the standard delay.
#[derive(Default)]
pub struct TransitiveInheritanceRust {}

impl qobject::TransitiveInheritance {
    /// This function adds values on a thread with a shortened delay of 20ms.
    fn add_on_thread(self: Pin<&mut Self>, counter: i32) {
        self.add_on_thread_delayed(counter, 20);
    }
}
