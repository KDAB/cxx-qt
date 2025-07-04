// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "qt_gui")]
use crate::QColor;
#[cfg(not(target_os = "emscripten"))]
use crate::QDateTime;
use crate::{
    QByteArray, QDate, QLine, QLineF, QMargins, QMarginsF, QPersistentModelIndex, QPoint, QPointF,
    QRect, QRectF, QSize, QSizeF, QString, QTime, QUrl, QUuid, QVariant,
};
use core::{marker::PhantomData, mem::MaybeUninit};
use cxx::{type_id, ExternType};
use std::fmt;

/// The `QList` class is a template class that provides a dynamic array.
///
/// To use `QList` with a custom type, implement the [`QListElement`] trait for `T`.
///
/// Qt Documentation: [QList]("https://doc.qt.io/qt/qlist.html#details")
#[repr(C)]
pub struct QList<T>
where
    T: QListElement,
{
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 `QList` has one pointer as a member
    /// Qt6 `QVector`/`QList` has one member, which contains two pointers and a size_t
    #[cfg(cxxqt_qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(cxxqt_qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
    _value: PhantomData<T>,
}

impl<T> Clone for QList<T>
where
    T: QListElement,
{
    /// Constructs a copy of the `QList`.
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
    /// Destroys the `QList`.
    fn drop(&mut self) {
        T::drop(self);
    }
}

impl<T> PartialEq for QList<T>
where
    T: QListElement + PartialEq,
{
    /// Returns `true` if both lists contain the same elements in the same order, otherwise `false`.
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T> Eq for QList<T> where T: QListElement + Eq {}

impl<T> fmt::Debug for QList<T>
where
    T: QListElement + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> QList<T>
where
    T: QListElement,
{
    /// Inserts `value` at the end of the list.
    ///
    /// The value is a reference here so it can be opaque or trivial but
    /// note that the value is copied when being appended into the list.
    pub fn append_clone(&mut self, value: &T) {
        T::append_clone(self, value);
    }

    /// Removes all elements from the list.
    ///
    /// In Qt 6, the capacity is preserved. In Qt 5, this function releases the memory used by the list.
    pub fn clear(&mut self) {
        T::clear(self);
    }

    /// Returns `true` if the list contains an occurrence of `value`; otherwise returns `false`.
    pub fn contains(&self, value: &T) -> bool {
        T::contains(self, value)
    }

    /// Returns the item at index position `index` in the list, or `None` if `index` is out of bounds (i.e. `index < 0 || index >= self.len()`).
    pub fn get(&self, index: isize) -> Option<&T> {
        if index >= 0 && index < self.len() {
            Some(unsafe { T::get_unchecked(self, index) })
        } else {
            None
        }
    }

    /// Returns the index position of the first occurrence of `value` in the list. Returns -1 if no item matched.
    pub fn index_of(&self, value: &T) -> isize {
        T::index_of(self, value)
    }

    /// Inserts item `value` into the list at index position `pos`.
    ///
    /// The value is a reference here so it can be opaque or trivial but
    /// note that the value is copied when being inserted into the list.
    pub fn insert_clone(&mut self, pos: isize, value: &T) {
        T::insert_clone(self, pos, value);
    }

    /// Returns `true` if the list has size 0; otherwise returns `false`.
    pub fn is_empty(&self) -> bool {
        T::len(self) == 0
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `&'a T`.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            index: 0,
        }
    }

    /// Returns the number of items in the list.
    pub fn len(&self) -> isize {
        T::len(self)
    }

    /// Removes the element at index position `pos`.
    ///
    /// Element removal will preserve the list's capacity and not reduce the amount of allocated memory.
    pub fn remove(&mut self, pos: isize) {
        T::remove(self, pos);
    }

    /// Attempts to allocate memory for at least `size` elements.
    ///
    /// If you know in advance how large the list will be, you should call this function to prevent reallocations and memory fragmentation. If you resize the list often, you are also likely to get better performance.
    ///
    /// If in doubt about how much space shall be needed, it is usually better to use an upper bound as `size`, or a high estimate of the most likely size, if a strict upper bound would be much bigger than this. If `size` is an underestimate, the list will grow as needed once the reserved size is exceeded, which may lead to a larger allocation than your best overestimate would have and will slow the operation that triggers it.
    pub fn reserve(&mut self, size: isize) {
        T::reserve(self, size);
    }

    /// Helper function for handling Rust values.
    pub(crate) fn reserve_usize(&mut self, size: usize) {
        if size != 0 {
            T::reserve(self, isize::try_from(size).unwrap_or(isize::MAX));
        }
    }
}

