// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
pub mod signals {
    use cxx_qt_lib::{QPoint, QVariant};

    // ANCHOR: book_signals_enum
    pub enum Signal {
        Ready,
        RustDataChanged { data: i32 },
        TrivialDataChanged { trivial: QPoint },
        OpaqueDataChanged { opaque: QVariant },
    }
    // ANCHOR_END: book_signals_enum

    #[derive(Default)]
    pub struct Data {
        data: i32,
        trivial: QPoint,
        opaque: QVariant,
    }

    #[derive(Default)]
    struct RustObj;

    // ANCHOR: book_rust_obj_impl
    impl RustObj {
        #[invokable]
        fn invokable(&self, cpp: &mut CppObj) {
            unsafe {
                cpp.emit_immediate(Signal::Ready);
            }

            cpp.emit_queued(Signal::RustDataChanged { data: cpp.data() });
            cpp.emit_queued(Signal::TrivialDataChanged {
                trivial: *cpp.trivial(),
            });
            cpp.emit_queued(Signal::OpaqueDataChanged {
                opaque: cpp.opaque(),
            });
        }
    }
    // ANCHOR_END: book_rust_obj_impl
}
// ANCHOR_END: book_macro_code
