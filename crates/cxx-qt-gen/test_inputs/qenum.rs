#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[qenum(MyObject)]
    enum MyEnum {
        A,
    }

    // Associated QEnums can be namespaced independently
    #[qenum(MyObject)]
    #[namespace = "my_namespace"]
    enum MyOtherEnum {
        X,
        Y,
        Z,
    }

    #[qml_element]
    qnamespace!("cxx_qt::my_object");

    #[qenum]
    enum MyNamespacedEnum {
        A,
        B,
        C,
    }

    qnamespace!("other_namespace");

    #[qenum]
    #[namespace = "other_namespace"]
    enum MyOtherNamespacedEnum {
        Variant1,
        Variant2,
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[derive(Default)]
        type MyObject = super::MyObjectRust;

        #[qinvokable]
        fn my_invokable(self: &MyObject, qenum: MyEnum, other_qenum: MyOtherEnum);
    }

    // Test that we can correctly associate a QEnum to a renamed QObject
    #[qenum(MyRenamedObject)]
    enum MyRenamedEnum {
        A,
        B,
        C,
    }

    extern "RustQt" {
        #[qobject]
        #[rust_name = "MyRenamedObject"]
        type CxxName = super::InternalObject;
    }
}
