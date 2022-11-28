// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge(cxx_file_stem = "nested_qobjects")]
mod ffi {
    // ANCHOR: book_extern_block
    unsafe extern "C++" {
        #[cxx_name = "InnerObject"]
        type CxxInnerObject = super::qobject::InnerObject;
    }
    // ANCHOR_END: book_extern_block

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct InnerObject {
        #[qproperty]
        counter: i32,
    }

    #[cxx_qt::qobject]
    pub struct OuterObject {
        #[qproperty]
        inner: *mut CxxInnerObject,
    }

    impl Default for OuterObject {
        fn default() -> Self {
            Self {
                inner: std::ptr::null_mut(),
            }
        }
    }

    #[cxx_qt::qsignals(OuterObject)]
    pub enum OuterSignals {
        Called { inner: *mut CxxInnerObject },
    }

    impl qobject::OuterObject {
        #[qinvokable]
        pub fn print_count(self: Pin<&mut Self>, inner: *mut CxxInnerObject) {
            if let Some(inner) = unsafe { inner.as_ref() } {
                println!("Inner object's counter property: {}", inner.counter());
            }

            self.emit(OuterSignals::Called { inner });
        }

        #[qinvokable]
        pub fn reset(self: Pin<&mut Self>) {
            // We need to convert the *mut T to a Pin<&mut T> so that we can reach the methods
            if let Some(inner) = unsafe { self.inner().as_mut() } {
                // TODO: Use `pin!` one it's stable so that this unsafe block can be removed
                // https://doc.rust-lang.org/std/pin/macro.pin.html
                let pinned_inner = unsafe { Pin::new_unchecked(inner) };
                // Now pinned inner can be used as normal
                pinned_inner.set_counter(10);
            }

            // Retrieve *mut T
            let inner = *self.inner();
            self.emit(OuterSignals::Called { inner });
        }
    }
}
// ANCHOR_END: book_macro_code
