// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt::casting::Upcast;
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

        #[rust_name = "dump_object_info"]
        fn dumpObjectInfo(&self);
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

    fn parent(&self) -> *mut QObjectExternal;

    fn set_parent(self: Pin<&mut Self>, parent: &Self);

    fn dump_object_info(&self);
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

impl<T> QObjectExt for T
where
    T: Upcast<QObject>,
{
    fn block_signals(self: Pin<&mut Self>, block: bool) -> bool {
        cast_pin(self.upcast_pin()).block_signals(block)
    }

    fn signals_blocked(&self) -> bool {
        cast(self.upcast()).signals_blocked()
    }

    fn set_object_name(self: Pin<&mut Self>, name: &QString) {
        cast_pin(self.upcast_pin()).set_object_name(name)
    }

    fn object_name(&self) -> QString {
        cast(self.upcast()).object_name()
    }

    fn parent(&self) -> *mut QObjectExternal {
        cast(self.upcast()).parent()
    }

    fn set_parent(self: Pin<&mut Self>, parent: &Self) {
        let s = cast_pin(self.upcast_pin());
        unsafe {
            s.set_parent(cast(parent.upcast()) as *const QObjectExternal as *mut QObjectExternal)
        }
    }

    fn dump_object_info(&self) {
        cast(self.upcast()).dump_object_info()
    }
}
