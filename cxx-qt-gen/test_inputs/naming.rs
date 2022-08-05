#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    #[derive(Default)]
    pub struct Data {
        property_name: i32,
    }

    #[derive(Default)]
    pub struct RustObj;

    impl cxx_qt::QObject<RustObj> {
        #[invokable]
        pub fn invokable_name(&self) {
            println!("Bye from Rust!");
        }
    }
}