impl<T> QList<T>
where
    T: QListElement + ExternType<Kind = cxx::kind::Trivial>,
{
    /// Inserts `value` at the end of the list.
    pub fn append(&mut self, value: T) {
        T::append(self, value);
    }

    /// Inserts item `value` into the list at index position `pos`.
    pub fn insert(&mut self, pos: isize, value: T) {
        T::insert(self, pos, value);
    }
}

impl<T> From<&QList<T>> for Vec<T>
where
    T: QListElement + Clone,
{
    /// Convert a reference to a [`QList`] into a [`Vec`] by making a deep copy of the data.
    /// The original `QList` can still be used after constructing the `Vec`.
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
    /// Convert anything that can be cheaply converted to a slice, such as an [array] or [`Vec`], into a [`QList`]
    /// by making a deep copy of the data.
    /// The original slice can still be used after constructing the `QList`.
    fn from(vec: S) -> Self {
        let mut qlist = Self::default();
        qlist.reserve_usize(vec.as_ref().len());
        for element in vec.as_ref() {
            qlist.append_clone(element);
        }
        qlist
    }
}

impl<'a, T> Extend<&'a T> for QList<T>
where
    T: QListElement,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve_usize(iter.size_hint().0);
        for element in iter {
            self.append_clone(element);
        }
    }
}

impl<T> Extend<T> for QList<T>
where
    T: QListElement + ExternType<Kind = cxx::kind::Trivial>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve_usize(iter.size_hint().0);
        for element in iter {
            self.append(element);
        }
    }
}

impl<'a, T> FromIterator<&'a T> for QList<T>
where
    T: QListElement,
{
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let mut qlist = Self::default();
        qlist.extend(iter);
        qlist
    }
}

impl<T> FromIterator<T> for QList<T>
where
    T: QListElement + ExternType<Kind = cxx::kind::Trivial>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut qlist = Self::default();
        qlist.extend(iter);
        qlist
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

impl<T> ExactSizeIterator for Iter<'_, T>
where
    T: QListElement,
{
    fn len(&self) -> usize {
        (self.list.len() - self.index) as usize
    }
}

impl<'a, T> IntoIterator for &'a QList<T>
where
    T: QListElement,
{
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
        mod $module;

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
impl_qlist_element!(QByteArray, qlist_qbytearray, "QList_QByteArray");
#[cfg(feature = "qt_gui")]
impl_qlist_element!(QColor, qlist_qcolor, "QList_QColor");
impl_qlist_element!(QDate, qlist_qdate, "QList_QDate");
#[cfg(not(target_os = "emscripten"))]
impl_qlist_element!(QDateTime, qlist_qdatetime, "QList_QDateTime");
impl_qlist_element!(QLine, qlist_qline, "QList_QLine");
impl_qlist_element!(QLineF, qlist_qlinef, "QList_QLineF");
impl_qlist_element!(QMargins, qlist_qmargins, "QList_QMargins");
impl_qlist_element!(QMarginsF, qlist_qmarginsf, "QList_QMarginsF");
impl_qlist_element!(
    QPersistentModelIndex,
    qlist_qpersistentmodelindex,
    "QList_QPersistentModelIndex"
);
impl_qlist_element!(QPoint, qlist_qpoint, "QList_QPoint");
impl_qlist_element!(QPointF, qlist_qpointf, "QList_QPointF");
impl_qlist_element!(QRect, qlist_qrect, "QList_QRect");
impl_qlist_element!(QRectF, qlist_qrectf, "QList_QRectF");
impl_qlist_element!(QSize, qlist_qsize, "QList_QSize");
impl_qlist_element!(QSizeF, qlist_qsizef, "QList_QSizeF");
impl_qlist_element!(QString, qlist_qstring, "QList_QString");
impl_qlist_element!(QTime, qlist_qtime, "QList_QTime");
impl_qlist_element!(QUrl, qlist_qurl, "QList_QUrl");
impl_qlist_element!(QUuid, qlist_quuid, "QList_QUuid");
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

    #[cfg(feature = "serde")]
    #[test]
    fn qlist_serde() {
        let qlist = QList::<u8>::from([0, 1, 2]);
        assert_eq!(crate::serde_impl::roundtrip(&qlist), qlist);
    }
}
