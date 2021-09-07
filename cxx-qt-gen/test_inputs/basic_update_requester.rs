mod my_object {
    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn say_hi(&self, string: &QString, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        fn say_bye(&self) {
            println!("Bye from Rust!");
        }
    }

    impl UpdateRequestHandler<CppObj> for RustObj {
        fn handle_update_request(&mut self, _cpp: Pin<&mut CppObj>) {
            println!("update")
        }
    }
}
