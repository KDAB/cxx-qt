// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "qt_gui")]
use crate::QColor;
use crate::{
    QByteArray, QDate, QDateTime, QPoint, QPointF, QRect, QRectF, QSize, QSizeF, QString, QTime,
    QUrl, QVariant,
};
use core::{marker::PhantomData, mem::MaybeUninit};
use cxx::{type_id, ExternType};

mod qvector_bool;
mod qvector_f32;
mod qvector_f64;
mod qvector_i16;
mod qvector_i32;
mod qvector_i64;
mod qvector_i8;
mod qvector_qbytearray;
#[cfg(feature = "qt_gui")]
mod qvector_qcolor;
mod qvector_qdate;
mod qvector_qdatetime;
mod qvector_qpoint;
mod qvector_qpointf;
mod qvector_qrect;
mod qvector_qrectf;
mod qvector_qsize;
mod qvector_qsizef;
mod qvector_qstring;
mod qvector_qtime;
mod qvector_qurl;
mod qvector_qvariant;
mod qvector_u16;
mod qvector_u32;
mod qvector_u64;
mod qvector_u8;

/// The QVector class is a template class that provides a dynamic array.
///
/// To use QVector with a custom type, implement the [`QVectorElement`] trait for T.
#[repr(C)]
pub struct QVector<T>
where
    T: QVectorElement,
{
    /// The layout has changed between Qt 5 and Qt 6
    ///
    /// Qt5 QVector has one pointer as a member
    /// Qt6 QVector/QList has one member, which contains two pointers and a size_t
    #[cfg(qt_version_major = "5")]
    _space: MaybeUninit<usize>,
    #[cfg(qt_version_major = "6")]
    _space: MaybeUninit<[usize; 3]>,
    _value: PhantomData<T>,
}

impl<T> Clone for QVector<T>
where
    T: QVectorElement,
{
    /// Constructs a copy of the QVector.
    fn clone(&self) -> Self {
        T::clone(self)
    }
}

impl<T> Default for QVector<T>
where
    T: QVectorElement,
{
    /// Constructs an empty vector.
    fn default() -> Self {
        T::default()
    }
}

impl<T> Drop for QVector<T>
where
    T: QVectorElement,
{
    /// Destroys the QVector.
    fn drop(&mut self) {
        T::drop(self);
    }
}

impl<T> QVector<T>
where
    T: QVectorElement,
{
    /// Inserts value at the end of the vector.
    ///
    /// The value is a reference here so it can be opaque or trivial but
    /// note that the value is copied when being appended into the vector.
    pub fn append_clone(&mut self, value: &T) {
        T::append_clone(self, value);
    }

    /// Removes all elements from the vector.
    pub fn clear(&mut self) {
        T::clear(self);
    }

    /// Returns true if the vector contains item value; otherwise returns false.
    pub fn contains(&self, value: &T) -> bool {
        T::contains(self, value)
    }

    /// Returns the item at index position in the vector.
    ///
    /// index must be a valid position in the vector (i.e., 0 <= index < len()).
    pub fn get(&self, index: isize) -> Option<&T> {
        if index >= 0 && index < self.len() {
            Some(unsafe { T::get_unchecked(self, index) })
        } else {
            None
        }
    }

    /// Returns the index position of the first occurrence of value in the vector,
    /// searching forward from index position from. Returns -1 if no item matched.
    pub fn index_of(&self, value: &T) -> isize {
        T::index_of(self, value)
    }

    /// Inserts item value into the vector at the given position.
    ///
    /// The value is a reference here so it can be opaque or trivial but
    /// note that the value is copied when being inserted into the vector.
    pub fn insert_clone(&mut self, pos: isize, value: &T) {
        T::insert_clone(self, pos, value);
    }

    /// Returns true if the vector contains no elements; otherwise returns false.
    pub fn is_empty(&self) -> bool {
        T::len(self) == 0
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is &'a T.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            vector: self,
            index: 0,
        }
    }

    /// Returns the number of items in the vector.
    pub fn len(&self) -> isize {
        T::len(self)
    }

    /// Reserve the specified capacity to prevent repeated allocations
    /// when the maximum size is known.
    pub fn reserve(&mut self, size: isize) {
        T::reserve(self, size);
    }

    /// Removes the element at index position.
    pub fn remove(&mut self, pos: isize) {
        T::remove(self, pos);
    }
}

