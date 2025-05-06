// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub use cxx_qt::QObject;
use std::pin::Pin;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qobject.h");
        #[rust_name = "QObjectExternal"]
        type QObject;
        #[rust_name = "block_signals"]
        pub fn blockSignals(self: Pin<&mut Self>, block: bool) -> bool;

        #[rust_name = "signals_blocked"]
        pub fn signalsBlocked(&self) -> bool;

        #[rust_name = "set_object_name"]
        pub fn setObjectName(self: Pin<&mut Self>, name: &QString);

        #[rust_name = "object_name"]
        pub fn objectName(&self) -> QString;

        pub fn parent(&self) -> *mut QObjectExternal;

        #[rust_name = "set_parent"]
        pub unsafe fn setParent(self: Pin<&mut Self>, parent: *mut QObjectExternal);
    }
}

use ffi::{QObjectExternal, QString};

/// Trait which exposes methods available on a `QObject`.
/// Exposes some basic signals and methods for now, more to be added.
pub trait QObjectExt {
    fn block_signals(self: Pin<&mut Self>, block: bool) -> bool;

    fn signals_blocked(&self) -> bool;

    fn set_object_name(self: Pin<&mut Self>, name: &QString);

    fn object_name(&self) -> QString;

    fn parent(&self) -> *mut Self;

    fn set_parent(self: Pin<&mut Self>, parent: &Self);
}

/// Used to convert the QObject type from the library type to the C++ type, as a pin
fn cast_pin(obj: Pin<&mut QObject>) -> Pin<&mut QObjectExternal> {
    unsafe {
        let mut_ptr = obj.get_unchecked_mut() as *mut QObject as *mut QObjectExternal;
        Pin::new_unchecked(&mut *mut_ptr)
    }
}

/// Used to convert the QObject type from the library type to the C++ type
fn cast(obj: &QObject) -> &QObjectExternal {
    unsafe {
        let ptr = obj as *const QObject as *const QObjectExternal;
        &*ptr
    }
}

impl QObjectExt for QObject {
    fn block_signals(self: Pin<&mut Self>, block: bool) -> bool {
        cast_pin(self).block_signals(block)
    }

    fn signals_blocked(&self) -> bool {
        cast(self).signals_blocked()
    }

    fn set_object_name(self: Pin<&mut Self>, name: &QString) {
        cast_pin(self).set_object_name(name)
    }

    fn object_name(&self) -> QString {
        cast(self).object_name()
    }

    fn parent(&self) -> *mut Self {
        cast(self).parent() as *mut Self
    }

    fn set_parent(self: Pin<&mut Self>, parent: &Self) {
        unsafe {
            cast_pin(self)
                .set_parent(cast(parent) as *const QObjectExternal as *mut QObjectExternal);
        }
    }
}
