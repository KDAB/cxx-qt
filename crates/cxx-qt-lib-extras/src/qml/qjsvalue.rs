use core::pin::Pin;

use cxx::UniquePtr;
#[cfg(feature = "serde")]
use serde::de::DeserializeOwned;

use cxx_qt_lib::{QVariant, QVariantValue, QString};

#[cfg(feature = "serde")]
use crate::qml::QJSEngine;
use crate::qml::JSEngineDeserializer;

#[cxx_qt::bridge]
mod qjsvalue {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib-extras/qjsvalue.h");
        type QJSValue;

    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        fn qjsvalue_new() -> UniquePtr<QJSValue>;
        fn qjsvalue_new_null() -> UniquePtr<QJSValue>;
        fn qjsvalue_new_bool(val: bool) -> UniquePtr<QJSValue>;
        fn qjsvalue_new_int(val: i32) -> UniquePtr<QJSValue>;
        fn qjsvalue_new_uint(val: u32) -> UniquePtr<QJSValue>;
        fn qjsvalue_new_double(val: f64) -> UniquePtr<QJSValue>;
        fn qjsvalue_new_qstring(val: &QString) -> UniquePtr<QJSValue>;

        fn qjsvalue_from_jsvalue(jsvalue: &QJSValue) -> UniquePtr<QJSValue>;

        fn qjsvalue_to_string(value: &QJSValue) -> QString;

        fn qjsvalue_property(value: &QJSValue, name: &QString) -> UniquePtr<QJSValue>;
        fn qjsvalue_element(value: &QJSValue, index: u32) -> UniquePtr<QJSValue>;

        fn qjsvalue_to_qvariant(value: &QJSValue) -> QVariant;

        #[rust_name = "can_convert_qjsvalue"]
        fn qvariantCanConvertQJSValue(variant: &QVariant) -> bool;
        #[rust_name = "qjsvalue_from_qvariant"]
        fn qjsvalueFromQVariant(variant: &QVariant) -> UniquePtr<QJSValue>;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {

        #[rust_name = "is_bool"]
        fn isBool(self: &QJSValue) -> bool;
        #[rust_name = "is_array"]
        fn isArray(self: &QJSValue) -> bool;
        #[rust_name = "is_callable"]
        fn isCallable(self: &QJSValue) -> bool;
        #[rust_name = "is_date"]
        fn isDate(self: &QJSValue) -> bool;
        #[rust_name = "is_error"]
        fn isError(self: &QJSValue) -> bool;
        #[rust_name = "is_null"]
        fn isNull(self: &QJSValue) -> bool;
        #[rust_name = "is_number"]
        fn isNumber(self: &QJSValue) -> bool;
        #[rust_name = "is_object"]
        fn isObject(self: &QJSValue) -> bool;
        #[rust_name = "is_qmetaobject"]
        fn isQMetaObject(self: &QJSValue) -> bool;
        #[rust_name = "is_qobject"]
        fn isQObject(self: &QJSValue) -> bool;
        #[rust_name = "is_regexp"]
        fn isRegExp(self: &QJSValue) -> bool;
        #[rust_name = "is_string"]
        fn isString(self: &QJSValue) -> bool;
        #[rust_name = "is_undefined"]
        fn isUndefined(self: &QJSValue) -> bool;
        #[rust_name = "is_url"]
        fn isUrl(self: &QJSValue) -> bool;
        #[rust_name = "is_variant"]
        fn isVariant(self: &QJSValue) -> bool;

        #[rust_name = "has_property"]
        fn hasProperty(self: &QJSValue, name: &QString) -> bool;
        #[rust_name = "set_property"]
        fn setProperty(self: Pin<&mut QJSValue>, name: &QString, value: &QJSValue);
        #[rust_name = "set_element"]
        #[cxx_name = "setProperty"]
        fn setElement(self: Pin<&mut QJSValue>, index: u32, value: &QJSValue);

        #[rust_name = "to_bool"]
        fn toBool(self: &QJSValue) -> bool;
        #[rust_name = "to_uint"]
        fn toUInt(self: &QJSValue) -> u32;
        #[rust_name = "to_int"]
        fn toInt(self: &QJSValue) -> i32;
        #[rust_name = "to_f64"]
        fn toNumber(self: &QJSValue) -> f64;

    }
}

