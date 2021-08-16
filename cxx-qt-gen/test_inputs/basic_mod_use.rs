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

    struct MyObject {
        number: i32,
    }

    impl MyTrait for MyObject {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }

    impl MyObject {
        fn invokable(&self) {}
    }
}
