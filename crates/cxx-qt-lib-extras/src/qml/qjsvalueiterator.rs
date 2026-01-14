use cxx::UniquePtr;

use crate::qml::QJSValue;

#[cxx_qt::bridge]
mod qjsvalueiterator {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib-extras/qjsvalueiterator.h");
        type QJSValueIterator;

        type QJSValue = crate::qml::QJSValue;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        fn qjsvalueiterator_new(value: &QJSValue) -> UniquePtr<QJSValueIterator>;
        fn qjsvalueiterator_value(iterator: &QJSValueIterator) -> UniquePtr<QJSValue>;

        #[rust_name = "qjsvalueiterator_name"]
        fn name(self: &QJSValueIterator) -> QString;
        #[rust_name = "qjsvalueiterator_has_next"]
        fn hasNext(self: &QJSValueIterator) -> bool;
        #[rust_name = "qjsvalueiterator_next"]
        fn next(self: Pin<&mut QJSValueIterator>) -> bool;
    }
}

pub use qjsvalueiterator::QJSValueIterator;

impl QJSValueIterator {
    pub fn new(value: &QJSValue) -> UniquePtr<Self> {
        qjsvalueiterator::qjsvalueiterator_new(value)
    }

    pub fn value(&self) -> UniquePtr<QJSValue> {
        qjsvalueiterator::qjsvalueiterator_value(self)
    }

    pub fn has_next(&self) -> bool {
        qjsvalueiterator::qjsvalueiterator_has_next(self)
    }

    pub fn next(self: Pin<&mut Self>) -> bool {
        qjsvalueiterator::qjsvalueiterator_next(self)
    }

    pub fn name(&self) -> QString {
        qjsvalueiterator::qjsvalueiterator_name(self)
    }
}
