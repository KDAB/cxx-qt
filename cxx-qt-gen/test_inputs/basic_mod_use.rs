mod my_object {
    use super::MyTrait;

    struct Data {
        number: i32,
    }

    impl Default for Data {
        fn default() -> Self {
            Self { number: 32 }
        }
    }

    struct RustObj {
        number: i32,
    }

    impl MyTrait for RustObj {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }

    impl RustObj {
        fn invokable(&self) {}
    }
}
