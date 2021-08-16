mod my_object {
    #[derive(Default)]
    struct Data {
        my_number: i32,
    }

    struct RustObj {
        my_number: i32,
    }

    impl RustObj {
        fn say_bye(&self) {
            println!("Bye from Rust!");
        }
    }
}
