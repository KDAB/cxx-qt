mod my_object {
    use cxx_qt_lib::QString;

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn double_number(&self, number: i32) -> i32 {
            number * 2
        }

        #[invokable]
        fn hello_message(&self, msg: &QString) -> String {
            format!("Hello {}", msg)
        }

        #[invokable]
        fn static_message(&self) -> &str {
            "Hello"
        }
    }
}
