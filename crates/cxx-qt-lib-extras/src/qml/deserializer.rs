use core::pin::Pin;

use serde::{
    de::{
        self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, Unexpected, VariantAccess, Visitor,
    },
    Deserializer,
};

use cxx_qt_lib::QString;

use crate::qml::{QJSEngine, QJSValue, QJSValueIterator};

pub struct JSEngineDeserializer<'a> {
    value: &'a QJSValue,
}

impl<'a> JSEngineDeserializer<'a> {
    pub fn new(value: &'a QJSValue) -> Self {
        Self { value }
    }
}

impl<'de, 'a> de::Deserializer<'de> for JSEngineDeserializer<'a> {
    type Error = serde_json::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_bool() {
            visitor.visit_bool(self.value.to_bool())
        } else if self.value.is_number() {
            visitor.visit_f64(self.value.to_f64())
        } else if self.value.is_string() {
            let s = self.value.to_qstring();
            visitor.visit_string(s.to_string())
        } else if self.value.is_array() {
            self.deserialize_seq(visitor)
        } else if self.value.is_object() {
            self.deserialize_map(visitor)
        } else if self.value.is_null() {
            visitor.visit_unit()
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("unsupported type"),
                &visitor,
            ))
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_bool() {
            visitor.visit_bool(self.value.to_bool())
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a bool"),
                &visitor,
            ))
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_number() {
            visitor.visit_i8(self.value.to_int() as i8)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not an i8"),
                &visitor,
            ))
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_number() {
            visitor.visit_i16(self.value.to_int() as i16)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not an i16"),
                &visitor,
            ))
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_number() {
            visitor.visit_i32(self.value.to_int())
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not an i32"),
                &visitor,
            ))
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_number() {
            visitor.visit_i64(self.value.to_int() as i64)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not an i64"),
                &visitor,
            ))
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_number() {
            visitor.visit_u8(self.value.to_uint() as u8)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a u8"),
                &visitor,
            ))
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_number() {
            visitor.visit_u16(self.value.to_uint() as u16)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a u16"),
                &visitor,
            ))
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_number() {
            visitor.visit_u32(self.value.to_uint())
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a u32"),
                &visitor,
            ))
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_number() {
            visitor.visit_u64(self.value.to_uint() as u64)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a u64"),
                &visitor,
            ))
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_number() {
            visitor.visit_f32(self.value.to_f64() as f32)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a f32"),
                &visitor,
            ))
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_number() {
            visitor.visit_f64(self.value.to_f64())
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a f64"),
                &visitor,
            ))
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_string() {
            let s = self.value.to_qstring();
            let s = s.to_string();
            let mut chars = s.chars();
            if let Some(c) = chars.next() {
                if chars.next().is_none() {
                    return visitor.visit_char(c);
                }
            }
            Err(de::Error::invalid_type(
                Unexpected::Str(&s.to_string()),
                &visitor,
            ))
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a char"),
                &visitor,
            ))
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_string() {
            visitor.visit_str(&self.value.to_qstring().to_string())
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a string"),
                &visitor,
            ))
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_string() {
            visitor.visit_string(self.value.to_qstring().to_string())
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a string"),
                &visitor,
            ))
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_array() {
            let len = self.value.get_property(&QString::from("length")).to_uint() as usize;
            let mut vec = Vec::with_capacity(len);
            for i in 0..len {
                let elem = self.value.get_element(i as u32);
                if elem.is_number() {
                    vec.push(elem.to_uint() as u8);
                } else {
                    return Err(de::Error::invalid_type(
                        Unexpected::Other("not a byte array"),
                        &visitor,
                    ));
                }
            }
            visitor.visit_bytes(&vec)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not bytes"),
                &visitor,
            ))
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_array() {
            let len = self.value.get_property(&QString::from("length")).to_uint() as usize;
            let mut vec = Vec::with_capacity(len);
            for i in 0..len {
                let elem = self.value.get_element(i as u32);
                if elem.is_number() {
                    vec.push(elem.to_uint() as u8);
                } else {
                    return Err(de::Error::invalid_type(
                        Unexpected::Other("not a byte array"),
                        &visitor,
                    ));
                }
            }
            visitor.visit_byte_buf(vec)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not bytes"),
                &visitor,
            ))
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_null() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_null() {
            visitor.visit_unit()
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a unit"),
                &visitor,
            ))
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_array() {
            let len = self.value.get_property(&QString::from("length")).to_uint() as usize;
            let seq = QJSSeqAccess {
                array: self.value,
                index: 0,
                len,
            };
            visitor.visit_seq(seq)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a sequence"),
                &visitor,
            ))
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_object() {
            let keys = QJSValueIterator::new(self.value);
            let map = QJSMapAccess {
                object: self.value,
                keys,
            };
            visitor.visit_map(map)
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not a map"),
                &visitor,
            ))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.value.is_string() {
            visitor.visit_enum(QJSVariantAccess {
                variant: self.value.to_qstring().to_string(),
                value: self.value.clone(),
            })
        } else if self.value.is_object() {
            let mut keys = QJSValueIterator::new(self.value);
            if keys.has_next() {
                keys.as_mut().unwrap().next();
                let key = keys.name();
                let value = self.value.get_property(&key);
                visitor.visit_enum(QJSVariantAccess {
                    variant: key.to_string(),
                    value,
                })
            } else {
                Err(de::Error::invalid_type(
                    Unexpected::Other("not a single key object"),
                    &visitor,
                ))
            }
        } else {
            Err(de::Error::invalid_type(
                Unexpected::Other("not an enum"),
                &visitor,
            ))
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct QJSSeqAccess<'a> {
    array: &'a QJSValue,
    index: usize,
    len: usize,
}

