mod my_object {
    struct MyObject {
        number: i32,
    }

    impl Default for MyObject {
        fn default() -> Self {
            Self {
                number: 32,
            }
        }
    }

    impl MyObject {
        fn invokable(&self) {}
    }
}