impl<T> QVector<T>
where
    T: QVectorElement + ExternType<Kind = cxx::kind::Trivial>,
{
    /// Inserts value at the end of the vector.
    pub fn append(&mut self, value: T) {
        T::append(self, value);
    }

    /// Inserts item value into the vector at the given position.
    pub fn insert(&mut self, pos: isize, value: T) {
        T::insert(self, pos, value);
    }
}

impl<T> From<&QVector<T>> for Vec<T>
where
    T: QVectorElement + Clone,
{
    /// Convert a reference to a [QVector] into a [Vec] by making a deep copy of the data.
    /// The original QVector can still be used after constructing the Vec.
    fn from(qvec: &QVector<T>) -> Self {
        let mut vec = Vec::with_capacity(qvec.len().try_into().unwrap());
        for element in qvec.iter() {
            vec.push(element.clone());
        }
        vec
    }
}

impl<T, S> From<S> for QVector<T>
where
    T: QVectorElement + Clone,
    S: AsRef<[T]>,
{
    /// Convert anything that can be cheaply converted to a slice, such as an [array] or [Vec], into a [QVector]
    /// by making a deep copy of the data.
    /// The original slice can still be used after constructing the QVector.
    fn from(vec: S) -> Self {
        let mut qvec = Self::default();
        qvec.reserve(vec.as_ref().len().try_into().unwrap());
        for element in vec.as_ref() {
            qvec.append_clone(element);
        }
        qvec
    }
}

unsafe impl<T> ExternType for QVector<T>
where
    T: ExternType + QVectorElement,
{
    type Id = T::TypeId;
    type Kind = cxx::kind::Trivial;
}

