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
mod qobject {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        type MyObject = super::MyObjectRust;
    }

    // Enabling threading on the qobject
    impl cxx_qt::Threading for MyObject {}

    // Note that we are only testing with C++ here so we don't need qinvokable
    unsafe extern "RustQt" {
        #[cxx_name = "doubleNumberSelf"]
        fn double_number_self(self: Pin<&mut MyObject>);

        #[cxx_name = "doubleNumber"]
        fn double_number(self: &MyObject, number: i32) -> i32;

        #[cxx_name = "sayHi"]
        fn say_hi(self: &MyObject, string: &QString, number: i32);

        #[cxx_name = "queueTest"]
        fn queue_test(self: Pin<&mut MyObject>);

        #[cxx_name = "queueTestMultiThread"]
        fn queue_test_multi_thread(self: Pin<&mut MyObject>);

        #[cxx_name = "fetchUpdateCallCount"]
        fn fetch_update_call_count(self: &MyObject) -> i32;

        #[cxx_name = "throwException"]
        fn throw_exception(self: &MyObject) -> Result<i32>;
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

impl qobject::MyObject {
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
        self.update_call_count
    }

    fn throw_exception(&self) -> Result<i32, String> {
        Err("RustException".to_string())
    }
}
