#[cxx_qt::bridge]
mod qjsvaluelist {
    unsafe extern "C++" {
        include!("cxx-qt-lib-extras/qjsvalue.h");
        type QJSValue = crate::qml::QJSValue;

        include!("cxx-qt-lib-extras/qjsvaluelist.h");
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        type QJSValueList;

        #[rust_name = "cxx_clear"]
        fn clear(self: Pin<&mut QJSValueList>);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QJSValueList, _: &QJSValue) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[rust_name = "qjsvaluelist_new"]
        fn qjsvaluelistNew() -> UniquePtr<QJSValueList>;
        #[rust_name = "qjsvaluelist_clone"]
        fn qjsvaluelistClone(other: &QJSValueList) -> UniquePtr<QJSValueList>;
    }
}

pub use qjsvaluelist::QJSValueList;

impl QJSValueList {
    pub fn new() -> cxx::UniquePtr<Self> {
        qjsvaluelist::qjsvaluelist_new()
    }

    pub fn clone(self: &Self) -> cxx::UniquePtr<Self> {
        qjsvaluelist::qjsvaluelist_clone(self)
    }
}
