// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QPoint = cxx_qt_lib::QPoint;
        type QVariant = cxx_qt_lib::QVariant;
    }

    // ANCHOR: book_signals_enum
    #[cxx_qt::signals(Signals)]
    pub enum Signal {
        Ready,
        RustDataChanged { data: i32 },
        TrivialDataChanged { trivial: QPoint },
        OpaqueDataChanged { opaque: UniquePtr<QVariant> },
    }
    // ANCHOR_END: book_signals_enum

    #[cxx_qt::qobject]
    pub struct Signals {
        #[qproperty]
        data: i32,
        #[qproperty]
        trivial: QPoint,
        #[qproperty(cxx_type = "QVariant")]
        opaque: UniquePtr<QVariant>,
    }

    impl Default for Signals {
        fn default() -> Self {
            Self {
                data: 0,
                trivial: QPoint::default(),
                opaque: QVariant::null(),
            }
        }
    }

    // ANCHOR: book_rust_obj_impl
    impl cxx_qt::QObject<Signals> {
        #[qinvokable]
        pub fn invokable(mut self: Pin<&mut Self>) {
            unsafe {
                self.as_mut().emit_immediate(Signal::Ready);
            }

            let signal = Signal::RustDataChanged {
                data: *self.get_data(),
            };
            self.as_mut().emit_queued(signal);
            let signal = Signal::TrivialDataChanged {
                trivial: self.get_trivial().clone(),
            };
            self.as_mut().emit_queued(signal);
            let signal = Signal::OpaqueDataChanged {
                opaque: QVariant::from_ref(self.get_opaque()),
            };
            self.as_mut().emit_queued(signal);
        }
    }
    // ANCHOR_END: book_rust_obj_impl
}
// ANCHOR_END: book_macro_code
