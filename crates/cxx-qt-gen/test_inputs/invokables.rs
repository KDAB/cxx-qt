#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
        include!(<QtCore/QObject>);
        type QObject;
    }

    unsafe extern "RustQt" {
        #[qobject]
        type MyObject = super::MyObjectRust;

        #[cxx_name = "cppMethod"]
        fn cpp_method(self: &MyObject);

        #[qinvokable]
        fn invokable(self: &MyObject);

        #[qinvokable]
        #[cxx_name = "invokableMutable"]
        fn invokable_mutable(self: Pin<&mut MyObject>);

        #[qinvokable]
        #[cxx_name = "invokableParameters"]
        fn invokable_parameters(self: &MyObject, opaque: &QColor, trivial: &QPoint, primitive: i32);

        #[qinvokable]
        #[cxx_name = "invokableReturnOpaque"]
        fn invokable_return_opaque(self: Pin<&mut MyObject>) -> UniquePtr<Opaque>;

        #[qinvokable]
        #[cxx_name = "invokableReturnTrivial"]
        fn invokable_return_trivial(self: Pin<&mut MyObject>) -> QPoint;

        #[qinvokable]
        #[cxx_final]
        #[cxx_name = "invokableFinal"]
        fn invokable_final(self: &MyObject);

        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "invokableOverride"]
        fn invokable_override(self: &MyObject);

        #[qinvokable]
        #[cxx_virtual]
        #[cxx_name = "invokableVirtual"]
        fn invokable_virtual(self: &MyObject);

        #[qinvokable]
        #[cxx_name = "invokableResultTuple"]
        fn invokable_result_tuple(self: &MyObject) -> Result<()>;

        #[qinvokable]
        #[cxx_name = "invokableResultType"]
        fn invokable_result_type(self: &MyObject) -> Result<String>;
    }

    impl cxx_qt::Threading for MyObject {}

    impl<'a>
        cxx_qt::Constructor<
            (i32, &'a QString),
            BaseArguments = (*mut QObject,),
            NewArguments = (&'a QString,),
        > for MyObject
    {
    }

    impl cxx_qt::Constructor<()> for MyObject {}
}
