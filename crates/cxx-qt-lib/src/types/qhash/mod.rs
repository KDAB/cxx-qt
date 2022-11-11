// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use core::{marker::PhantomData, mem::MaybeUninit};
use cxx::{type_id, ExternType};

mod qhash_qstring_qvariant;
pub use qhash_qstring_qvariant::QHashPair_QString_QVariant;

/// The QHash class is a template class that provides a hash-table-based dictionary.
///
/// Note that this means that T needs to have a C++ global
/// [`qHash()` function](https://doc.qt.io/qt-6/qhash.html#qhash).
///
/// To use QHash with a custom pair, implement the [`QHashPair`] trait for T.
#[repr(C)]
pub struct QHash<T>
where
    T: QHashPair,
{
    _space: MaybeUninit<usize>,
    _value: PhantomData<T>,
}

impl<T> Clone for QHash<T>
where
    T: QHashPair,
{
    /// Constructs a copy of other.
    fn clone(&self) -> Self {
        T::clone(self)
    }
}

impl<T> Default for QHash<T>
where
    T: QHashPair,
{
    /// Constructs an empty hash.
    fn default() -> Self {
        T::default()
    }
}

impl<T> Drop for QHash<T>
where
    T: QHashPair,
{
    /// Destroys the hash.
    fn drop(&mut self) {
        T::drop(self)
    }
}

impl<T> QHash<T>
where
    T: QHashPair,
{
    /// Removes all items from the hash.
    pub fn clear(&mut self) {
        T::clear(self)
    }

    /// Returns true if the hash contains an item with the key; otherwise returns false.
    pub fn contains(&self, key: &T::Key) -> bool {
        T::contains(self, key)
    }

    /// Inserts a new item with the key and a value of value.
    ///
    /// The key and value is a reference here so it can be opaque or trivial but
    /// note that the key and value is copied when being inserted into the hash.
    pub fn insert_clone(&mut self, key: &T::Key, value: &T::Value) {
        T::insert_clone(self, key, value)
    }

    /// Returns true if the hash contains no items; otherwise returns false.
    pub fn is_empty(&self) -> bool {
        T::len(self) == 0
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is (&'a T::Key, &'a T::Value).
    pub fn iter(&self) -> Iter<T> {
        Iter {
            hash: self,
            index: 0,
        }
    }

    /// Returns the number of items in the hash.
    pub fn len(&self) -> usize {
        T::len(self)
    }

    /// Removes all the items that have the key from the hash.
    ///
    /// Returns true if at least one item was removed, otherwise returns false.
    pub fn remove(&mut self, key: &T::Key) -> bool {
        T::remove(self, key)
    }

    /// Returns the value associated with the key.
    pub fn value(&self, key: &T::Key) -> T::Value {
        T::value(self, key)
    }
}

impl<T> QHash<T>
where
    T: QHashPair,
    T::Key: ExternType<Kind = cxx::kind::Trivial>,
    T::Value: ExternType<Kind = cxx::kind::Trivial>,
{
    /// Inserts a new item with the key and a value of value.
    pub fn insert(&mut self, key: T::Key, value: T::Value) {
        T::insert(self, key, value)
    }
}

unsafe impl<T> ExternType for QHash<T>
where
    T: QHashPair,
{
    type Id = T::TypeId;
    type Kind = cxx::kind::Trivial;
}

pub struct Iter<'a, T>
where
    T: QHashPair,
{
    hash: &'a QHash<T>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: QHashPair,
{
    type Item = (&'a T::Key, &'a T::Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.hash.len() {
            let next = unsafe {
                (
                    T::get_unchecked_key(self.hash, self.index),
                    T::get_unchecked_value(self.hash, self.index),
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
    T: QHashPair,
{
    fn len(&self) -> usize {
        self.hash.len() - self.index
    }
}

/// Trait implementation for a pair in a [`QHash`].
pub trait QHashPair: Sized {
    type Key;
    type Value;
    type TypeId;

    fn clear(hash: &mut QHash<Self>);
    fn clone(hash: &QHash<Self>) -> QHash<Self>;
    fn contains(hash: &QHash<Self>, key: &Self::Key) -> bool;
    fn default() -> QHash<Self>;
    fn drop(hash: &mut QHash<Self>);
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked_key(hash: &QHash<Self>, pos: usize) -> &Self::Key;
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked_value(hash: &QHash<Self>, pos: usize) -> &Self::Value;
    fn insert(hash: &mut QHash<Self>, key: Self::Key, value: Self::Value)
    where
        Self::Key: ExternType<Kind = cxx::kind::Trivial>,
        Self::Value: ExternType<Kind = cxx::kind::Trivial>;
    fn insert_clone(hash: &mut QHash<Self>, key: &Self::Key, value: &Self::Value);
    fn len(hash: &QHash<Self>) -> usize;
    fn remove(hash: &mut QHash<Self>, key: &Self::Key) -> bool;
    fn value(hash: &QHash<Self>, key: &Self::Key) -> Self::Value;
}

macro_rules! impl_qhash_pair {
    ( $keyTypeName:ty, $valueTypeName:ty, $module:ident, $pairTypeName:ident, $typeId:literal ) => {
        impl QHashPair for $module::$pairTypeName {
            type Key = $keyTypeName;
            type Value = $valueTypeName;
            type TypeId = type_id!($typeId);

            fn clear(hash: &mut QHash<Self>) {
                hash.cxx_clear();
            }

            fn clone(hash: &QHash<Self>) -> QHash<Self> {
                $module::clone(hash)
            }

            fn contains(hash: &QHash<Self>, key: &$keyTypeName) -> bool {
                hash.cxx_contains(key)
            }

            fn default() -> QHash<Self> {
                $module::default()
            }

            fn drop(hash: &mut QHash<Self>) {
                $module::drop(hash);
            }

            unsafe fn get_unchecked_key(hash: &QHash<Self>, pos: usize) -> &$keyTypeName {
                $module::get_unchecked_key(hash, pos as isize)
            }

            unsafe fn get_unchecked_value(hash: &QHash<Self>, pos: usize) -> &$valueTypeName {
                $module::get_unchecked_value(hash, pos as isize)
            }

            fn insert(hash: &mut QHash<Self>, key: $keyTypeName, value: $valueTypeName) {
                $module::insert(hash, &key, &value);
            }

            fn insert_clone(hash: &mut QHash<Self>, key: &$keyTypeName, value: &$valueTypeName) {
                $module::insert(hash, key, value);
            }

            fn len(hash: &QHash<Self>) -> usize {
                $module::len(hash) as usize
            }

            fn remove(hash: &mut QHash<Self>, key: &$keyTypeName) -> bool {
                $module::remove(hash, key)
            }

            fn value(hash: &QHash<Self>, key: &$keyTypeName) -> $valueTypeName {
                $module::value(hash, key)
            }
        }
    };
}

// For now we will implement useful combinations for Qt
// Other combinations the developer will have to implement themselves
// or a generator could be made later https://github.com/KDAB/cxx-qt/issues/355
//
// QVariantHash
impl_qhash_pair!(
    crate::QString,
    crate::QVariant,
    qhash_qstring_qvariant,
    QHashPair_QString_QVariant,
    "QHash_QString_QVariant"
);
