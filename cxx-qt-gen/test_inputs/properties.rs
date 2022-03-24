mod my_object {
    use cxx_qt_lib::Color;

    #[derive(Default)]
    struct Data {
        primitive: i32,
        opaque: Color,
        nested: crate::nested_object::CppObj,
    }

    #[derive(Default)]
    struct RustObj;
}
