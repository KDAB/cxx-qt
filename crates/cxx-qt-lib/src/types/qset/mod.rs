// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{QDate, QDateTime, QString, QTime, QUrl};
use core::{marker::PhantomData, mem::MaybeUninit};
use cxx::{type_id, ExternType};

mod qset_bool;
mod qset_f32;
mod qset_f64;
mod qset_i16;
mod qset_i32;
mod qset_i8;
mod qset_qdate;
mod qset_qdatetime;
mod qset_qstring;
mod qset_qtime;
mod qset_qurl;
mod qset_u16;
mod qset_u32;
mod qset_u8;

/// The QSet class is a template class that provides a hash-table-based set.
///
/// Note that this means that T needs to have a global `qHash()` function.
///
/// To use QSet with a custom type, implement the [`QSetElement`] trait for T.
#[repr(C)]
pub struct QSet<T>
where
    T: QSetElement,
{
    _space: MaybeUninit<usize>,
    _value: PhantomData<T>,
}

impl<T> Clone for QSet<T>
where
    T: QSetElement,
{
    /// Constructs a copy of the QSet.
    fn clone(&self) -> Self {
        T::clone(self)
    }
}

impl<T> Default for QSet<T>
where
    T: QSetElement,
{
    /// Constructs an empty set.
    fn default() -> Self {
        T::default()
    }
}

impl<T> Drop for QSet<T>
where
    T: QSetElement,
{
    /// Destroys the QSet.
    fn drop(&mut self) {
        T::drop(self);
    }
}

impl<T> QSet<T>
where
    T: QSetElement,
{
    /// Removes all elements from the set.
    pub fn clear(&mut self) {
        T::clear(self);
    }

    /// Returns true if the set contains item value; otherwise returns false.
    pub fn contains(&self, value: &T) -> bool {
        T::contains(self, value)
    }

    /// Inserts item value into the set, if value isn't already in the set,
    /// and returns an iterator pointing at the inserted item.
    ///
    /// The value is a reference here so it can be opaque or trivial but
    /// note that the value is copied when being inserted into the set.
    pub fn insert_clone(&mut self, value: &T) {
        T::insert_clone(self, value);
    }

    /// Returns true if the set contains no elements; otherwise returns false.
    pub fn is_empty(&self) -> bool {
        T::len(self) == 0
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is &'a T.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            set: self,
            index: 0,
        }
    }

    /// Returns the number of items in the set.
    pub fn len(&self) -> usize {
        T::len(self)
    }

    /// Removes any occurrence of item value from the set.
    /// Returns true if an item was actually removed; otherwise returns false.
    pub fn remove(&mut self, value: &T) -> bool {
        T::remove(self, value)
    }
}

impl<T> QSet<T>
where
    T: QSetElement + ExternType<Kind = cxx::kind::Trivial>,
{
    /// Inserts item value into the set, if value isn't already in the set,
    /// and returns an iterator pointing at the inserted item.
    pub fn insert(&mut self, value: T) {
        T::insert(self, value);
    }
}

unsafe impl<T> ExternType for QSet<T>
where
    T: ExternType + QSetElement,
{
    type Id = T::TypeId;
    type Kind = cxx::kind::Trivial;
}

pub struct Iter<'a, T>
where
    T: QSetElement,
{
    set: &'a QSet<T>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: QSetElement,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.set.len() {
            let next = unsafe { T::get_unchecked(self.set, self.index) };
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
    T: QSetElement,
{
    fn len(&self) -> usize {
        self.set.len() - self.index
    }
}

/// Trait implementation for an element in a [`QSet`].
pub trait QSetElement: Sized {
    type TypeId;

    fn clear(set: &mut QSet<Self>);
    fn clone(set: &QSet<Self>) -> QSet<Self>;
    fn contains(set: &QSet<Self>, value: &Self) -> bool;
    fn default() -> QSet<Self>;
    fn drop(set: &mut QSet<Self>);
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked(set: &QSet<Self>, pos: usize) -> &Self;
    fn insert(set: &mut QSet<Self>, value: Self)
    where
        Self: ExternType<Kind = cxx::kind::Trivial>;
    fn insert_clone(set: &mut QSet<Self>, value: &Self);
    fn len(set: &QSet<Self>) -> usize;
    fn remove(set: &mut QSet<Self>, value: &Self) -> bool;
}

macro_rules! impl_qset_element {
    ( $typeName:ty, $module:ident, $typeId:literal ) => {
        impl QSetElement for $typeName {
            type TypeId = type_id!($typeId);

            fn clear(set: &mut QSet<Self>) {
                set.cxx_clear()
            }

            fn clone(set: &QSet<Self>) -> QSet<Self> {
                $module::clone(set)
            }

            fn contains(set: &QSet<Self>, value: &Self) -> bool {
                set.cxx_contains(value)
            }

            fn default() -> QSet<Self> {
                $module::default()
            }

            fn drop(set: &mut QSet<Self>) {
                $module::drop(set);
            }

            unsafe fn get_unchecked(set: &QSet<Self>, pos: usize) -> &Self {
                $module::get_unchecked(set, pos)
            }

            fn insert(set: &mut QSet<Self>, value: Self) {
                $module::insert(set, &value);
            }

            fn insert_clone(set: &mut QSet<Self>, value: &Self) {
                $module::insert(set, value);
            }

            fn len(set: &QSet<Self>) -> usize {
                $module::len(set)
            }

            fn remove(set: &mut QSet<Self>, value: &Self) -> bool {
                set.cxx_remove(value)
            }
        }
    };
}

impl_qset_element!(bool, qset_bool, "QSet_bool");
impl_qset_element!(f32, qset_f32, "QSet_f32");
impl_qset_element!(f64, qset_f64, "QSet_f64");
impl_qset_element!(i8, qset_i8, "QSet_i8");
impl_qset_element!(i16, qset_i16, "QSet_i16");
impl_qset_element!(i32, qset_i32, "QSet_i32");
impl_qset_element!(QDate, qset_qdate, "QSet_QDate");
impl_qset_element!(QDateTime, qset_qdatetime, "QSet_QDateTime");
impl_qset_element!(QString, qset_qstring, "QSet_QString");
impl_qset_element!(QTime, qset_qtime, "QSet_QTime");
impl_qset_element!(QUrl, qset_qurl, "QSet_QUrl");
impl_qset_element!(u8, qset_u8, "QSet_u8");
impl_qset_element!(u16, qset_u16, "QSet_u16");
impl_qset_element!(u32, qset_u32, "QSet_u32");
