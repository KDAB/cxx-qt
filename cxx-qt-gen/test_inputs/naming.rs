#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[derive(Default)]
    pub struct Data {
        property_name: i32,
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    impl cxx_qt::QObject<MyObject> {
        #[invokable]
        pub fn invokable_name(&self) {
            println!("Bye from Rust!");
        }
    }
}
