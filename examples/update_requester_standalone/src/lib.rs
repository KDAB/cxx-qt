// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::update_requester::{CxxQObject, UpdateRequester};
use once_cell::sync::OnceCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

static UPDATE_REQUESTER: OnceCell<UpdateRequester> = OnceCell::new();

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("TestObject.h");

        type TestObject;
    }

    extern "Rust" {
        unsafe fn init_rust(ptr: *mut TestObject);
        fn request_update();
        fn request_on_multiple_threads();
    }
}

unsafe fn init_rust(ptr: *mut ffi::TestObject) {
    UPDATE_REQUESTER
        .set(UpdateRequester::new(ptr as *mut CxxQObject))
        .unwrap();
}

fn request_update() {
    UPDATE_REQUESTER.get().unwrap().request_update();
}

fn request_on_multiple_threads() {
    static N_THREADS: usize = 100;
    static N_REQUESTS: AtomicUsize = AtomicUsize::new(0);

    N_REQUESTS.store(0, Ordering::Relaxed);
    let mut handles = Vec::new();

    for _ in 0..N_THREADS {
        handles.push(thread::spawn(|| {
            request_update();
            N_REQUESTS.fetch_add(1, Ordering::Relaxed);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Make sure we actually ran all the threads
    assert_eq!(N_REQUESTS.load(Ordering::Relaxed), N_THREADS);
}
