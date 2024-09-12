#[cxx_qt::bridge]
mod inheritance {
    extern "C++" {
        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        type QAbstractItemModel;
    }

    extern "C++Qt" {
        #[qobject]
        type MyStruct;
    }

    extern "RustQt" {
        #[qobject]
        #[base = QAbstractItemModel]
        type MyObject = super::MyObjectRust;
    }

    unsafe extern "RustQt" {
        /// Inherited hasChildren from the base class
        #[cxx_name = "hasChildren"]
        #[inherit]
        fn has_children_super(self: &MyObject, parent: &QModelIndex) -> bool;
    }

    extern "RustQt" {
        /// Inherited fetchMore from the base class
        #[inherit]
        unsafe fn fetch_more(self: Pin<&mut MyObject>, index: &QModelIndex);
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        #[cxx_override]
        fn data(self: &MyObject, _index: &QModelIndex, _role: i32) -> QVariant;

        #[qinvokable]
        #[cxx_override]
        fn has_children(self: &MyObject, _parent: &QModelIndex) -> bool;
    }
}
