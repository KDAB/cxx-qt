#!/usr/bin/
// Test for qtfile
#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {
        #[qobject]
        type MyObject = super::MyObjectRust;
    }

    unsafe extern "RustQt" {
        fn my_fn(self: &MyObject);
    }
}
