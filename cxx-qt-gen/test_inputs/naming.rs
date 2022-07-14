mod my_object {
    extern "Qt" {
        #[derive(Default)]
        struct Data {
            property_name: i32,
        }

        #[derive(Default)]
        struct RustObj;

        impl RustObj {
            #[invokable]
            fn invokable_name(&self) {
                println!("Bye from Rust!");
            }
        }
    }
}
