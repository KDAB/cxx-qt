mod my_object {
    struct RustObj;

    impl RustObj {
        fn sub_test(&self, _cpp: Pin<&mut CppObj>, sub: Pin<&mut crate::sub_object::SubObject>) {
            println!("Bye from Rust!");
        }
    }
}
