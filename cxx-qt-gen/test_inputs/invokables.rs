mod my_object {
    use cxx_qt_lib::QColor;

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        #[invokable]
        pub fn invokable(&self) {
            println!("invokable");
        }

        #[invokable]
        pub fn invokable_cpp_obj(&self, cpp: &mut CppObj) {
            println!("cppobj");
        }

        #[invokable]
        pub fn invokable_mutable(&mut self) {
            println!("This method is mutable!");
        }

        #[invokable]
        pub fn invokable_mutable_cpp_obj(&mut self, cpp: &mut CppObj) {
            println!("This method is mutable!");
        }

        #[invokable]
        pub fn invokable_nested_parameter(&self, nested: &mut crate::cxx_qt_nested_object::CppObj) {
            println!("nested!");
        }

        #[invokable]
        pub fn invokable_parameters(&self, opaque: &QColor, trivial: &QPoint, primitive: i32) {
            println!(
                "Red: {}, Point X: {}, Number: {}",
                opaque.red(),
                trivial.x(),
                primitive,
            );
        }

        #[invokable]
        pub fn invokable_parameters_cpp_obj(&self, primitive: i32, cpp: &mut CppObj) {
            println!("{}", primitive);
        }

        #[invokable]
        pub fn invokable_return_opaque(&mut self) -> UniquePtr<QColor> {
            cxx_qt_lib::QColor::from_rgba(255, 0, 0, 0)
        }

        #[invokable]
        pub fn invokable_return_primitive(&mut self) -> i32 {
            2
        }

        #[invokable]
        pub fn invokable_return_static(&mut self) -> UniquePtr<QString> {
            QString::from_str("static")
        }

        pub fn rust_only_method(&self) {
            println!("QML can't call this :)");
        }
    }
}
