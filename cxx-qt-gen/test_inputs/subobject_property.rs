mod my_object {
    #[derive(Default)]
    struct Data {
        obj: crate::sub_object::CppObj,
    }

    #[derive(Default)]
    struct RustObj;
}
