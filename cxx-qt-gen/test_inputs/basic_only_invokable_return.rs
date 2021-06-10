mod my_object {
    struct MyObject {}

    impl MyObject {
        fn double_number(&self, number: i32) -> i32 {
            number * 2
        }

        fn hello_message(&self, msg: &str) -> String {
            format!("Hello {}", msg)
        }

        fn static_message(&self) -> &str {
            "Hello"
        }
    }
}