mod my_object {
    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn say_hi(&self, _cpp: &mut CppObj, string: &str, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        #[invokable]
        fn say_bye(&self, _cpp: &mut CppObj) {
            println!("Bye from Rust!");
        }
    }
}
