// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{
    QColor, QDate, QDateTime, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QString, QTime, QUrl,
    QVariant,
};
use core::{marker::PhantomData, mem::MaybeUninit};
use cxx::{type_id, ExternType};

mod qt5list_bool;
mod qt5list_f32;
mod qt5list_f64;
mod qt5list_i16;
mod qt5list_i32;
mod qt5list_i64;
mod qt5list_i8;
mod qt5list_qcolor;
mod qt5list_qdate;
mod qt5list_qdatetime;
mod qt5list_qpoint;
mod qt5list_qpointf;
mod qt5list_qrect;
mod qt5list_qrectf;
mod qt5list_qsize;
mod qt5list_qsizef;
mod qt5list_qstring;
mod qt5list_qtime;
mod qt5list_qurl;
mod qt5list_qvariant;
mod qt5list_u16;
mod qt5list_u32;
mod qt5list_u64;
mod qt5list_u8;

/// The Qt5List class is a template class that provides a dynamic array.
///
/// To use Qt5List with a custom type, implement the [`Qt5ListElement`] trait for T.
#[repr(C)]
pub struct Qt5List<T>
where
    T: Qt5ListElement,
{
    /// Qt5 Qt5List has one pointer as a member
    _space: MaybeUninit<usize>,
    _value: PhantomData<T>,
}

impl<T> Clone for Qt5List<T>
where
    T: Qt5ListElement,
{
    /// Constructs a copy of the Qt5List.
    fn clone(&self) -> Self {
        T::clone(self)
    }
}

impl<T> Default for Qt5List<T>
where
    T: Qt5ListElement,
{
    /// Constructs an empty list.
    fn default() -> Self {
        T::default()
    }
}

impl<T> Drop for Qt5List<T>
where
    T: Qt5ListElement,
{
    /// Destroys the Qt5List.
    fn drop(&mut self) {
        T::drop(self);
    }
}

impl<T> Qt5List<T>
where
    T: Qt5ListElement,
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
    pub fn get(&self, index: i32) -> Option<&T> {
        if index >= 0 && index < self.len() {
            Some(unsafe { T::get_unchecked(self, index) })
        } else {
            None
        }
    }

    /// Returns the index position of the first occurrence of value in the list,
    /// searching forward from index position from. Returns -1 if no item matched.
    pub fn index_of(&self, value: &T) -> i32 {
        T::index_of(self, value)
    }

    /// Inserts item value into the list, if value isn't already in the list,
    /// and returns an iterator pointing at the inserted item.
    ///
    /// The value is a reference here so it can be opaque or trivial but
    /// note that the value is copied when being inserted into the list.
    pub fn insert_clone(&mut self, pos: i32, value: &T) {
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
    pub fn len(&self) -> i32 {
        T::len(self)
    }

    /// Removes the element at index position.
    pub fn remove(&mut self, pos: i32) {
        T::remove(self, pos);
    }
}

impl<T> Qt5List<T>
where
    T: Qt5ListElement + ExternType<Kind = cxx::kind::Trivial>,
{
    /// Inserts value at the end of the list.
    pub fn append(&mut self, value: T) {
        T::append(self, value);
    }

    /// Inserts item value into the list, if value isn't already in the list,
    /// and returns an iterator pointing at the inserted item.
    pub fn insert(&mut self, pos: i32, value: T) {
        T::insert(self, pos, value);
    }
}

unsafe impl<T> ExternType for Qt5List<T>
where
    T: ExternType + Qt5ListElement,
{
    type Id = T::TypeId;
    type Kind = cxx::kind::Trivial;
}

