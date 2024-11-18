// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::{marker::PhantomData, mem::MaybeUninit, pin::Pin};
use cxx::ExternType;
use thiserror::Error;

use crate::Threading;

/// Errors that can occur from CXX-Qt
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ThreadingQueueError {
    /// Threading failed as the object has been destroyed
    #[error("Cannot queue function pointer as object has been destroyed")]
    ObjectDestroyed,
    /// Threading failed calling invokeMethod on the object
    #[error("Cannot queue function pointer as invokeMethod on object failed")]
    InvokeMethodFailed,
    /// Threading failed with unknown error
    #[error("Cannot queue as an unknown error occurred")]
    Unknown,
}

impl From<u8> for ThreadingQueueError {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::ObjectDestroyed,
            2 => Self::InvokeMethodFailed,
            _others => Self::Unknown,
        }
    }
}

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
    // The layout is one std::shared_ptr, which is two pointers in size
    _space: MaybeUninit<[usize; 2]>,
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

// CxxQtThread is safe to be sent across threads and handles
// locking and checks with the original QObject to prevent issues
unsafe impl<T> Send for CxxQtThread<T> where T: Threading {}

// CxxQtThread is safe to use as a reference in parallel from multiple
// places as it protects the queue call and the closure with mutexes
unsafe impl<T> Sync for CxxQtThread<T> where T: Threading {}

impl<T> CxxQtThread<T>
where
    T: Threading,
{
    /// Queue the given closure onto the Qt event loop for this QObject
    ///
    /// The first argument of the closure is a pinned mutable reference to the QObject.
    /// With this parameter, you can then update the QObject to reflect any state changes that have occured in the background thread.
    pub fn queue<F>(&self, f: F) -> Result<(), crate::ThreadingQueueError>
    where
        F: FnOnce(Pin<&mut T>),
        F: Send + 'static,
    {
        T::queue(self, f)
    }

    /// Checks whether the associated `QObject` has been destroyed.
    ///
    /// This method only confirms if the `QObject` has already been destroyed.
    /// It does not guarantee that the `QObject` remains alive for any
    /// subsequent operations. There is a potential race condition when using
    /// `is_destroyed()` before calling `queue`. Specifically, the `QObject` may
    /// be destroyed after the check but before the `queue` call.
    ///
    /// For example:
    /// ```rust,ignore
    /// if !thread.is_destroyed() {
    ///     thread.queue(/*...*/).unwrap();
    /// }
    /// ```
    /// In this scenario, the `QObject` might be destroyed between the
    /// `is_destroyed` check and the `queue` invocation, resulting in a panic.
    ///
    /// To handle such cases safely, it is recommended to call `queue(...).ok()`
    /// directly without checking `is_destroyed()`. This approach allows you to
    /// handle the potential failure gracefully without risking a panic.
    ///
    /// However, `is_destroyed()` can still be useful in scenarios where you
    /// need to control loops or perform cleanup operations based on the
    /// destruction status of the `QObject`. For instance:
    /// ```rust,ignore
    /// while !thread.is_destroyed() {
    ///     thread.queue(/*...*/).ok();
    /// }
    /// ```
    pub fn is_destroyed(&self) -> bool {
        T::is_destroyed(self)
    }
}
