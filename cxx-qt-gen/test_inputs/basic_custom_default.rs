mod my_object {
    struct Data {
        number: i32,
    }

    impl Default for Data {
        fn default() -> Self {
            Self { number: 32 }
        }
    }

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn invokable(&self) {}
    }
}
