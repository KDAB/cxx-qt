use cxx_qt::make_qobject;

#[make_qobject]
mod my_object {
    #[derive(Default)]
    struct MyObject {
        number: i32,
        string: String,
    }

    impl MyObject {
        fn double_number(&self, number: i32) -> i32 {
            number * 2
        }

        fn say_hi(&self, string: &str, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }
    }
}
