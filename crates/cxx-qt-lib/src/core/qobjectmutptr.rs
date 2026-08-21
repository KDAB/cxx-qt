// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt::QObject;

/// A thin wrapper around `*mut QObject`.
/// Using the 'newtype' idiom lets us implement `ExternType`
/// for it, which wouldn't be possible for pure `*mut QObject` itself.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QObjectMutPtr(*mut QObject);

impl QObjectMutPtr {
    /// Create a new instance from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer is passed to C++ as a `QObject*` and may be dereferenced
    /// there. This wrapper tracks neither ownership nor lifetime,
    /// so the object must outlive every use of the underlying QObject pointer
    /// (including any copy stored in a `QVariant` or `QList`).
    pub unsafe fn from_raw(raw: *mut QObject) -> Self {
        Self(raw)
    }

    /// Return a wrapped raw const pointer.
    pub fn as_ptr(&self) -> *const QObject {
        self.0
    }

    /// Return a wrapped raw mut pointer.
    pub fn as_mut_ptr(&self) -> *mut QObject {
        self.0
    }

    /// Consume an object a return a wrapped raw mut pointer.
    pub fn into_raw(self) -> *mut QObject {
        self.0
    }
}

unsafe impl cxx::ExternType for QObjectMutPtr {
    type Id = cxx::type_id!("QObjectMutPtr");
    type Kind = cxx::kind::Trivial;
}
