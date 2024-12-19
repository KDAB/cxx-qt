use core::pin::Pin;
use std::collections::HashMap;

use cxx::UniquePtr;
#[cfg(feature = "serde")]
use serde::ser::*;

use cxx_qt_lib::QString;

use crate::qml::{QJSEngine, QJSValue};

pub struct JSEngineSerializer<'a> {
    engine: Pin<&'a mut QJSEngine>,
}

impl<'a> JSEngineSerializer<'a> {
    pub fn new(engine: Pin<&'a mut QJSEngine>) -> Self {
        Self { engine }
    }
}

impl<'a> Serializer for JSEngineSerializer<'a> {
    type Ok = UniquePtr<QJSValue>;
    type Error = serde_json::Error;

    type SerializeSeq = QJSSerializeSeq<'a>;
    type SerializeTuple = QJSSerializeSeq<'a>;
    type SerializeTupleStruct = QJSSerializeSeq<'a>;
    type SerializeTupleVariant = QJSSerializeTupleVariant<'a>;
    type SerializeMap = QJSSerializeMap<'a>;
    type SerializeStruct = QJSSerializeMap<'a>;
    type SerializeStructVariant = QJSSerializeStructVariant<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_int(v as i32))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_int(v as i32))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_int(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_int(v as i32)) // Assuming 32-bit int for simplicity
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_uint(v as u32))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_uint(v as u32))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_uint(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_f64(v as f64)) // Assuming 32-bit int for simplicity
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_f64(v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_f64(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let s: String = v.into();
        self.serialize_str(&s)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::from_str(v))
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let vec: Vec<_> = v.iter().map(|&b| QJSValue::from_uint(b as u32)).collect();
        Ok(QJSValue::from_array(self.engine.as_mut(), &vec))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::null())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(QJSValue::null())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        mut self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut map: HashMap<String, UniquePtr<QJSValue>> = HashMap::new();

        let serializer = JSEngineSerializer::new(self.engine.as_mut());
        map.insert(variant.to_string(), value.serialize(serializer)?);
        Ok(QJSValue::from_map(self.engine.as_mut(), &map))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let JSEngineSerializer { mut engine } = self;
        let array = engine.as_mut().new_array(len.unwrap_or(0) as u32);
        Ok(QJSSerializeSeq {
            array,
            index: 0,
            engine: engine,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let JSEngineSerializer { mut engine } = self;
        let array = engine.as_mut().new_array(len as u32);
        Ok(QJSSerializeTupleVariant {
            name: variant.to_string(),
            array,
            index: 0,
            engine: engine,
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let JSEngineSerializer { mut engine } = self;
        let object = engine.as_mut().new_object();
        Ok(QJSSerializeMap {
            object,
            key: None,
            engine: engine,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let JSEngineSerializer { mut engine } = self;
        let object = engine.as_mut().new_object();
        Ok(QJSSerializeStructVariant {
            name: variant.to_string(),
            object,
            engine: engine,
        })
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: std::fmt::Display,
    {
        let s = value.to_string();
        self.serialize_str(&s)
    }
}

pub struct QJSSerializeSeq<'a> {
    array: UniquePtr<QJSValue>,
    index: usize,
    engine: Pin<&'a mut QJSEngine>,
}

impl<'a> SerializeSeq for QJSSerializeSeq<'a> {
    type Ok = UniquePtr<QJSValue>;
    type Error = serde_json::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = {
            let serializer = JSEngineSerializer::new(self.engine.as_mut());
            value.serialize(serializer)?
        };
        self.array
            .as_mut()
            .unwrap()
            .set_element(self.index as u32, &value);
        self.index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array)
    }
}

impl<'a> SerializeTuple for QJSSerializeSeq<'a> {
    type Ok = UniquePtr<QJSValue>;
    type Error = serde_json::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = {
            let serializer = JSEngineSerializer::new(self.engine.as_mut());
            value.serialize(serializer)?
        };
        self.array
            .as_mut()
            .unwrap()
            .set_element(self.index as u32, &value);
        self.index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array)
    }
}

