// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt::casting::Upcast;
pub use cxx_qt::QObject;
use std::pin::Pin;
use std::ptr;

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
    /// If `block` is `true`, signals emitted by this object are blocked (i.e., emitting a signal will not invoke anything connected to it). If `block` is `false`, no such blocking will occur.
    ///
    /// The return value is the previous value of [`signals_blocked`](Self::signals_blocked).
    ///
    /// Note that the [destroyed](https://doc.qt.io/qt/qobject.html#destroyed)() signal will be emitted even if the signals for this object have been blocked.
    ///
    /// Signals emitted while being blocked are not buffered.
    fn block_signals(self: Pin<&mut Self>, block: bool) -> bool;

    /// Returns `true` if signals are blocked; otherwise returns `false`.
    ///
    /// Signals are not blocked by default.
    fn signals_blocked(&self) -> bool;

    /// Sets the object's name to `name`.
    fn set_object_name(self: Pin<&mut Self>, name: &QString);

    /// Returns the name of this object.
    fn object_name(&self) -> QString;

    /// Returns a const pointer to the parent object.
    fn parent(&self) -> *const QObject;

    /// Makes the object a child of `parent`.
    fn set_parent(self: Pin<&mut Self>, parent: &Self);

    /// Makes the object parentless.
    fn unset_parent(self: Pin<&mut Self>);
}

/// Used to convert the QObject type from the library type to the C++ type, as a pin
fn cast_pin(obj: Pin<&mut QObject>) -> Pin<&mut QObjectExternal> {
    unsafe { Pin::new_unchecked(&mut *ptr::from_mut(obj.get_unchecked_mut()).cast()) }
}

/// Used to convert the QObject type from the library type to the C++ type
fn cast(obj: &QObject) -> &QObjectExternal {
    unsafe { &*(ptr::from_ref(obj).cast()) }
}

fn uncast(obj_extern: *mut QObjectExternal) -> *mut QObject {
    obj_extern.cast()
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

    fn parent(&self) -> *const QObject {
        uncast(cast(self.upcast()).parent())
    }

    fn set_parent(self: Pin<&mut Self>, parent: &Self) {
        let s = cast_pin(self.upcast_pin());
        unsafe {
            s.set_parent(cast(parent.upcast()) as *const QObjectExternal as *mut QObjectExternal)
        }
    }

    fn unset_parent(self: Pin<&mut Self>) {
        unsafe { cast_pin(self.upcast_pin()).set_parent(ptr::null_mut()) }
    }
}
