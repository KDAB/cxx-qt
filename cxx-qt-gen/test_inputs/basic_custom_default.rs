mod my_object {
    struct Data {
        number: i32,
    }

    impl Default for Data {
        fn default() -> Self {
            Self { number: 32 }
        }
    }

    struct MyObject {
        number: i32,
    }

    impl MyObject {
        fn invokable(&self) {}
    }
}