impl<'a> SerializeTupleStruct for QJSSerializeSeq<'a> {
    type Ok = UniquePtr<QJSValue>;
    type Error = serde_json::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = {
            let serializer = JSEngineSerializer::new(self.engine.as_mut());
            value.serialize(serializer)?
        };
        self.array
            .as_mut()
            .unwrap()
            .set_element(self.index as u32, &value);
        self.index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array)
    }
}

pub struct QJSSerializeTupleVariant<'a> {
    name: String,
    array: UniquePtr<QJSValue>,
    index: usize,
    engine: Pin<&'a mut QJSEngine>,
}

impl<'a> SerializeTupleVariant for QJSSerializeTupleVariant<'a> {
    type Ok = UniquePtr<QJSValue>;
    type Error = serde_json::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = {
            let serializer = JSEngineSerializer::new(self.engine.as_mut());
            value.serialize(serializer)?
        };
        self.array
            .as_mut()
            .unwrap()
            .set_element(self.index as u32, &value);
        self.index += 1;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut object = self.engine.as_mut().new_object();
        object
            .as_mut()
            .unwrap()
            .set_property(&QString::from(&self.name), &self.array);
        Ok(object)
    }
}

pub struct QJSSerializeMap<'a> {
    object: UniquePtr<QJSValue>,
    key: Option<QString>,
    engine: Pin<&'a mut QJSEngine>,
}

impl<'a> SerializeMap for QJSSerializeMap<'a> {
    type Ok = UniquePtr<QJSValue>;
    type Error = serde_json::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let serializer = JSEngineSerializer::new(self.engine.as_mut());
        self.key = Some(key.serialize(serializer)?.to_qstring());
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if let Some(ref key) = self.key {
            let value = {
                let serializer = JSEngineSerializer::new(self.engine.as_mut());
                value.serialize(serializer)?
            };
            self.object.as_mut().unwrap().set_property(key, &value);
        }
        self.key = None;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.object)
    }
}

impl<'a> SerializeStruct for QJSSerializeMap<'a> {
    type Ok = UniquePtr<QJSValue>;
    type Error = serde_json::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = {
            let serializer = JSEngineSerializer::new(self.engine.as_mut());
            value.serialize(serializer)?
        };
        self.object
            .as_mut()
            .unwrap()
            .set_property(&QString::from(key), &value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.object)
    }
}

pub struct QJSSerializeStructVariant<'a> {
    name: String,
    object: UniquePtr<QJSValue>,
    engine: Pin<&'a mut QJSEngine>,
}

impl<'a> SerializeStructVariant for QJSSerializeStructVariant<'a> {
    type Ok = UniquePtr<QJSValue>;
    type Error = serde_json::Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = {
            let serializer = JSEngineSerializer::new(self.engine.as_mut());
            value.serialize(serializer)?
        };
        self.object
            .as_mut()
            .unwrap()
            .set_property(&QString::from(key), &value);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut variant = self.engine.as_mut().new_object();
        variant
            .as_mut()
            .unwrap()
            .set_property(&QString::from(&self.name), &self.object);
        Ok(variant)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::qml::*;
    use cxx::UniquePtr;
    use cxx_qt_lib::{QCoreApplication, QString};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct SerializeTest {
        key: String,
        value: i32,
    }

    fn setup_and_serialize<T: Serialize>(
        value: &T,
    ) -> (
        UniquePtr<QJSEngine>,
        UniquePtr<QJSValue>,
        UniquePtr<QCoreApplication>,
    ) {
        let app = QCoreApplication::new();
        let mut engine_ptr = QJSEngine::new();
        let mut engine = engine_ptr.as_mut().unwrap();
        let serialized_value = engine.as_mut().serialize(&value).unwrap();
        (engine_ptr, serialized_value, app)
    }

    #[test]
    fn test_serialize_bool() {
        let (mut engine, value, _app) = setup_and_serialize(&true);
        assert_eq!(engine.as_mut().unwrap().qjsvalue_to_json(&value), "true");
    }

