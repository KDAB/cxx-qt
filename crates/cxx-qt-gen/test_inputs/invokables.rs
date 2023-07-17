#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    unsafe extern "RustQt" {
        #[cxx_qt::qobject]
        type MyObject = super::MyObjectRust;

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

    impl cxx_qt::Threading for qobject::MyObject {}

    impl
        cxx_qt::Constructor<
            (i32, *mut QObject),
            BaseArguments = (*mut QObject,),
            NewArguments = (i32,),
        > for qobject::MyObject
    {
    }
}
