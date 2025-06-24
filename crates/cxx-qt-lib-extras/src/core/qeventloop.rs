// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::pin::Pin;
use std::time::Duration;

use cxx::{type_id, UniquePtr};
use cxx_qt_lib::{QFlag, QFlags};

#[cxx_qt::bridge]
mod ffi {
    /// This enum controls the types of events processed by [`QEventLoop::process_filtered_events`].
    #[repr(i32)]
    enum QEventLoopProcessEventsFlag {
        /// All events. Note that [`QEventType::DeferredDelete`](https://doc.qt.io/qt-6/qevent.html#Type-enum) events are processed specially. See [`QObject::delete_later`](https://doc.qt.io/qt-6/qobject.html#deleteLater) for more details.
        AllEvents = 0x00,
        /// Do not process user input events, such as [QEventType::MouseButtonPress](https://doc.qt.io/qt-6/qevent.html#Type-enum) and [QEventType::KeyPress](https://doc.qt.io/qt-6/qevent.html#Type-enum). Note that the events are not discarded; they will be delivered the next time [`QEventLoop::process_filtered_events`] is called without this flag.
        ExcludeUserInputEvents = 0x01,
        /// Do not process socket notifier events. Note that the events are not discarded; they will be delivered the next time [`QEventLoop::process_filtered_events`] is called without this flag.
        ExcludeSocketNotifiers = 0x02,
        /// Wait for events if no pending events are available.
        WaitForMoreEvents = 0x04,
    }

    extern "C++" {
        include!("cxx-qt-lib-extras/qeventloop.h");
        type QEventLoopProcessEventsFlag;
        type QEventLoopProcessEventsFlags = super::QEventLoopProcessEventsFlags;
    }

    extern "Rust" {
        type EventLoopClosure<'a>;
    }

    unsafe extern "C++Qt" {
        /// The `QEventLoop` class provides a means of entering and leaving an event loop.
        ///
        /// Qt Documentation: [QEventLoop](https://doc.qt.io/qt-6/qeventloop.html#details)
        #[qobject]
        type QEventLoop;

        /// Enters the main event loop and waits until [`exit`](QEventLoop::exit) is called. Returns the value that was passed to [`exit`](QEventLoop::exit).
        ///
        /// Only events of the types allowed by `flags` will be processed.
        ///
        /// It is necessary to call this function to start event handling. The main event loop receives events from the window system and dispatches these to the application widgets.
        ///
        /// Generally speaking, no user interaction can take place before calling this function. As a special case, modal widgets like [`QMessageBox`](https://doc.qt.io/qt-6/qmessagebox.html) can be used before calling this function, because modal widgets use their own local event loop.
        ///
        /// To make your application perform idle processing (i.e. executing a special function whenever there are no pending events), use a [`QChronoTimer`](https://doc.qt.io/qt-6/qchronotimer.html) with 0ns timeout. More sophisticated idle processing schemes can be achieved using [`process_events`](QEventLoop::process_events).
        fn exec(self: Pin<&mut QEventLoop>, flags: QEventLoopProcessEventsFlags) -> i32;

        /// Tells the event loop to exit with a return code.
        ///
        /// After this function has been called, the event loop returns from the call to [`exec`](QeventLoop::exec) or [`exec_all`](QeventLoop::exec_all). The call returns `return_code`.
        ///
        /// By convention, a `return_code` of 0 means success, and any non-zero value indicates an error.
        ///
        /// Note that unlike the C library function of the same name, this function does return to the caller – it is event processing that stops.
        fn exit(self: Pin<&mut QEventLoop>, return_code: i32);

        /// Processes some pending events that match `flags`. Returns `true` if pending events were handled; otherwise returns `false`.
        ///
        /// This function is especially useful if you have a long running operation and want to show its progress without allowing user input; i.e. by using the [`QEventLoopProcessEventsFlag::ExcludeUserInputEvents`] flag.
        ///
        /// This function is simply a wrapper for [QAbstractEventDispatcher::process_events`](https://doc.qt.io/qt-6/qabstracteventdispatcher.html#processEvents). See the documentation for that function for details.
        #[rust_name = "process_events"]
        fn processEvents(self: Pin<&mut QEventLoop>, flags: QEventLoopProcessEventsFlags) -> bool;

        #[doc(hidden)]
        #[rust_name = "process_events_until_msecs"]
        fn processEvents(
            self: Pin<&mut QEventLoop>,
            flags: QEventLoopProcessEventsFlags,
            max_time: i32,
        );

        /// Tells the event loop to exit normally.
        ///
        /// Same as [`self.exit(0)`](QEventLoop::exit).
        fn quit(self: Pin<&mut QEventLoop>);

        /// Wakes up the event loop.
        #[rust_name = "wake_up"]
        fn wakeUp(self: Pin<&mut QEventLoop>);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[allow(clippy::needless_lifetimes)]
        #[doc(hidden)]
        #[rust_name = "qeventloop_exec_with"]
        fn qeventloopExecWith<'a>(
            event_loop: Pin<&mut QEventLoop>,
            context: &mut EventLoopClosure<'a>,
            functor: fn(&mut EventLoopClosure<'a>),
        ) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qeventloop_init_default"]
        fn make_unique() -> UniquePtr<QEventLoop>;
    }
}

