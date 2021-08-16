mod my_object {
    #[derive(Default)]
    struct Data {
        obj: crate::sub_object::SubObject,
    }

    struct RustObj {
        obj: crate::sub_object::SubObject,
    }
}
