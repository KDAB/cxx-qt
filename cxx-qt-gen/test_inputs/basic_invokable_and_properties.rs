mod my_object {
    struct MyObject {
        string: String,
        number: i32,
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