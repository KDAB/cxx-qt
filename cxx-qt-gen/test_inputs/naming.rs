#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!(<QtCore/QStringListModel>);
    }

    #[cxx_qt::qobject(base = "QStringListModel")]
    #[derive(Default)]
    pub struct MyObject {
        #[qproperty]
        property_name: i32,
    }

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn invokable_name(self: Pin<&mut Self>) {
            println!("Bye from Rust!");
            self.as_mut().set_property_name(5);
        }
    }
}
