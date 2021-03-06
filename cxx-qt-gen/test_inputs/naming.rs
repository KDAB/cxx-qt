mod my_object {
    #[derive(Default)]
    pub struct Data {
        property_name: i32,
    }

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        #[invokable]
        pub fn invokable_name(&self) {
            println!("Bye from Rust!");
        }
    }
}
