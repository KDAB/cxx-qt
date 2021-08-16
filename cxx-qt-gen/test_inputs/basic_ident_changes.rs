mod my_object {
    #[derive(Default)]
    struct Data {
        my_number: i32,
    }

    struct MyObject {
        my_number: i32,
    }

    impl MyObject {
        fn say_bye(&self) {
            println!("Bye from Rust!");
        }
    }
}
