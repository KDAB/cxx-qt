// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod data;
mod types;

#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    pub struct MyObject {
        #[qproperty]
        number: i32,
        #[qproperty]
        string: QString,

        update_call_count: i32,
    }

    impl Default for MyObject {
        fn default() -> Self {
            Self {
                number: 0,
                string: QString::from(""),
                update_call_count: 0,
            }
        }
    }

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn double_number_self(self: Pin<&mut Self>) {
            let value = self.get_number() * 2;
            self.set_number(value);
        }

        #[qinvokable]
        pub fn double_number(&self, number: i32) -> i32 {
            number * 2
        }

        #[qinvokable]
        pub fn say_hi(&self, string: &QString, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        #[qinvokable]
        pub fn queue_test(self: Pin<&mut Self>) {
            let qt_thread = self.qt_thread();
            qt_thread
                .queue(|ctx| unsafe {
                    ctx.rust_mut().update_call_count += 1;
                })
                .unwrap();
        }

        #[qinvokable]
        pub fn queue_test_multi_thread(self: Pin<&mut Self>) {
            static N_THREADS: usize = 100;
            static N_REQUESTS: std::sync::atomic::AtomicUsize =
                std::sync::atomic::AtomicUsize::new(0);

            let mut handles = Vec::new();
            for _ in 0..N_THREADS {
                let qt_thread = self.qt_thread();
                handles.push(std::thread::spawn(move || {
                    // TODO: for now we use the unsafe rust_mut() API
                    // later there will be getters and setters for the properties
                    qt_thread
                        .queue(|ctx| unsafe {
                            ctx.rust_mut().update_call_count += 1;
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

        #[qinvokable]
        pub fn update_call_count(&self) -> i32 {
            self.rust().update_call_count
        }
    }
}