impl<'de, 'a> SeqAccess<'de> for QJSSeqAccess<'a> {
    type Error = serde_json::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.index >= self.len {
            return Ok(None);
        }
        let elem = self.array.get_element(self.index as u32);
        self.index += 1;
        let de = JSEngineDeserializer::new(&elem);
        seed.deserialize(de).map(Some)
    }
}

struct QJSMapAccess<'a> {
    object: &'a QJSValue,
    keys: cxx::UniquePtr<QJSValueIterator>,
}

impl<'de, 'a> MapAccess<'de> for QJSMapAccess<'a> {
    type Error = serde_json::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if !self.keys.has_next() {
            return Ok(None);
        }
        self.keys.as_mut().unwrap().next();
        let key = self.keys.name();
        let key_str = key.to_string();
        let value = QJSValue::from_str(&key_str);
        let de = JSEngineDeserializer::new(&value);
        seed.deserialize(de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let key = self.keys.name();
        let value = self.object.get_property(&key);
        let de = JSEngineDeserializer::new(&value);
        seed.deserialize(de)
    }
}

struct QJSVariantAccess {
    variant: String,
    value: cxx::UniquePtr<QJSValue>,
}

impl<'de> EnumAccess<'de> for QJSVariantAccess {
    type Error = serde_json::Error;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let binding = QJSValue::from_str(&self.variant);
        let de = JSEngineDeserializer::new(&binding);
        seed.deserialize(de).map(|v| (v, self))
    }
}

impl<'de> VariantAccess<'de> for QJSVariantAccess {
    type Error = serde_json::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        let de = JSEngineDeserializer::new(&self.value);
        seed.deserialize(de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let de = JSEngineDeserializer::new(&self.value);
        de.deserialize_seq(visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let de = JSEngineDeserializer::new(&self.value);
        de.deserialize_map(visitor)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use cxx::UniquePtr;
    use cxx_qt_lib::{QCoreApplication, QString};
    use serde::{Deserialize, Serialize};

    use crate::qml::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct SerializeTest {
        key: String,
        value: i32,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct DeserializeTest {
        name: String,
        age: i32,
        list: Vec<String>,
        map: HashMap<String, SerializeTest>,
    }

    /// Setup function to create a QJSEngine and evaluate a script
    fn setup_and_evaluate(
        value_str: &str,
    ) -> (
        UniquePtr<QJSEngine>,
        UniquePtr<QJSValue>,
        UniquePtr<QCoreApplication>,
    ) {
        let app = QCoreApplication::new();
        let mut engine_ptr = QJSEngine::new();
        let mut engine = engine_ptr.as_mut().unwrap();
        let value =
            engine
                .as_mut()
                .evaluate(&QString::from(value_str), &QString::from("filename"), 1);
        (engine_ptr, value, app)
    }

    #[test]
    fn test_deserialize_bool() {
        let (mut engine, value, _) = setup_and_evaluate("true");
        let deserialized: bool = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, true);
    }

    #[test]
    fn test_deserialize_string() {
        let (mut engine, value, _) = setup_and_evaluate("'Hello, world!'");
        let deserialized: String = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, "Hello, world!");
    }

    #[test]
    fn test_deserialize_integer() {
        let (mut engine, value, _) = setup_and_evaluate("42");
        let deserialized: i32 = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, 42);
    }

