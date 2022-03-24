#[attrA]
#[make_qobject]
#[attrB]
pub mod my_object {
    use super::MyTrait;

    enum Event {
        MyEvent,
    }

    fn do_something() {
        println!("I am a free function");
    }

    #[derive(Default)]
    struct Data {
        number: i32,
    }

    impl MyTrait for Data {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn test_angled(&self, optional: Option<bool>) -> Option<bool> {
            optional
        }

        fn test_unknown(&self, unknown: MyType) -> MyType {
            unknown
        }
    }

    impl MyTrait for RustObj {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }
}
