#!/usr/bin/
// Test for qtitem and qtfile, for code coverage
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
