#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[qenum(MyObject)]
    enum MyEnum {
        A,
    }

    #[qenum(MyObject)]
    enum MyOtherEnum {
        X,
        Y,
        Z,
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[derive(Default)]
        type MyObject = super::MyObjectRust;

        #[qinvokable]
        fn my_invokable(self: &MyObject, qenum: MyEnum, other_qenum: MyOtherEnum);
    }
}
