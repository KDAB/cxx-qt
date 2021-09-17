mod my_object {
    use super::MyTrait;

    enum Event {
        MyEvent,
    }

    fn do_something() {
        println!("I am a free function");
    }

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

    impl MyTrait for RustObj {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }

    impl RustObj {
        #[invokable]
        fn invokable(&self) {}
    }
}
