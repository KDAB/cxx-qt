// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "signals")]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }

    // ANCHOR: book_signals_enum
    #[cxx_qt::signals(Signals)]
    pub enum Signal<'a> {
        Ready,
        RustDataChanged { data: i32 },
        TrivialDataChanged { trivial: QPoint },
        OpaqueDataChanged { opaque: QVariant },
        ReferenceDataChanged { reference: &'a QPoint },
    }
    // ANCHOR_END: book_signals_enum

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct Signals {
        #[qproperty]
        data: i32,
        #[qproperty]
        trivial: QPoint,
        #[qproperty]
        opaque: QVariant,
    }

    // ANCHOR: book_rust_obj_impl
    impl qobject::Signals {
        #[qinvokable]
        pub fn invokable(mut self: Pin<&mut Self>) {
            self.as_mut().emit(Signal::Ready);

            let signal = Signal::RustDataChanged { data: *self.data() };
            self.as_mut().emit(signal);
            let signal = Signal::TrivialDataChanged {
                trivial: self.trivial().clone(),
            };
            self.as_mut().emit(signal);
            let signal = Signal::OpaqueDataChanged {
                opaque: self.opaque().clone(),
            };
            self.as_mut().emit(signal);

            let point = QPoint::new(1, 2);
            self.as_mut()
                .emit(Signal::ReferenceDataChanged { reference: &point });
        }
    }
    // ANCHOR_END: book_rust_obj_impl
}
// ANCHOR_END: book_macro_code
