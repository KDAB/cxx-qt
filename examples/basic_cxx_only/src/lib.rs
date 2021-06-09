#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("main.h");

        fn get_cpp_number() -> i32;
    }

    extern "Rust" {
        fn get_numbers_sum() -> i32;
    }
}

fn get_numbers_sum() -> i32 {
    ffi::get_cpp_number() + 2
}