    #[test]
    fn test_deserialize_float() {
        let (mut engine, value, _) = setup_and_evaluate("3.14159");
        let deserialized: f64 = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, 3.14159);
    }

    #[test]
    fn test_deserialize_vector() {
        let (mut engine, value, _) = setup_and_evaluate("['apple', 'banana', 'cherry']");
        let deserialized: Vec<String> = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_deserialize_map() {
        let (mut engine, value, _) = setup_and_evaluate(
            "JSON.parse('{\"mapkey\": { \"key\": \"example\", \"value\": 123 } }')",
        );
        let deserialized: HashMap<String, SerializeTest> =
            engine.as_mut().unwrap().deserialize(&value).unwrap();
        let inner = SerializeTest {
            key: "example".to_string(),
            value: 123,
        };
        let mut map = HashMap::new();
        map.insert("mapkey".to_string(), inner);
        assert_eq!(deserialized, map);
    }

    #[test]
    fn test_deserialize_custom_type() {
        let (mut engine, value, _) = setup_and_evaluate(
            "JSON.parse('{\"name\": \"Alice\", \"age\": 30, \"list\": [\"music\", \"books\"], \"map\": { \"key1\": { \"key\": \"item\", \"value\": 10 } } }')"
        );
        let deserialized: DeserializeTest = engine.as_mut().unwrap().deserialize(&value).unwrap();
        let mut map = HashMap::new();
        map.insert(
            "key1".to_string(),
            SerializeTest {
                key: "item".to_string(),
                value: 10,
            },
        );

        let expected = DeserializeTest {
            name: "Alice".to_string(),
            age: 30,
            list: vec!["music".to_string(), "books".to_string()],
            map: map,
        };
        assert_eq!(deserialized, expected);
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct TupleStruct(i32, String, f64);

    #[test]
    fn test_deserialize_tuple() {
        let (mut engine, value, _) = setup_and_evaluate("JSON.parse('[1, \"two\", 3.14]')");
        let deserialized: (i32, String, f64) =
            engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, (1, "two".to_string(), 3.14));
    }

    #[test]
    fn test_deserialize_tuple_struct() {
        let (mut engine, value, _) = setup_and_evaluate("JSON.parse('[42, \"Hello\", 2.718]')");
        let deserialized: TupleStruct = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, TupleStruct(42, "Hello".to_string(), 2.718));
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum MyEnum {
        Unit,
        NewType(i32),
        Tuple(i32, String),
        Struct { id: u32, name: String },
    }

    #[test]
    fn test_deserialize_enum_unit() {
        let (mut engine, value, _) = setup_and_evaluate("JSON.parse('{\"Unit\": null}')");
        let deserialized: MyEnum = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, MyEnum::Unit);
    }
    #[test]
    fn test_deserialize_enum_newtype() {
        let (mut engine, value, _) = setup_and_evaluate("JSON.parse('{\"NewType\": 42}')");
        let deserialized: MyEnum = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, MyEnum::NewType(42));
    }

    #[test]
    fn test_deserialize_enum_tuple() {
        let (mut engine, value, _) =
            setup_and_evaluate("JSON.parse('{\"Tuple\": [123, \"test\"]}')");
        let deserialized: MyEnum = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, MyEnum::Tuple(123, "test".to_string()));
    }

    #[test]
    fn test_deserialize_enum_struct() {
        let (mut engine, value, _) =
            setup_and_evaluate("JSON.parse('{\"Struct\": {\"id\": 1, \"name\": \"Alice\"}}')");
        let deserialized: MyEnum = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(
            deserialized,
            MyEnum::Struct {
                id: 1,
                name: "Alice".to_string()
            }
        );
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct UnitStruct;

    #[test]
    fn test_deserialize_unit() {
        let (mut engine, value, _) = setup_and_evaluate("null");
        let deserialized: () = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, ());
    }

    #[test]
    fn test_deserialize_unit_struct() {
        let (mut engine, value, _) = setup_and_evaluate("null");
        let deserialized: UnitStruct = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized, UnitStruct);
    }

    #[test]
    fn test_deserialize_invalid_json() {
        let (mut engine, value, _) = setup_and_evaluate("{ this is : 'invalid' }");
        let result: Result<HashMap<String, i32>, _> = engine.as_mut().unwrap().deserialize(&value);
        assert!(result.is_err(), "Expected an error for invalid JSON input");
    }

    #[test]
    fn test_deserialize_excess_data() {
        let (mut engine, value, _) = setup_and_evaluate(
            "JSON.parse('{\"name\": \"Bob\", \"age\": 25, \"extra\": \"ignored\", \"list\": [], \"map\": {}}')",
        );
        let deserialized: DeserializeTest = engine.as_mut().unwrap().deserialize(&value).unwrap();
        assert_eq!(deserialized.name, "Bob");
        assert_eq!(deserialized.age, 25);
        // The test ensures that 'extra' field does not cause a failure
    }
}