pub struct Iter<'a, T>
where
    T: Qt5ListElement,
{
    list: &'a Qt5List<T>,
    index: i32,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Qt5ListElement,
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
    T: Qt5ListElement,
{
    fn len(&self) -> usize {
        (self.list.len() - self.index) as usize
    }
}

/// Trait implementation for an element in a [`Qt5List`].
pub trait Qt5ListElement: Sized {
    type TypeId;

    fn append(list: &mut Qt5List<Self>, value: Self)
    where
        Self: ExternType<Kind = cxx::kind::Trivial>;
    fn append_clone(list: &mut Qt5List<Self>, value: &Self);
    fn clear(list: &mut Qt5List<Self>);
    fn clone(list: &Qt5List<Self>) -> Qt5List<Self>;
    fn contains(list: &Qt5List<Self>, value: &Self) -> bool;
    fn default() -> Qt5List<Self>;
    fn drop(list: &mut Qt5List<Self>);
    fn index_of(list: &Qt5List<Self>, value: &Self) -> i32;
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked(list: &Qt5List<Self>, pos: i32) -> &Self;
    fn insert(list: &mut Qt5List<Self>, pos: i32, value: Self)
    where
        Self: ExternType<Kind = cxx::kind::Trivial>;
    fn insert_clone(list: &mut Qt5List<Self>, pos: i32, value: &Self);
    fn len(list: &Qt5List<Self>) -> i32;
    fn remove(list: &mut Qt5List<Self>, pos: i32);
}

macro_rules! impl_qt5list_element {
    ( $typeName:ty, $module:ident, $typeId:literal ) => {
        impl Qt5ListElement for $typeName {
            type TypeId = type_id!($typeId);

            fn append(list: &mut Qt5List<Self>, value: Self) {
                list.cxx_append(&value);
            }

            fn append_clone(list: &mut Qt5List<Self>, value: &Self) {
                list.cxx_append(value);
            }

            fn clear(list: &mut Qt5List<Self>) {
                list.cxx_clear()
            }

            fn clone(list: &Qt5List<Self>) -> Qt5List<Self> {
                $module::clone(list)
            }

            fn contains(list: &Qt5List<Self>, value: &Self) -> bool {
                list.cxx_contains(value)
            }

            fn default() -> Qt5List<Self> {
                $module::default()
            }

            fn drop(list: &mut Qt5List<Self>) {
                $module::drop(list);
            }

            unsafe fn get_unchecked(list: &Qt5List<Self>, pos: i32) -> &Self {
                list.cxx_get_unchecked(pos)
            }

            fn index_of(list: &Qt5List<Self>, value: &Self) -> i32 {
                list.cxx_index_of(value, 0)
            }

            fn insert(list: &mut Qt5List<Self>, pos: i32, value: Self) {
                list.cxx_insert(pos, &value);
            }

            fn insert_clone(list: &mut Qt5List<Self>, pos: i32, value: &Self) {
                list.cxx_insert(pos, value);
            }

            fn len(list: &Qt5List<Self>) -> i32 {
                list.cxx_len()
            }

            fn remove(list: &mut Qt5List<Self>, pos: i32) {
                list.cxx_remove(pos);
            }
        }
    };
}

impl_qt5list_element!(bool, qt5list_bool, "Qt5List_bool");
impl_qt5list_element!(f32, qt5list_f32, "Qt5List_f32");
impl_qt5list_element!(f64, qt5list_f64, "Qt5List_f64");
impl_qt5list_element!(i8, qt5list_i8, "Qt5List_i8");
impl_qt5list_element!(i16, qt5list_i16, "Qt5List_i16");
impl_qt5list_element!(i32, qt5list_i32, "Qt5List_i32");
impl_qt5list_element!(i64, qt5list_i64, "Qt5List_i64");
impl_qt5list_element!(QColor, qt5list_qcolor, "Qt5List_QColor");
impl_qt5list_element!(QDate, qt5list_qdate, "Qt5List_QDate");
impl_qt5list_element!(QDateTime, qt5list_qdatetime, "Qt5List_QDateTime");
impl_qt5list_element!(QPoint, qt5list_qpoint, "Qt5List_QPoint");
impl_qt5list_element!(QPointF, qt5list_qpointf, "Qt5List_QPointF");
impl_qt5list_element!(QRect, qt5list_qrect, "Qt5List_QRect");
impl_qt5list_element!(QRectF, qt5list_qrectf, "Qt5List_QRectF");
impl_qt5list_element!(QSize, qt5list_qsize, "Qt5List_QSize");
impl_qt5list_element!(QSizeF, qt5list_qsizef, "Qt5List_QSizeF");
impl_qt5list_element!(QString, qt5list_qstring, "Qt5List_QString");
impl_qt5list_element!(QTime, qt5list_qtime, "Qt5List_QTime");
impl_qt5list_element!(QUrl, qt5list_qurl, "Qt5List_QUrl");
impl_qt5list_element!(QVariant, qt5list_qvariant, "Qt5List_QVariant");
impl_qt5list_element!(u8, qt5list_u8, "Qt5List_u8");
impl_qt5list_element!(u16, qt5list_u16, "Qt5List_u16");
impl_qt5list_element!(u32, qt5list_u32, "Qt5List_u32");
impl_qt5list_element!(u64, qt5list_u64, "Qt5List_u64");
