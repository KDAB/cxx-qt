use cxx_qt::make_qobject;

#[make_qobject]
mod my_object {
    struct MyObject {}

    impl MyObject {
        fn say_hi(&self, string: String, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }
    }
}
