#!/usr/bin/
// Test for qtitem and qtfile, for code coverage
#[cxx_qt::bridge(cxx_file_stem = "shebang")]
mod ffi {
    extern "RustQt" {
        #[qobject]
        type MyObject = super::MyObjectRust;
    }

    unsafe extern "RustQt" {
        fn my_fn(self: &MyObject);
    }
}
