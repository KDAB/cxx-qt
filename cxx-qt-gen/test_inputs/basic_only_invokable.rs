mod my_object {
    use cxx_qt_lib::QString;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn say_hi(&self, string: &QString, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        #[invokable]
        fn say_bye(&self) {
            println!("Bye from Rust!");
        }

        fn plain_old_method(&self) {
            println!("QML can't call this :)");
        }
    }
}
