// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use core::{marker::PhantomData, mem::MaybeUninit};
use cxx::{type_id, ExternType};

/// The QMap class is a template class that provides an associative array.
///
/// To use QMap with a custom pair, implement the [`QMapPair`] trait for T.
#[repr(C)]
pub struct QMap<T>
where
    T: QMapPair,
{
    _space: MaybeUninit<usize>,
    _value: PhantomData<T>,
}

impl<T> Clone for QMap<T>
where
    T: QMapPair,
{
    /// Constructs a copy of other.
    fn clone(&self) -> Self {
        T::clone(self)
    }
}

impl<T> Default for QMap<T>
where
    T: QMapPair,
{
    /// Constructs an empty map.
    fn default() -> Self {
        T::default()
    }
}

impl<T> Drop for QMap<T>
where
    T: QMapPair,
{
    /// Destroys the map.
    fn drop(&mut self) {
        T::drop(self)
    }
}

impl<T> PartialEq for QMap<T>
where
    T: QMapPair,
    T::Value: PartialEq,
{
    /// Returns true if both maps contain the same key value pairs
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().all(|(k, v)| other.get(k).as_ref() == Some(v))
    }
}

impl<T> Eq for QMap<T>
where
    T: QMapPair,
    T::Value: Eq,
{
}

impl<T> QMap<T>
where
    T: QMapPair,
{
    /// Removes all items from the map.
    pub fn clear(&mut self) {
        T::clear(self)
    }

    /// Returns true if the map contains an item with the key; otherwise returns false.
    pub fn contains(&self, key: &T::Key) -> bool {
        T::contains(self, key)
    }

    /// Returns the value associated with the key if it exists.
    pub fn get(&self, key: &T::Key) -> Option<T::Value> {
        if self.contains(key) {
            Some(T::get_or_default(self, key))
        } else {
            None
        }
    }

    /// Returns the value associated with the key or a default value.
    pub fn get_or_default(&self, key: &T::Key) -> T::Value {
        T::get_or_default(self, key)
    }

    /// Inserts a new item with the key and a value of value.
    ///
    /// The key and value are references here so they can be opaque or trivial.
    /// Note that the key and value are cloned before inserting into the map.
    pub fn insert_clone(&mut self, key: &T::Key, value: &T::Value) {
        T::insert_clone(self, key, value)
    }

    /// Returns true if the map contains no items; otherwise returns false.
    pub fn is_empty(&self) -> bool {
        T::len(self) == 0
    }

    /// An iterator visiting all key-value pairs in an arbitrary order.
    /// The iterator element type is (&T::Key, &T::Value).
    pub fn iter(&self) -> Iter<T> {
        Iter {
            map: self,
            index: 0,
        }
    }

    /// Returns the number of items in the map.
    pub fn len(&self) -> isize {
        T::len(self)
    }

    /// Removes all the items that have the key from the map.
    pub fn remove(&mut self, key: &T::Key) -> bool {
        T::remove(self, key)
    }
}

impl<T> QMap<T>
where
    T: QMapPair,
    T::Key: ExternType<Kind = cxx::kind::Trivial>,
    T::Value: ExternType<Kind = cxx::kind::Trivial>,
{
    /// Inserts a new item with the key and a value of value.
    pub fn insert(&mut self, key: T::Key, value: T::Value) {
        T::insert(self, key, value)
    }
}

unsafe impl<T> ExternType for QMap<T>
where
    T: QMapPair,
{
    type Id = T::TypeId;
    type Kind = cxx::kind::Trivial;
}

pub struct Iter<'a, T>
where
    T: QMapPair,
{
    map: &'a QMap<T>,
    index: isize,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: QMapPair,
{
    type Item = (&'a T::Key, &'a T::Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.map.len() {
            let next = unsafe {
                (
                    T::get_unchecked_key(self.map, self.index),
                    T::get_unchecked_value(self.map, self.index),
                )
            };
            self.index += 1;
            Some(next)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T>
where
    T: QMapPair,
{
    fn len(&self) -> usize {
        (self.map.len() - self.index) as usize
    }
}

/// Trait implementation for a pair in a [`QMap`].
pub trait QMapPair: Sized {
    type Key;
    type Value;
    type TypeId;

    fn clear(map: &mut QMap<Self>);
    fn clone(map: &QMap<Self>) -> QMap<Self>;
    fn contains(map: &QMap<Self>, key: &Self::Key) -> bool;
    fn default() -> QMap<Self>;
    fn drop(map: &mut QMap<Self>);
    fn get_or_default(map: &QMap<Self>, key: &Self::Key) -> Self::Value;
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked_key(map: &QMap<Self>, pos: isize) -> &Self::Key;
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked_value(map: &QMap<Self>, pos: isize) -> &Self::Value;
    fn insert(map: &mut QMap<Self>, key: Self::Key, value: Self::Value)
    where
        Self::Key: ExternType<Kind = cxx::kind::Trivial>,
        Self::Value: ExternType<Kind = cxx::kind::Trivial>;
    fn insert_clone(map: &mut QMap<Self>, key: &Self::Key, value: &Self::Value);
    fn len(map: &QMap<Self>) -> isize;
    fn remove(map: &mut QMap<Self>, key: &Self::Key) -> bool;
}

macro_rules! impl_qmap_pair {
    ( $keyTypeName:ty, $valueTypeName:ty, $module:ident, $pairTypeName:ident, $typeId:literal ) => {
        mod $module;
        pub use $module::$pairTypeName;

        impl QMapPair for $module::$pairTypeName {
            type Key = $keyTypeName;
            type Value = $valueTypeName;
            type TypeId = type_id!($typeId);

            fn clear(map: &mut QMap<Self>) {
                map.cxx_clear();
            }

            fn clone(map: &QMap<Self>) -> QMap<Self> {
                $module::clone(map)
            }

            fn contains(map: &QMap<Self>, key: &$keyTypeName) -> bool {
                map.cxx_contains(key)
            }

            fn default() -> QMap<Self> {
                $module::default()
            }

            fn drop(map: &mut QMap<Self>) {
                $module::drop(map);
            }

            fn get_or_default(map: &QMap<Self>, key: &$keyTypeName) -> $valueTypeName {
                $module::get_or_default(map, key)
            }

            unsafe fn get_unchecked_key(map: &QMap<Self>, pos: isize) -> &$keyTypeName {
                $module::get_unchecked_key(map, pos)
            }

            unsafe fn get_unchecked_value(map: &QMap<Self>, pos: isize) -> &$valueTypeName {
                $module::get_unchecked_value(map, pos)
            }

            fn insert(map: &mut QMap<Self>, key: $keyTypeName, value: $valueTypeName) {
                $module::insert(map, &key, &value);
            }

            fn insert_clone(map: &mut QMap<Self>, key: &$keyTypeName, value: &$valueTypeName) {
                $module::insert(map, key, value);
            }

            fn len(map: &QMap<Self>) -> isize {
                $module::len(map)
            }

            fn remove(map: &mut QMap<Self>, key: &$keyTypeName) -> bool {
                $module::remove(map, key)
            }
        }
    };
}

// For now we will implement useful combinations for Qt
// Other combinations the developer will have to implement themselves
// or a generator could be made later https://github.com/KDAB/cxx-qt/issues/355
//
// QVariantMap
impl_qmap_pair!(
    crate::QString,
    crate::QVariant,
    qmap_qstring_qvariant,
    QMapPair_QString_QVariant,
    "QMap_QString_QVariant"
);
