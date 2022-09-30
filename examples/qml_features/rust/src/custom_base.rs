// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

/// Define a QModelIndex that is trivial for CXX
///
/// TODO: later this will likely be in cxx-qt-lib
#[repr(C)]
pub struct QModelIndex {
    _space: MaybeUninit<[usize; 3]>,
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
// TODO: later this will likely be in cxx-qt-lib
unsafe impl ExternType for QModelIndex {
    type Id = type_id!("QModelIndex");
    type Kind = cxx::kind::Trivial;
}

// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "custom_base")]
mod ffi {
    unsafe extern "C++" {
        include!("qabstractlistmodelcxx.h");

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        // Define the interface of the QModelIndex
        type QModelIndex = super::QModelIndex;
        fn row(self: &QModelIndex) -> i32;

        #[cxx_name = "beginInsertRows"]
        fn begin_insert_rows(self: Pin<&mut CustomBaseQt>, first: i32, last: i32);
        #[cxx_name = "endInsertRows"]
        fn end_insert_rows(self: Pin<&mut CustomBaseQt>);
    }

    #[cxx_qt::qobject(base = "QAbstractListModelCXX")]
    #[derive(Default)]
    pub struct CustomBase {
        vector: Vec<(u32, f64)>,
    }

    impl qobject::CustomBase {
        #[qinvokable]
        pub fn add(mut self: Pin<&mut Self>) {
            let count = self.rust().vector.len();
            self.as_mut().begin_insert_rows(count as i32, count as i32);
            self.as_mut()
                .vector_mut()
                .push((count as u32, (count as f64) / 3.0));
            self.as_mut().end_insert_rows();
        }
    }

    // QAbstractListModel implementation
    impl qobject::CustomBase {
        #[qinvokable(cxx_override)]
        fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
            if let Some((id, value)) = self.rust().vector.get(index.row() as usize) {
                return match role {
                    0 => QVariant::from(*id),
                    1 => QVariant::from(*value),
                    _ => QVariant::default(),
                };
            }

            QVariant::default()
        }

        #[qinvokable(cxx_override)]
        pub fn role_names_as_vec(&self) -> Vec<String> {
            vec!["id".to_owned(), "value".to_owned()]
        }

        #[qinvokable(cxx_override)]
        pub fn row_count(&self, _parent: &QModelIndex) -> i32 {
            self.rust().vector.len() as i32
        }
    }
}
// ANCHOR_END: book_macro_code