pub struct Iter<'a, T>
where
    T: QVectorElement,
{
    vector: &'a QVector<T>,
    index: isize,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: QVectorElement,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vector.len() {
            let next = unsafe { T::get_unchecked(self.vector, self.index) };
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
    T: QVectorElement,
{
    fn len(&self) -> usize {
        (self.vector.len() - self.index) as usize
    }
}

/// Trait implementation for an element in a [`QVector`].
pub trait QVectorElement: Sized {
    type TypeId;

    fn append(vector: &mut QVector<Self>, value: Self)
    where
        Self: ExternType<Kind = cxx::kind::Trivial>;
    fn append_clone(vector: &mut QVector<Self>, value: &Self);
    fn clear(vector: &mut QVector<Self>);
    fn clone(vector: &QVector<Self>) -> QVector<Self>;
    fn contains(vector: &QVector<Self>, value: &Self) -> bool;
    fn default() -> QVector<Self>;
    fn drop(vector: &mut QVector<Self>);
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked(vector: &QVector<Self>, pos: isize) -> &Self;
    fn index_of(vector: &QVector<Self>, value: &Self) -> isize;
    fn insert(vector: &mut QVector<Self>, pos: isize, value: Self)
    where
        Self: ExternType<Kind = cxx::kind::Trivial>;
    fn insert_clone(vector: &mut QVector<Self>, pos: isize, value: &Self);
    fn len(vector: &QVector<Self>) -> isize;
    fn remove(vector: &mut QVector<Self>, pos: isize);
    fn reserve(vector: &mut QVector<Self>, size: isize);
}

macro_rules! impl_qvector_element {
    ( $typeName:ty, $module:ident, $typeId:literal ) => {
        impl QVectorElement for $typeName {
            type TypeId = type_id!($typeId);

            fn append(vector: &mut QVector<Self>, value: Self) {
                $module::append(vector, &value);
            }

            fn append_clone(vector: &mut QVector<Self>, value: &Self) {
                $module::append(vector, value);
            }

            fn clear(vector: &mut QVector<Self>) {
                vector.cxx_clear()
            }

            fn clone(vector: &QVector<Self>) -> QVector<Self> {
                $module::clone(vector)
            }

            fn contains(vector: &QVector<Self>, value: &Self) -> bool {
                vector.cxx_contains(value)
            }

            fn default() -> QVector<Self> {
                $module::default()
            }

            fn drop(vector: &mut QVector<Self>) {
                $module::drop(vector);
            }

            unsafe fn get_unchecked(vector: &QVector<Self>, pos: isize) -> &Self {
                $module::get_unchecked(vector, pos)
            }

            fn index_of(vector: &QVector<Self>, value: &Self) -> isize {
                $module::index_of(vector, value)
            }

            fn insert(vector: &mut QVector<Self>, pos: isize, value: Self) {
                $module::insert(vector, pos, &value);
            }

            fn insert_clone(vector: &mut QVector<Self>, pos: isize, value: &Self) {
                $module::insert(vector, pos, value);
            }

            fn len(vector: &QVector<Self>) -> isize {
                $module::len(vector)
            }

            fn remove(vector: &mut QVector<Self>, pos: isize) {
                $module::remove(vector, pos);
            }

            fn reserve(vector: &mut QVector<Self>, size: isize) {
                $module::reserve(vector, size);
            }
        }
    };
}

impl_qvector_element!(bool, qvector_bool, "QVector_bool");
impl_qvector_element!(f32, qvector_f32, "QVector_f32");
impl_qvector_element!(f64, qvector_f64, "QVector_f64");
impl_qvector_element!(i8, qvector_i8, "QVector_i8");
impl_qvector_element!(i16, qvector_i16, "QVector_i16");
impl_qvector_element!(i32, qvector_i32, "QVector_i32");
impl_qvector_element!(i64, qvector_i64, "QVector_i64");
impl_qvector_element!(QByteArray, qvector_qbytearray, "QVector_QByteArray");
#[cfg(feature = "qt_gui")]
impl_qvector_element!(QColor, qvector_qcolor, "QVector_QColor");
impl_qvector_element!(QDate, qvector_qdate, "QVector_QDate");
impl_qvector_element!(QDateTime, qvector_qdatetime, "QVector_QDateTime");
impl_qvector_element!(QPoint, qvector_qpoint, "QVector_QPoint");
impl_qvector_element!(QPointF, qvector_qpointf, "QVector_QPointF");
impl_qvector_element!(QRect, qvector_qrect, "QVector_QRect");
impl_qvector_element!(QRectF, qvector_qrectf, "QVector_QRectF");
impl_qvector_element!(QSize, qvector_qsize, "QVector_QSize");
impl_qvector_element!(QSizeF, qvector_qsizef, "QVector_QSizeF");
impl_qvector_element!(QString, qvector_qstring, "QVector_QString");
impl_qvector_element!(QTime, qvector_qtime, "QVector_QTime");
impl_qvector_element!(QUrl, qvector_qurl, "QVector_QUrl");
impl_qvector_element!(QVariant, qvector_qvariant, "QVector_QVariant");
impl_qvector_element!(u8, qvector_u8, "QVector_u8");
impl_qvector_element!(u16, qvector_u16, "QVector_u16");
impl_qvector_element!(u32, qvector_u32, "QVector_u32");
impl_qvector_element!(u64, qvector_u64, "QVector_u64");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn qvec_from_array_to_vec() {
        let array = [0, 1, 2];
        let qvec = QVector::<u8>::from(array);
        assert_eq!(Vec::from(&qvec), array);
    }
}
