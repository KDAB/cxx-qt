#[cxx_qt::bridge]
mod inheritance {
    extern "C++" {
        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }

    #[cxx_qt::qobject(base = "QAbstractItemModel")]
    #[derive(Default)]
    pub struct MyObject {
        data: Vec<i32>,
    }

    #[cxx_qt::inherit]
    unsafe extern "C++" {
        #[cxx_name = "hasChildren"]
        fn has_children_super(self: &qobject::MyObject, parent: &QModelIndex) -> bool;
    }

    #[cxx_qt::inherit]
    extern "C++" {
        unsafe fn fetch_more(self: Pin<&mut qobject::MyObject>, index: &QModelIndex);
    }

    impl qobject::MyObject {
        #[qinvokable(cxx_override)]
        pub fn data(&self, _index: &QModelIndex, _role: i32) -> QVariant {
            QVariant::default()
        }

        #[qinvokable(cxx_override)]
        pub fn has_children(&self, _parent: &QModelIndex) -> bool {
            false
        }
    }
}
