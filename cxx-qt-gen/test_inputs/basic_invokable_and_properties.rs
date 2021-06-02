mod my_object {
    struct MyObject {
        number: i32,
        string: String,
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