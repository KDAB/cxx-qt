// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge(cxx_file_stem = "rust_containers")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qhash.h");
        type QHash_QString_QVariant = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_QString_QVariant>;
        include!("cxx-qt-lib/qlist.h");
        type QList_i32 = cxx_qt_lib::QList<i32>;
        include!("cxx-qt-lib/qmap.h");
        type QMap_QString_QVariant = cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant>;
        include!("cxx-qt-lib/qset.h");
        type QSet_i32 = cxx_qt_lib::QSet<i32>;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
        include!("cxx-qt-lib/qvector.h");
        type QVector_i32 = cxx_qt_lib::QVector<i32>;
    }

    #[cxx_qt::qobject]
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
        map: QMap_QString_QVariant,
        set: QSet_i32,
        vector: QVector_i32,
    }

    impl qobject::RustContainers {
        #[qinvokable]
        pub fn reset(mut self: Pin<&mut Self>) {
            self.as_mut().set_hash(QHash_QString_QVariant::default());
            self.as_mut().set_list(QList_i32::default());
            self.as_mut().set_map(QMap_QString_QVariant::default());
            self.as_mut().set_set(QSet_i32::default());
            self.as_mut().set_vector(QVector_i32::default());

            self.update_strings();
        }

        #[qinvokable]
        pub fn append_vector(mut self: Pin<&mut Self>, value: i32) {
            self.as_mut().vector_mut().append(value);

            self.update_strings();
        }

        #[qinvokable]
        pub fn append_list(mut self: Pin<&mut Self>, value: i32) {
            self.as_mut().list_mut().append(value);

            self.update_strings();
        }

        #[qinvokable]
        pub fn insert_set(mut self: Pin<&mut Self>, value: i32) {
            self.as_mut().set_mut().insert(value);

            self.update_strings();
        }

        #[qinvokable]
        pub fn insert_hash(mut self: Pin<&mut Self>, key: QString, value: QVariant) {
            self.as_mut().hash_mut().insert(key, value);

            self.update_strings();
        }

        #[qinvokable]
        pub fn insert_map(mut self: Pin<&mut Self>, key: QString, value: QVariant) {
            self.as_mut().map_mut().insert(key, value);

            self.update_strings();
        }

        pub fn update_strings(mut self: Pin<&mut Self>) {
            let hash_items = self
                .as_ref()
                .hash()
                .iter()
                .map(|(key, value)| {
                    let value = value.try_value::<i32>().unwrap_or(0);
                    format!("{} => {}", key, value)
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
                    let value = value.try_value::<i32>().unwrap_or(0);
                    format!("{} => {}", key, value)
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