pub use ffi::{QEventLoop, QEventLoopProcessEventsFlag};

/// [`QFlags`] of [`QEventLoopProcessEventsFlag`].
pub type QEventLoopProcessEventsFlags = QFlags<QEventLoopProcessEventsFlag>;

unsafe impl QFlag for QEventLoopProcessEventsFlag {
    type TypeId = type_id!("QEventLoopProcessEventsFlags");

    type Repr = i32;

    fn to_repr(self) -> Self::Repr {
        self.repr
    }
}

impl QEventLoop {
    /// Constructs an event loop object.
    pub fn new() -> UniquePtr<Self> {
        ffi::qeventloop_init_default()
    }

    /// Enters the main event loop and waits until [`exit`](QEventLoop::exit) is called. Returns the value that was passed to [`exit`](QEventLoop::exit).
    ///
    /// It is necessary to call this function to start event handling. The main event loop receives events from the window system and dispatches these to the application widgets.
    ///
    /// Generally speaking, no user interaction can take place before calling this function. As a special case, modal widgets like [`QMessageBox`](https://doc.qt.io/qt-6/qmessagebox.html) can be used before calling this function, because modal widgets use their own local event loop.
    ///
    /// To make your application perform idle processing (i.e. executing a special function whenever there are no pending events), use a [`QChronoTimer`](https://doc.qt.io/qt-6/qchronotimer.html) with 0ns timeout. More sophisticated idle processing schemes can be achieved using [`process_all_events`](QEventLoop::process_all_events).
    pub fn exec_all(self: Pin<&mut Self>) -> i32 {
        self.exec(QEventLoopProcessEventsFlag::AllEvents.into())
    }

    /// Enters an event loop, runs a `closure`, and exits the event loop when the closure completes.
    ///
    /// As with `QEventLoop`'s other methods, a [`QApplication`](crate::QApplication), [`QGuiApplication`](cxx_qt_lib::QGuiApplication), or [`QCoreApplication`](cxx_qt_lib::QCoreApplication) must be running.
    pub fn exec_with<F>(self: Pin<&mut QEventLoop>, closure: F)
    where
        F: FnOnce(),
    {
        let mut closure = EventLoopClosure {
            closure: Some(Box::new(closure)),
        };
        ffi::qeventloop_exec_with(self, &mut closure, EventLoopClosure::run);
    }

    /// Processes some pending events. Returns `true` if pending events were handled; otherwise returns `false`.
    ///
    /// This function is simply a wrapper for [`QAbstractEventDispatcher::process_events`](https://doc.qt.io/qt-6/qabstracteventdispatcher.html#processEvents). See the documentation for that function for details.
    pub fn process_all_events(self: Pin<&mut QEventLoop>) -> bool {
        self.process_events(QEventLoopProcessEventsFlag::AllEvents.into())
    }

    /// Process pending events that match `flags` until `deadline` has expired, or until there are no more events to process, whichever happens first. This function is especially useful if you have a long running operation and want to show its progress without allowing user input, i.e. by using the [`QEventLoopProcessEventsFlag::ExcludeUserInputEvents`] flag.
    ///
    /// **Notes:**
    ///
    /// * This function does not process events continuously; it returns after all available events are processed.
    /// * Specifying the [`QEventLoopProcessEventsFlag::WaitForMoreEvents`] flag makes no sense and will be ignored.
    pub fn process_events_until(
        self: Pin<&mut QEventLoop>,
        flags: QEventLoopProcessEventsFlags,
        deadline: Duration,
    ) {
        self.process_events_until_msecs(
            flags,
            i32::try_from(deadline.as_millis()).unwrap_or(i32::MAX),
        );
    }

    /// Process pending events until `deadline` has expired, or until there are no more events to process, whichever happens first.
    ///
    /// **Note:** This function does not process events continuously; it returns after all available events are processed.
    pub fn process_all_events_until<T>(self: Pin<&mut QEventLoop>, deadline: Duration) {
        self.process_events_until(QEventLoopProcessEventsFlag::AllEvents.into(), deadline);
    }
}

struct EventLoopClosure<'a> {
    closure: Option<Box<dyn FnOnce() + 'a>>,
}

impl<'a> EventLoopClosure<'a> {
    pub fn run(&mut self) {
        self.closure.take().unwrap()();
    }
}

#[cfg(test)]
mod tests {
    use cxx_qt_lib::QCoreApplication;

    use super::QEventLoop;

    #[test]
    fn qeventloop_exec_with() {
        std::mem::forget(QCoreApplication::new()); // cargo test may randomly segfault if app is dropped
        let mut increment_count = 0;
        QEventLoop::new().pin_mut().exec_with(|| {
            increment_count += 1;
        });
        assert_eq!(increment_count, 1);
    }
}
