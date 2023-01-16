// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "custom_base_class")]
mod ffi {
    // ANCHOR: book_base_include
    unsafe extern "C++" {
        include!("qabstractlistmodelcxx.h");
        // ANCHOR_END: book_base_include

        include!("cxx-qt-lib/qhash.h");
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;

        include!("cxx-qt-lib/qvector.h");
        type QVector_i32 = cxx_qt_lib::QVector<i32>;

        #[cxx_name = "beginInsertRows"]
        fn begin_insert_rows(self: Pin<&mut CustomBaseClassQt>, first: i32, last: i32);
        #[cxx_name = "endInsertRows"]
        fn end_insert_rows(self: Pin<&mut CustomBaseClassQt>);

        #[cxx_name = "beginRemoveRows"]
        fn begin_remove_rows(self: Pin<&mut CustomBaseClassQt>, first: i32, last: i32);
        #[cxx_name = "endRemoveRows"]
        fn end_remove_rows(self: Pin<&mut CustomBaseClassQt>);

        #[cxx_name = "beginResetModel"]
        fn begin_reset_model(self: Pin<&mut CustomBaseClassQt>);
        #[cxx_name = "endResetModel"]
        fn end_reset_model(self: Pin<&mut CustomBaseClassQt>);

        fn index(
            self: &CustomBaseClassQt,
            row: i32,
            column: i32,
            parent: &QModelIndex,
        ) -> QModelIndex;
    }

    // ANCHOR: book_qobject_base
    #[cxx_qt::qobject(base = "QAbstractListModelCXX")]
    #[derive(Default)]
    pub struct CustomBaseClass {
        // ANCHOR_END: book_qobject_base
        id: u32,
        vector: Vec<(u32, f64)>,
    }

    // ANCHOR: book_qsignals_inherit
    #[cxx_qt::qsignals(CustomBaseClass)]
    pub enum Signals<'a> {
        #[inherit]
        DataChanged {
            top_left: &'a QModelIndex,
            bottom_right: &'a QModelIndex,
            roles: &'a QVector_i32,
        },
    }
    // ANCHOR_END: book_qsignals_inherit

    impl qobject::CustomBaseClass {
        #[qinvokable]
        pub fn add(self: Pin<&mut Self>) {
            self.add_cpp_context();
        }

        #[qinvokable]
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
            let count = self.vector().len();
            self.as_mut().begin_insert_rows(count as i32, count as i32);
            let id = *self.id();
            self.as_mut().set_id(id + 1);
            self.as_mut().vector_mut().push((id, (id as f64) / 3.0));
            self.as_mut().end_insert_rows();
        }

        #[qinvokable]
        pub fn clear(mut self: Pin<&mut Self>) {
            self.as_mut().begin_reset_model();
            self.as_mut().set_id(0);
            self.as_mut().vector_mut().clear();
            self.as_mut().end_reset_model();
        }

        #[qinvokable]
        pub fn multiply(mut self: Pin<&mut Self>, index: i32, factor: f64) {
            if let Some((_, value)) = self.as_mut().vector_mut().get_mut(index as usize) {
                *value *= factor;

                // Emit dataChanged for the index and value role
                let model_index = self.index(index, 0, &QModelIndex::default());
                let mut vector_roles = QVector_i32::default();
                vector_roles.append(1);
                self.as_mut().emit(Signals::DataChanged {
                    top_left: &model_index,
                    bottom_right: &model_index,
                    roles: &vector_roles,
                });
            }
        }

        #[qinvokable]
        pub fn remove(mut self: Pin<&mut Self>, index: i32) {
            if index < 0 || (index as usize) >= self.vector().len() {
                return;
            }

            self.as_mut().begin_remove_rows(index, index);
            self.as_mut().vector_mut().remove(index as usize);
            self.as_mut().end_remove_rows();
        }
    }

    // QAbstractListModel implementation
    impl qobject::CustomBaseClass {
        #[qinvokable(cxx_override)]
        fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
            if let Some((id, value)) = self.rust().vector.get(index.row() as usize) {
                return match role {
                    0 => QVariant::from(id),
                    1 => QVariant::from(value),
                    _ => QVariant::default(),
                };
            }

            QVariant::default()
        }

        #[qinvokable(cxx_override)]
        pub fn role_names(&self) -> QHash_i32_QByteArray {
            let mut roles = QHash_i32_QByteArray::default();
            roles.insert(0, cxx_qt_lib::QByteArray::from("id"));
            roles.insert(1, cxx_qt_lib::QByteArray::from("value"));
            roles
        }

        #[qinvokable(cxx_override)]
        pub fn row_count(&self, _parent: &QModelIndex) -> i32 {
            self.rust().vector.len() as i32
        }
    }
}
// ANCHOR_END: book_macro_code
