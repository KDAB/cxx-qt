use core::pin::Pin;

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Serialize};

use cxx_qt_lib::QString;

use crate::qml::QJSValue;
#[cfg(feature = "serde")]
use crate::qml::{JSEngineDeserializer, JSEngineSerializer};

#[cxx_qt::bridge]
mod qjsengine {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib-extras/qjsengine.h");
        include!("cxx-qt-lib-extras/qjsvalue.h");
        type QJSValue = crate::qml::QJSValue;
    }

    unsafe extern "C++Qt" {
        #[qobject]
        type QJSEngine;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qjsengine_new"]
        fn qjsengineNew() -> UniquePtr<QJSEngine>;

        #[doc(hidden)]
        #[rust_name = "qjsengine_new_array"]
        fn jsengineNewArray(engine: Pin<&mut QJSEngine>, length: u32) -> UniquePtr<QJSValue>;

        #[doc(hidden)]
        #[rust_name = "qjsengine_new_object"]
        fn jsengineNewObject(engine: Pin<&mut QJSEngine>) -> UniquePtr<QJSValue>;

        #[doc(hidden)]
        #[rust_name = "qjsengine_evaluate"]
        fn jsengineEvaluate(
            engine: Pin<&mut QJSEngine>,
            src: &QString,
            filename: &QString,
            line: i32,
        ) -> UniquePtr<QJSValue>;

        #[doc(hidden)]
        #[rust_name = "qjsengine_import_module"]
        fn jsengineImportModule(engine: Pin<&mut QJSEngine>, name: &QString)
            -> UniquePtr<QJSValue>;

        #[doc(hidden)]
        #[rust_name = "qjsengine_global_object"]
        fn jsengineGlobalObject(engine: Pin<&mut QJSEngine>) -> UniquePtr<QJSValue>;
    }
}

pub use qjsengine::QJSEngine;

impl QJSEngine {
    pub fn new() -> cxx::UniquePtr<Self> {
        qjsengine::qjsengine_new()
    }

    pub fn new_array(self: Pin<&mut Self>, length: u32) -> cxx::UniquePtr<QJSValue> {
        qjsengine::qjsengine_new_array(self, length)
    }

    pub fn new_object(self: Pin<&mut Self>) -> cxx::UniquePtr<QJSValue> {
        qjsengine::qjsengine_new_object(self)
    }

    pub fn evaluate(
        self: Pin<&mut Self>,
        src: &QString,
        filename: &QString,
        line: i32,
    ) -> cxx::UniquePtr<QJSValue> {
        qjsengine::qjsengine_evaluate(self, src, filename, line)
    }

    pub fn import_module(self: Pin<&mut Self>, name: &QString) -> cxx::UniquePtr<QJSValue> {
        qjsengine::qjsengine_import_module(self, name)
    }

    pub fn global_object(self: Pin<&mut Self>) -> cxx::UniquePtr<QJSValue> {
        qjsengine::qjsengine_global_object(self)
    }

    #[cfg(feature = "serde")]
    pub fn serialize<T: Serialize>(
        self: Pin<&mut Self>,
        value: &T,
    ) -> Result<cxx::UniquePtr<QJSValue>, serde_json::Error> {
        value.serialize(JSEngineSerializer::new(self))
    }

    #[cfg(feature = "serde")]
    pub fn deserialize<T: DeserializeOwned>(
        self: Pin<&mut Self>,
        value: &QJSValue,
    ) -> Result<T, serde_json::Error> {
        let de = JSEngineDeserializer::new(value);
        T::deserialize(de)
    }
}