    #[test]
    fn test_serialize_integer() {
        let (mut engine, value, _app) = setup_and_serialize(&42);
        assert_eq!(engine.as_mut().unwrap().qjsvalue_to_json(&value), "42");
    }

    #[test]
    fn test_serialize_string() {
        let (mut engine, value, _app) = setup_and_serialize(&"Hello, world!");
        assert_eq!(
            engine.as_mut().unwrap().qjsvalue_to_json(&value),
            "\"Hello, world!\""
        );
    }

    #[test]
    fn test_serialize_vector() {
        let (mut engine, value, _app) = setup_and_serialize(&vec!["apple", "banana", "cherry"]);
        assert_eq!(
            engine.as_mut().unwrap().qjsvalue_to_json(&value),
            "[\"apple\",\"banana\",\"cherry\"]"
        );
    }

    #[test]
    fn test_serialize_map() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        let (mut engine, value, _app) = setup_and_serialize(&map);
        assert_eq!(
            engine.as_mut().unwrap().qjsvalue_to_json(&value),
            "{\"key\":\"value\"}"
        );
    }

    #[test]
    fn test_serialize_custom_type() {
        let data = SerializeTest {
            key: "example".to_string(),
            value: 123,
        };
        let (mut engine, value, _app) = setup_and_serialize(&data);
        assert_eq!(
            engine.as_mut().unwrap().qjsvalue_to_json(&value),
            "{\"key\":\"example\",\"value\":123}"
        );
    }

    #[test]
    fn test_serialize_none() {
        let none: Option<i32> = None;
        let (mut engine, value, _app) = setup_and_serialize(&none);
        assert_eq!(engine.as_mut().unwrap().qjsvalue_to_json(&value), "null");
    }

    #[test]
    fn test_serialize_some() {
        let some = Some(42);
        let (mut engine, value, _app) = setup_and_serialize(&some);
        assert_eq!(engine.as_mut().unwrap().qjsvalue_to_json(&value), "42");
    }

    #[test]
    fn test_boolean_serialization() {
        let (_, value, _app) = setup_and_serialize(&true);
        assert!(value.is_bool());
        assert_eq!(value.to_bool(), true);
    }

    #[test]
    fn test_integer_serialization() {
        let (_, value, _app) = setup_and_serialize(&42);
        assert!(value.is_number());
        assert_eq!(value.to_int(), 42);
    }

    #[test]
    fn test_float_serialization() {
        let (_, value, _app) = setup_and_serialize(&3.14f64);
        assert!(value.is_number());
        assert_eq!(value.to_f64(), 3.14);
    }

    #[test]
    fn test_string_serialization() {
        let (_, value, _app) = setup_and_serialize(&"Hello, world!");
        assert!(value.is_string());
        assert_eq!(value.to_qstring().to_string(), "Hello, world!");
    }

    #[test]
    fn test_array_serialization() {
        let list = vec![1, 2, 3];
        let (mut engine, value, _app) = setup_and_serialize(&list);
        engine.as_mut().unwrap().qjsvalue_to_json(&value);
        assert!(value.is_array());
        assert_eq!(value.get_element(0).to_int(), 1);
        assert_eq!(value.get_element(1).to_int(), 2);
        assert_eq!(value.get_element(2).to_int(), 3);
    }

    #[test]
    fn test_object_serialization() {
        let mut map = HashMap::new();
        map.insert("key", 42);
        let (_engine, value, _app) = setup_and_serialize(&map);
        assert!(value.is_object());
        assert!(value.has_property(&QString::from("key")));
        assert_eq!(value.get_property(&QString::from("key")).to_int(), 42);
    }

    #[test]
    fn test_null_serialization() {
        let none: Option<i32> = None;
        let (mut engine, value, _app) = setup_and_serialize(&none);
        engine.as_mut().unwrap().qjsvalue_to_json(&value);
        assert!((*value).is_null());
    }

    #[test]
    fn test_undefined_serialization() {
        let value = QJSValue::undefined();
        assert!(value.is_undefined());
    }
}
