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

    unsafe extern "RustQt" {
        #[cxx_qt::qobject(qml_uri = "com.kdab.cxx_qt.demo", qml_version = "1.0")]
        #[qproperty(QString, string_hash)]
        #[qproperty(QString, string_list)]
        #[qproperty(QString, string_map)]
        #[qproperty(QString, string_set)]
        #[qproperty(QString, string_vector)]
        // Expose as a Q_PROPERTY so that QML tests can ensure that QVariantMap works
        #[qproperty(QMap_QString_QVariant, map)]
        type RustContainers = super::RustContainersRust;

        /// Reset all the containers
        #[qinvokable]
        fn reset(self: Pin<&mut qobject::RustContainers>);

        /// Append the given number to the vector container
        #[qinvokable]
        fn append_vector(self: Pin<&mut qobject::RustContainers>, value: i32);

        /// Append the given number to the list container
        #[qinvokable]
        fn append_list(self: Pin<&mut qobject::RustContainers>, value: i32);

        /// Insert the given number into the set container
        #[qinvokable]
        fn insert_set(self: Pin<&mut qobject::RustContainers>, value: i32);

        /// Insert the given string and variant to the hash container
        #[qinvokable]
        fn insert_hash(self: Pin<&mut qobject::RustContainers>, key: QString, value: QVariant);

        /// Insert the given string and variant to the map container
        #[qinvokable]
        fn insert_map(self: Pin<&mut qobject::RustContainers>, key: QString, value: QVariant);
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{
    QHash, QHashPair_QString_QVariant, QList, QMap, QMapPair_QString_QVariant, QSet, QString,
    QVariant, QVector,
};

/// A QObject which stores container types internally
///
/// It has Q_PROPERTYs which expose a string with the container's contents to show in QML
#[derive(Default)]
pub struct RustContainersRust {
    string_hash: QString,
    string_list: QString,
    string_map: QString,
    string_set: QString,
    string_vector: QString,

    pub(crate) hash: QHash<QHashPair_QString_QVariant>,
    pub(crate) list: QList<i32>,
    pub(crate) map: QMap<QMapPair_QString_QVariant>,
    pub(crate) set: QSet<i32>,
    pub(crate) vector: QVector<i32>,
}

// TODO: this will change to qobject::RustContainers once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::RustContainers {
    /// Reset all the containers
    fn reset(mut self: Pin<&mut Self>) {
        // Update the private rust fields via the rust_mut
        {
            let mut rust_mut = self.as_mut().rust_mut();
            rust_mut.hash = QHash::<QHashPair_QString_QVariant>::default();
            rust_mut.list = QList::<i32>::default();
            rust_mut.set = QSet::<i32>::default();
            rust_mut.vector = QVector::<i32>::default();
        }

        self.as_mut()
            .set_map(QMap::<QMapPair_QString_QVariant>::default());

        self.update_strings();
    }

    /// Append the given number to the vector container
    fn append_vector(mut self: Pin<&mut Self>, value: i32) {
        self.as_mut().rust_mut().vector.append(value);

        self.update_strings();
    }

    /// Append the given number to the list container
    fn append_list(mut self: Pin<&mut Self>, value: i32) {
        self.as_mut().rust_mut().list.append(value);

        self.update_strings();
    }

    /// Insert the given number into the set container
    fn insert_set(mut self: Pin<&mut Self>, value: i32) {
        self.as_mut().rust_mut().set.insert(value);

        self.update_strings();
    }

    /// Insert the given string and variant to the hash container
    fn insert_hash(mut self: Pin<&mut Self>, key: QString, value: QVariant) {
        self.as_mut().rust_mut().hash.insert(key, value);

        self.update_strings();
    }

    /// Insert the given string and variant to the map container
    fn insert_map(mut self: Pin<&mut Self>, key: QString, value: QVariant) {
        // Note: map is a Q_PROPERTY so ensure we manually trigger changed
        self.as_mut().rust_mut().map.insert(key, value);
        self.as_mut().map_changed();

        self.update_strings();
    }

    fn update_strings(mut self: Pin<&mut Self>) {
        let hash_items = self
            .as_ref()
            .rust()
            .hash
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
            .rust()
            .list
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        self.as_mut().set_string_list(QString::from(&list_items));

        let map_items = self
            .as_ref()
            .rust()
            .map
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
            .rust()
            .set
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        self.as_mut().set_string_set(QString::from(&set_items));

        let vector_items = self
            .as_ref()
            .rust()
            .vector
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        self.set_string_vector(QString::from(&vector_items));
    }
}
