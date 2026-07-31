// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt::QObject;

/// A thin wrapper around `*mut QObject`.
/// Using the 'newtype' idiom lets us implement `ExternType`
/// for it, which wouldn't possible for pure `*mut QObject` itself.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QObjectMutPtr(pub *mut QObject);

impl From<*mut QObject> for QObjectMutPtr {
    fn from(value: *mut QObject) -> Self {
        QObjectMutPtr(value)
    }
}

impl From<QObjectMutPtr> for *mut QObject {
    fn from(value: QObjectMutPtr) -> Self {
        value.0
    }
}

unsafe impl cxx::ExternType for QObjectMutPtr {
    type Id = cxx::type_id!("QObjectMutPtr");
    type Kind = cxx::kind::Trivial;
}
