// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "qt_gui")]
use crate::QColor;
use crate::{
    QDate, QDateTime, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QString, QTime, QUrl, QVariant,
};
use core::{marker::PhantomData, mem::MaybeUninit};
use cxx::{type_id, ExternType};

mod qlist_bool;
mod qlist_f32;
mod qlist_f64;
mod qlist_i16;
mod qlist_i32;
mod qlist_i64;
mod qlist_i8;
#[cfg(feature = "qt_gui")]
mod qlist_qcolor;
mod qlist_qdate;
mod qlist_qdatetime;
mod qlist_qpoint;
mod qlist_qpointf;
mod qlist_qrect;
mod qlist_qrectf;
mod qlist_qsize;
mod qlist_qsizef;
mod qlist_qstring;
mod qlist_qtime;
mod qlist_qurl;
mod qlist_qvariant;
mod qlist_u16;
mod qlist_u32;
mod qlist_u64;
mod qlist_u8;

/// The QList class is a template class that provides a dynamic array.
///
/// To use QList with a custom type, implement the [`QListElement`] trait for T.
#[repr(C)]
pub struct QList<T>
where
    T: QListElement,
{
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QList has one pointer as a member
    /// Qt6 QVector/QList has one member, which contains two pointers and a size_t
    #[cfg(qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
    _value: PhantomData<T>,
}

impl<T> Clone for QList<T>
where
    T: QListElement,
{
    /// Constructs a copy of the QList.
    fn clone(&self) -> Self {
        T::clone(self)
    }
}

impl<T> Default for QList<T>
where
    T: QListElement,
{
    /// Constructs an empty list.
    fn default() -> Self {
        T::default()
    }
}

impl<T> Drop for QList<T>
where
    T: QListElement,
{
    /// Destroys the QList.
    fn drop(&mut self) {
        T::drop(self);
    }
}

impl<T> QList<T>
where
    T: QListElement,
{
    /// Inserts value at the end of the list.
    ///
    /// The value is a reference here so it can be opaque or trivial but
    /// note that the value is copied when being appended into the list.
    pub fn append_clone(&mut self, value: &T) {
        T::append_clone(self, value);
    }

    /// Removes all elements from the list.
    pub fn clear(&mut self) {
        T::clear(self);
    }

    /// Returns true if the list contains item value; otherwise returns false.
    pub fn contains(&self, value: &T) -> bool {
        T::contains(self, value)
    }

    /// Returns the item at index position in the list.
    ///
    /// index must be a valid position in the list (i.e., 0 <= index < len()).
    pub fn get(&self, index: isize) -> Option<&T> {
        if index >= 0 && index < self.len() {
            Some(unsafe { T::get_unchecked(self, index) })
        } else {
            None
        }
    }

    /// Returns the index position of the first occurrence of value in the list,
    /// searching forward from index position from. Returns -1 if no item matched.
    pub fn index_of(&self, value: &T) -> isize {
        T::index_of(self, value)
    }

    /// Inserts item value into the list at the given position.
    ///
    /// The value is a reference here so it can be opaque or trivial but
    /// note that the value is copied when being inserted into the list.
    pub fn insert_clone(&mut self, pos: isize, value: &T) {
        T::insert_clone(self, pos, value);
    }

    /// Returns true if the list contains no elements; otherwise returns false.
    pub fn is_empty(&self) -> bool {
        T::len(self) == 0
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is &'a T.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            list: self,
            index: 0,
        }
    }

    /// Returns the number of items in the list.
    pub fn len(&self) -> isize {
        T::len(self)
    }

    /// Removes the element at index position.
    pub fn remove(&mut self, pos: isize) {
        T::remove(self, pos);
    }

    /// Reserve the specified capacity to prevent repeated allocations
    /// when the maximum size is known.
    pub fn reserve(&mut self, size: isize) {
        T::reserve(self, size);
    }
}

impl<T> From<&QList<T>> for Vec<T>
where
    T: QListElement + Clone,
{
    /// Convert a reference to a [QList] into a [Vec] by making a deep copy of the data.
    /// The original QList can still be used after constructing the Vec.
    fn from(qlist: &QList<T>) -> Self {
        let mut vec = Vec::with_capacity(qlist.len().try_into().unwrap());
        for element in qlist.iter() {
            vec.push(element.clone());
        }
        vec
    }
}

impl<T, S> From<S> for QList<T>
where
    T: QListElement + Clone,
    S: AsRef<[T]>,
{
    /// Convert anything that can be cheaply converted to a slice, such as an [array] or [Vec], into a [QList]
    /// by making a deep copy of the data.
    /// The original slice can still be used after constructing the QList.
    fn from(vec: S) -> Self {
        let mut qlist = Self::default();
        qlist.reserve(vec.as_ref().len().try_into().unwrap());
        for element in vec.as_ref() {
            qlist.append_clone(element);
        }
        qlist
    }
}

impl<T> QList<T>
where
    T: QListElement + ExternType<Kind = cxx::kind::Trivial>,
{
    /// Inserts value at the end of the list.
    pub fn append(&mut self, value: T) {
        T::append(self, value);
    }

    /// Inserts item value into the list at the given position.
    pub fn insert(&mut self, pos: isize, value: T) {
        T::insert(self, pos, value);
    }
}

