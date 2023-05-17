// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how Qt container types can be used

/// A CXX-Qt bridge which shows how to use Qt container types
#[cxx_qt::bridge(cxx_file_stem = "rust_containers")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qhash.h");
        /// QHash<QString, QVariant> from cxx_qt_lib
        type QHash_QString_QVariant = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_QString_QVariant>;
        include!("cxx-qt-lib/qlist.h");
        /// QList<i32> from cxx_qt_lib
        type QList_i32 = cxx_qt_lib::QList<i32>;
        include!("cxx-qt-lib/qmap.h");
        /// QMap<QString, QVariant> from cxx_qt_lib
        type QMap_QString_QVariant = cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant>;
        include!("cxx-qt-lib/qset.h");
        /// QSet<i32> from cxx_qt_lib
        type QSet_i32 = cxx_qt_lib::QSet<i32>;
        include!("cxx-qt-lib/qstring.h");
        /// QString from cxx_qt_lib
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvariant.h");
        /// QVariant from cxx_qt_lib
        type QVariant = cxx_qt_lib::QVariant;
        include!("cxx-qt-lib/qvector.h");
        /// QVector<i32> from cxx_qt_lib
        type QVector_i32 = cxx_qt_lib::QVector<i32>;
    }

    /// A QObject which stores container types internally
    ///
    /// It has Q_PROPERTYs which expose a string with the container's contents to show in QML
    #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
    #[derive(Default)]
    pub struct RustContainers {
        #[qproperty]
        string_hash: QString,
        #[qproperty]
        string_list: QString,
        #[qproperty]
        string_map: QString,
        #[qproperty]
        string_set: QString,
        #[qproperty]
        string_vector: QString,

        hash: QHash_QString_QVariant,
        list: QList_i32,
        // Expose as a Q_PROPERTY so that QML tests can ensure that QVariantMap works
        #[qproperty]
        map: QMap_QString_QVariant,
        set: QSet_i32,
        vector: QVector_i32,
    }

    impl qobject::RustContainers {
        /// Reset all the containers
        #[qinvokable]
        pub fn reset(mut self: Pin<&mut Self>) {
            self.as_mut().set_hash(QHash_QString_QVariant::default());
            self.as_mut().set_list(QList_i32::default());
            self.as_mut().set_map(QMap_QString_QVariant::default());
            self.as_mut().set_set(QSet_i32::default());
            self.as_mut().set_vector(QVector_i32::default());

            self.update_strings();
        }

        /// Append the given number to the vector container
        #[qinvokable]
        pub fn append_vector(mut self: Pin<&mut Self>, value: i32) {
            self.as_mut().vector_mut().append(value);

            self.update_strings();
        }

        /// Append the given number to the list container
        #[qinvokable]
        pub fn append_list(mut self: Pin<&mut Self>, value: i32) {
            self.as_mut().list_mut().append(value);

            self.update_strings();
        }

        /// Insert the given number into the set container
        #[qinvokable]
        pub fn insert_set(mut self: Pin<&mut Self>, value: i32) {
            self.as_mut().set_mut().insert(value);

            self.update_strings();
        }

        /// Insert the given string and variant to the hash container
        #[qinvokable]
        pub fn insert_hash(mut self: Pin<&mut Self>, key: QString, value: QVariant) {
            self.as_mut().hash_mut().insert(key, value);

            self.update_strings();
        }

        /// Insert the given string and variant to the map container
        #[qinvokable]
        pub fn insert_map(mut self: Pin<&mut Self>, key: QString, value: QVariant) {
            // SAFETY: map is a Q_PROPERTY so ensure we manually trigger changed
            unsafe {
                self.as_mut().map_mut().insert(key, value);
                self.as_mut().map_changed();
            }

            self.update_strings();
        }

        fn update_strings(mut self: Pin<&mut Self>) {
            let hash_items = self
                .as_ref()
                .hash()
                .iter()
                .map(|(key, value)| {
                    let value = value.value::<i32>().unwrap_or(0);
                    format!("{key} => {value}")
                })
                .collect::<Vec<String>>()
                .join(", ");
            self.as_mut().set_string_hash(QString::from(&hash_items));

            let list_items = self
                .as_ref()
                .list()
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            self.as_mut().set_string_list(QString::from(&list_items));

            let map_items = self
                .as_ref()
                .map()
                .iter()
                .map(|(key, value)| {
                    let value = value.value::<i32>().unwrap_or(0);
                    format!("{key} => {value}")
                })
                .collect::<Vec<String>>()
                .join(", ");
            self.as_mut().set_string_map(QString::from(&map_items));

            let set_items = self
                .as_ref()
                .set()
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            self.as_mut().set_string_set(QString::from(&set_items));

            let vector_items = self
                .as_ref()
                .vector()
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            self.set_string_vector(QString::from(&vector_items));
        }
    }
}
