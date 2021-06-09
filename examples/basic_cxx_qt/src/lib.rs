use cxx_qt::make_qobject;

#[make_qobject]
mod my_object {
    struct MyObject {}

    impl MyObject {
        fn say_hi(&self) {
            println!("Hi from Rust!");
        }
    }
}
