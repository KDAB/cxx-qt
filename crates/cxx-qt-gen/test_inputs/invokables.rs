#[cxx_qt::bridge(cxx_file_stem = "invokables", namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!(<QtCore/QObject>);
        type QObject;

        type Opaque;
    }

    unsafe extern "RustQt" {
        #[qobject]
        type MyObject = super::MyObjectRust;

        fn cpp_method(self: &MyObject);

        #[qinvokable]
        fn invokable(self: &MyObject);

        #[qinvokable]
        fn invokable_mutable(self: Pin<&mut MyObject>);

        #[qinvokable]
        fn invokable_parameters(self: &MyObject, opaque: &QColor, trivial: &QPoint, primitive: i32);

        #[qinvokable]
        fn invokable_return_opaque(self: Pin<&mut MyObject>) -> UniquePtr<Opaque>;

        #[qinvokable]
        fn invokable_return_trivial(self: Pin<&mut MyObject>) -> QPoint;

        #[qinvokable]
        #[cxx_final]
        fn invokable_final(self: &MyObject);

        #[qinvokable]
        #[cxx_override]
        fn invokable_override(self: &MyObject);

        #[qinvokable]
        #[cxx_virtual]
        fn invokable_virtual(self: &MyObject);

        #[qinvokable]
        fn invokable_result_tuple(self: &MyObject) -> Result<()>;

        #[qinvokable]
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
