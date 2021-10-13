mod my_object {
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
}
