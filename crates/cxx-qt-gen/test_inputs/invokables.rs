#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject;

    unsafe extern "RustQt" {
        #[qinvokable]
        fn invokable(self: &qobject::MyObject);

        #[qinvokable]
        fn invokable_mutable(self: Pin<&mut qobject::MyObject>);

        #[qinvokable]
        fn invokable_parameters(
            self: &qobject::MyObject,
            opaque: &QColor,
            trivial: &QPoint,
            primitive: i32,
        );

        #[qinvokable]
        fn invokable_return_opaque(self: Pin<&mut qobject::MyObject>) -> UniquePtr<Opaque>;

        #[qinvokable]
        fn invokable_return_trivial(self: Pin<&mut qobject::MyObject>) -> QPoint;

        #[qinvokable(cxx_final)]
        fn invokable_final(self: &qobject::MyObject);

        #[qinvokable(cxx_override)]
        fn invokable_override(self: &qobject::MyObject);

        #[qinvokable(cxx_virtual)]
        fn invokable_virtual(self: &qobject::MyObject);
    }

    impl qobject::MyObject {
        pub fn invokable(&self) {
            println!("invokable");
        }

        pub fn invokable_mutable(self: Pin<&mut Self>) {
            println!("This method is mutable!");
        }

        pub fn invokable_parameters(&self, opaque: &QColor, trivial: &QPoint, primitive: i32) {
            println!(
                "Red: {}, Point X: {}, Number: {}",
                opaque.red(),
                trivial.x(),
                primitive,
            );
        }

        pub fn invokable_return_opaque(self: Pin<&mut Self>) -> UniquePtr<Opaque> {
            Opaque::new()
        }

        pub fn invokable_return_trivial(self: Pin<&mut Self>) -> QPoint {
            QPoint::new(1, 2)
        }

        pub fn invokable_final(&self) {
            println!("Final");
        }

        pub fn invokable_override(&self) {
            println!("Override");
        }

        pub fn invokable_virtual(&self) {
            println!("Virtual");
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

    impl cxx_qt::Threading for qobject::MyObject {}
}
