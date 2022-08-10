#[cxx_qt::bridge]
mod ffi {
    #[derive(Default)]
    pub struct Data {
        property_name: i32,
    }

    unsafe extern "C++" {
        include!(<QtCore/QStringListModel>);
    }

    #[cxx_qt::qobject(base = "QStringListModel")]
    #[derive(Default)]
    pub struct MyObject;

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn invokable_name(self: Pin<&mut Self>) {
            println!("Bye from Rust!");
            self.as_mut().set_property_name(5);
        }
    }
}
