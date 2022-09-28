#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/include/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    impl cxx_qt::QObject<MyObject> {
        #[qinvokable]
        pub fn invokable(&self) {
            println!("invokable");
        }

        #[qinvokable]
        pub fn invokable_mutable(self: Pin<&mut Self>) {
            println!("This method is mutable!");
        }

        #[qinvokable]
        pub fn invokable_parameters(&self, opaque: &QColor, trivial: &QPoint, primitive: i32) {
            println!(
                "Red: {}, Point X: {}, Number: {}",
                opaque.red(),
                trivial.x(),
                primitive,
            );
        }

        // Value and Opaque are not real types that would compile; these are only testing the code generation
        #[qinvokable(return_cxx_type = "Value")]
        pub fn invokable_return_opaque(self: Pin<&mut Self>) -> UniquePtr<Opaque> {
            Opaque::new()
        }

        #[qinvokable]
        pub fn invokable_return_trivial(self: Pin<&mut Self>) -> QPoint {
            QPoint::new(1, 2)
        }

        pub fn cpp_context_method(&self) {
            println!("C++ context method");
        }

        pub fn cpp_context_method_mutable(self: Pin<&mut Self>) {
            println!("mutable method");
        }

        pub fn cpp_context_method_return_opaque(&self) -> UniquePtr<Opaque> {
            Opaque::new()
        }
    }

    impl MyObject {
        pub fn rust_only_method(&self) {
            println!("QML or C++ can't call this :)");
        }
    }
}
