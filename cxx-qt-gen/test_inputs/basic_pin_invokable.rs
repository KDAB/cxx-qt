mod my_object {
    struct MyObject {}

    impl MyObject {
        fn say_hi(&self, _cpp: Pin<&mut CppObj>, string: &str, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        fn say_bye(&self, _cpp: Pin<&mut CppObj>) {
            println!("Bye from Rust!");
        }
    }
}