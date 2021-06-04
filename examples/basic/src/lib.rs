use cxx_qt::make_qobject;

// TODO: this is just a temporary thing to test if CMake is working,
// it should be removed once code generation has advanced to the point
// where we are producing cxx::bridge code
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("test.h");

        fn hello_from_cpp();
    }

    extern "Rust" {
        fn hello_from_rust();
    }
}

fn hello_from_rust() {
    println!("Hello from Rust!");
    ffi::hello_from_cpp();
}

#[make_qobject]
mod my_object {
    struct MyObject {
        name: String,
    }

    impl MyObject {
        fn say_hi(&self, string: &str, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }
    }
}
