mod my_object {
    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn sub_test(&self, _cpp: Pin<&mut FFICppObj>, sub: Pin<&mut crate::sub_object::SubObject>) {
            println!("Bye from Rust!");
        }
    }
}