unsafe impl<T> ExternType for QList<T>
where
    T: ExternType + QListElement,
{
    type Id = T::TypeId;
    type Kind = cxx::kind::Trivial;
}

pub struct Iter<'a, T>
where
    T: QListElement,
{
    list: &'a QList<T>,
    index: isize,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: QListElement,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.len() {
            let next = unsafe { T::get_unchecked(self.list, self.index) };
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
    T: QListElement,
{
    fn len(&self) -> usize {
        (self.list.len() - self.index) as usize
    }
}

/// Trait implementation for an element in a [`QList`].
pub trait QListElement: Sized {
    type TypeId;

    fn append(list: &mut QList<Self>, value: Self)
    where
        Self: ExternType<Kind = cxx::kind::Trivial>;
    fn append_clone(list: &mut QList<Self>, value: &Self);
    fn clear(list: &mut QList<Self>);
    fn clone(list: &QList<Self>) -> QList<Self>;
    fn contains(list: &QList<Self>, value: &Self) -> bool;
    fn default() -> QList<Self>;
    fn drop(list: &mut QList<Self>);
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked(list: &QList<Self>, pos: isize) -> &Self;
    fn index_of(list: &QList<Self>, value: &Self) -> isize;
    fn insert(list: &mut QList<Self>, pos: isize, value: Self)
    where
        Self: ExternType<Kind = cxx::kind::Trivial>;
    fn insert_clone(list: &mut QList<Self>, pos: isize, value: &Self);
    fn len(list: &QList<Self>) -> isize;
    fn remove(list: &mut QList<Self>, pos: isize);
    fn reserve(vector: &mut QList<Self>, size: isize);
}

macro_rules! impl_qlist_element {
    ( $typeName:ty, $module:ident, $typeId:literal ) => {
        impl QListElement for $typeName {
            type TypeId = type_id!($typeId);

            fn append(list: &mut QList<Self>, value: Self) {
                $module::append(list, &value);
            }

            fn append_clone(list: &mut QList<Self>, value: &Self) {
                $module::append(list, value);
            }

            fn clear(list: &mut QList<Self>) {
                list.cxx_clear()
            }

            fn clone(list: &QList<Self>) -> QList<Self> {
                $module::clone(list)
            }

            fn contains(list: &QList<Self>, value: &Self) -> bool {
                list.cxx_contains(value)
            }

            fn default() -> QList<Self> {
                $module::default()
            }

            fn drop(list: &mut QList<Self>) {
                $module::drop(list);
            }

            unsafe fn get_unchecked(list: &QList<Self>, pos: isize) -> &Self {
                $module::get_unchecked(list, pos)
            }

            fn index_of(list: &QList<Self>, value: &Self) -> isize {
                $module::index_of(list, value)
            }

            fn insert(list: &mut QList<Self>, pos: isize, value: Self) {
                $module::insert(list, pos, &value);
            }

            fn insert_clone(list: &mut QList<Self>, pos: isize, value: &Self) {
                $module::insert(list, pos, value);
            }

            fn len(list: &QList<Self>) -> isize {
                $module::len(list)
            }

            fn remove(list: &mut QList<Self>, pos: isize) {
                $module::remove(list, pos);
            }

            fn reserve(list: &mut QList<Self>, size: isize) {
                $module::reserve(list, size);
            }
        }
    };
}

impl_qlist_element!(bool, qlist_bool, "QList_bool");
impl_qlist_element!(f32, qlist_f32, "QList_f32");
impl_qlist_element!(f64, qlist_f64, "QList_f64");
impl_qlist_element!(i8, qlist_i8, "QList_i8");
impl_qlist_element!(i16, qlist_i16, "QList_i16");
impl_qlist_element!(i32, qlist_i32, "QList_i32");
impl_qlist_element!(i64, qlist_i64, "QList_i64");
#[cfg(feature = "qt_gui")]
impl_qlist_element!(QColor, qlist_qcolor, "QList_QColor");
impl_qlist_element!(QDate, qlist_qdate, "QList_QDate");
impl_qlist_element!(QDateTime, qlist_qdatetime, "QList_QDateTime");
impl_qlist_element!(QPoint, qlist_qpoint, "QList_QPoint");
impl_qlist_element!(QPointF, qlist_qpointf, "QList_QPointF");
impl_qlist_element!(QRect, qlist_qrect, "QList_QRect");
impl_qlist_element!(QRectF, qlist_qrectf, "QList_QRectF");
impl_qlist_element!(QSize, qlist_qsize, "QList_QSize");
impl_qlist_element!(QSizeF, qlist_qsizef, "QList_QSizeF");
impl_qlist_element!(QString, qlist_qstring, "QList_QString");
impl_qlist_element!(QTime, qlist_qtime, "QList_QTime");
impl_qlist_element!(QUrl, qlist_qurl, "QList_QUrl");
impl_qlist_element!(QVariant, qlist_qvariant, "QList_QVariant");
impl_qlist_element!(u8, qlist_u8, "QList_u8");
impl_qlist_element!(u16, qlist_u16, "QList_u16");
impl_qlist_element!(u32, qlist_u32, "QList_u32");
impl_qlist_element!(u64, qlist_u64, "QList_u64");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn qlist_from_array_to_vec() {
        let array = [0, 1, 2];
        let qlist = QList::<u8>::from(array);
        assert_eq!(Vec::from(&qlist), array);
    }
}
