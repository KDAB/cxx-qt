mod my_object {
    #[derive(Default)]
    struct Data {
        obj: crate::sub_object::SubObject,
    }

    struct MyObject {
        obj: crate::sub_object::SubObject,
    }
}
