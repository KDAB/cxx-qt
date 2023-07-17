// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod data;
mod empty;
mod locking;
mod types;

#[cxx_qt::bridge(cxx_file_stem = "my_object", namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[cxx_qt::qobject]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type MyObject = super::MyObjectRust;
    }

    // Enabling threading on the qobject
    impl cxx_qt::Threading for qobject::MyObject {}

    unsafe extern "RustQt" {
        #[qinvokable]
        fn double_number_self(self: Pin<&mut qobject::MyObject>);

        #[qinvokable]
        fn double_number(self: &qobject::MyObject, number: i32) -> i32;

        #[qinvokable]
        fn say_hi(self: &qobject::MyObject, string: &QString, number: i32);

        #[qinvokable]
        fn queue_test(self: Pin<&mut qobject::MyObject>);

        #[qinvokable]
        fn queue_test_multi_thread(self: Pin<&mut qobject::MyObject>);

        #[qinvokable]
        fn fetch_update_call_count(self: &qobject::MyObject) -> i32;
    }
}

use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;

pub struct MyObjectRust {
    number: i32,
    string: QString,

    pub(crate) update_call_count: i32,
}

impl Default for MyObjectRust {
    fn default() -> Self {
        Self {
            number: 0,
            string: QString::from(""),
            update_call_count: 0,
        }
    }
}

// TODO: this will change to qobject::MyObject once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::MyObject {
    fn double_number_self(self: Pin<&mut Self>) {
        let value = self.number() * 2;
        self.set_number(value);
    }

    fn double_number(&self, number: i32) -> i32 {
        number * 2
    }

    fn say_hi(&self, string: &QString, number: i32) {
        println!("Hi from Rust! String is {string} and number is {number}");
    }

    fn queue_test(self: Pin<&mut Self>) {
        let qt_thread = self.qt_thread();
        qt_thread
            .queue(|qobject| {
                qobject.rust_mut().update_call_count += 1;
            })
            .unwrap();
    }

    fn queue_test_multi_thread(self: Pin<&mut Self>) {
        static N_THREADS: usize = 100;
        static N_REQUESTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

        let mut handles = Vec::new();
        let qt_thread = self.qt_thread();
        for _ in 0..N_THREADS {
            let qt_thread_cloned = qt_thread.clone();
            handles.push(std::thread::spawn(move || {
                qt_thread_cloned
                    .queue(|qobject| {
                        qobject.rust_mut().update_call_count += 1;
                    })
                    .unwrap();
                N_REQUESTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // Make sure we actually ran all the threads
        assert_eq!(
            N_REQUESTS.load(std::sync::atomic::Ordering::Relaxed),
            N_THREADS
        );
    }

    fn fetch_update_call_count(&self) -> i32 {
        self.rust().update_call_count
    }
}
