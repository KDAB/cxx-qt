#!/usr/bin/
// Test for qtitem and qtfile, for code coverage
#[cxx_qt::bridge]
mod ffi {}

#[cxx::bridge]
mod ffi {}

struct MyStruct {
    name: &str,
    num: i32,
}
