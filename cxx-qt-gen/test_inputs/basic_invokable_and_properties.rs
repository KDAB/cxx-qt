mod my_object {
    #[derive(Default)]
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

        fn say_bye(&self) {
            println!("Bye from Rust!");
        }
    }
}