pub use qjsvalue::QJSValue;

impl QJSValue {
    pub fn undefined() -> UniquePtr<Self> {
        qjsvalue::qjsvalue_new()
    }

    pub fn null() -> UniquePtr<Self> {
        qjsvalue::qjsvalue_new_null()
    }

    pub fn from_bool(val: bool) -> UniquePtr<Self> {
        qjsvalue::qjsvalue_new_bool(val)
    }

    pub fn from_int(val: i32) -> UniquePtr<Self> {
        qjsvalue::qjsvalue_new_int(val)
    }

    pub fn from_uint(val: u32) -> UniquePtr<Self> {
        qjsvalue::qjsvalue_new_uint(val)
    }

    pub fn from_f64(val: f64) -> UniquePtr<Self> {
        qjsvalue::qjsvalue_new_double(val)
    }

    pub fn from_qstring(val: &QString) -> UniquePtr<Self> {
        qjsvalue::qjsvalue_new_qstring(val)
    }

    pub fn from_str(val: &str) -> UniquePtr<Self> {
        let qstring = QString::from(val);
        qjsvalue::qjsvalue_new_qstring(&qstring)
    }

    pub fn from_jsvalue(jsvalue: &QJSValue) -> UniquePtr<Self> {
        qjsvalue::qjsvalue_from_jsvalue(jsvalue)
    }

    pub fn from_map(
        engine: Pin<&mut QJSEngine>,
        map: &std::collections::HashMap<String, UniquePtr<QJSValue>>,
    ) -> UniquePtr<Self> {
        let mut obj = engine.new_object();
        let mut ptr = obj.as_mut().unwrap();
        for (key, value) in map.iter() {
            ptr.as_mut().set_property(&QString::from(key), &value);
        }
        obj
    }

    pub fn from_array(
        engine: Pin<&mut QJSEngine>,
        array: &Vec<UniquePtr<QJSValue>>,
    ) -> UniquePtr<Self> {
        let mut obj = engine.new_array(array.len() as u32);
        let mut ptr = obj.as_mut().unwrap();
        for (index, value) in array.iter().enumerate() {
            ptr.as_mut().set_element(index as u32, &value);
        }
        obj
    }

    pub fn to_qstring(&self) -> QString {
        qjsvalue::qjsvalue_to_string(self)
    }

    pub fn get_property(&self, name: &QString) -> UniquePtr<QJSValue> {
        qjsvalue::qjsvalue_property(self, name)
    }

    pub fn get_element(&self, index: u32) -> UniquePtr<QJSValue> {
        qjsvalue::qjsvalue_element(self, index)
    }

    pub fn clone(&self) -> UniquePtr<Self> {
        qjsvalue::qjsvalue_from_jsvalue(self)
    }

    pub fn to_qvariant(&self) -> QVariant {
        qjsvalue::qjsvalue_to_qvariant(self)
    }

    #[cfg(feature = "serde")]
    pub fn deserialize<T: DeserializeOwned>(self: &Self) -> Result<T, serde_json::Error> {
        let de = JSEngineDeserializer::new(self);
        T::deserialize(de)
    }

    pub fn from_qvariant(variant: &QVariant) -> Option<UniquePtr<Self>> {
        if qjsvalue::can_convert_qjsvalue(variant) {
            Some(qjsvalue::qjsvalue_from_qvariant(variant))
        } else {
            None
        }
    }
}

impl std::fmt::Display for QJSValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_qstring().to_string())
    }
}
