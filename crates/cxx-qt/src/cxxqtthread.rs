// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::{marker::PhantomData, mem::MaybeUninit, pin::Pin};
use cxx::ExternType;

use crate::Threading;

/// A threading helper which is created from a QObject that implements [Threading].
///
/// This allows for queueing closures onto the Qt event loop from a background thread
/// as [CxxQtThread] implements [Send].
#[repr(C)]
pub struct CxxQtThread<T>
where
    T: Threading,
{
    // The layout is two std::shared_ptr
    _space: MaybeUninit<[usize; 4]>,
    _value: PhantomData<T>,
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl<T> ExternType for CxxQtThread<T>
where
    T: ExternType + Threading,
{
    type Id = T::ThreadingTypeId;
    type Kind = cxx::kind::Trivial;
}

impl<T> Clone for CxxQtThread<T>
where
    T: Threading,
{
    fn clone(&self) -> Self {
        T::threading_clone(self)
    }
}

impl<T> Drop for CxxQtThread<T>
where
    T: Threading,
{
    fn drop(&mut self) {
        T::threading_drop(self);
    }
}

unsafe impl<T> Send for CxxQtThread<T> where T: Threading {}

impl<T> CxxQtThread<T>
where
    T: Threading,
{
    /// Queue the given closure onto the Qt event loop for this QObject
    pub fn queue<F>(&self, f: F) -> Result<(), cxx::Exception>
    where
        F: FnOnce(Pin<&mut T>),
        F: Send + 'static,
    {
        T::queue(self, f)
    }
}
