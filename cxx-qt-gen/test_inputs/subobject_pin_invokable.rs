mod my_object {
    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn sub_test(&self, _cpp: &mut CppObj, sub: &mut crate::sub_object::CppObj) {
            println!("Bye from Rust!");
        }
    }
}
