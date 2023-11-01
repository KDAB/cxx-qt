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
///
/// When the Rust thread needs to update a value in the QObject it can then queue a closure to the thread.
/// This closure will be executed on the thread the QObject lives in while holding a lock on the Rust object.
/// Updating the QObject is then thread-safe.
///
/// See the [Threading] example for more information.
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
    ///
    /// The first argument of the closure is a pinned mutable reference to the QObject.
    /// With this parameter, you can then update the QObject to reflect any state changes that have occured in the background thread.
    pub fn queue<F>(&self, f: F) -> Result<(), cxx::Exception>
    where
        F: FnOnce(Pin<&mut T>),
        F: Send + 'static,
    {
        T::queue(self, f)
    }
}
