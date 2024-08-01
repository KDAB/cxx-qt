#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    extern "RustQt" {
        #[qobject]
        #[derive(Default)]
        #[qproperty(i32, primitive)]
        #[qproperty(QPoint, trivial)]
        #[qproperty(i32, custom_function_prop, READ = my_getter, WRITE = my_setter, NOTIFY)]
        #[qproperty(i32, readonly_prop, READ)]
        #[qproperty(i32, custom_on_changed_prop, READ, WRITE, NOTIFY = my_on_changed)]
        #[qproperty(i32, const_prop, READ, CONSTANT)]
        #[qproperty(i32, resettable_prop, READ, WRITE, RESET = myResetFn)]
        #[qproperty(i32, required_prop, READ, WRITE, REQUIRED)]
        #[qproperty(i32, final_prop, READ, WRITE, FINAL)]
        type MyObject = super::MyObjectRust;
    }

    unsafe extern "RustQt" {
        #[rust_name = "my_getter"]
        fn myGetter(self: &MyObject) -> i32;

        #[cxx_name = "MyCustomSetter"]
        fn my_setter(self: Pin<&mut MyObject>, value: i32);

        #[qsignal]
        fn my_on_changed(self: Pin<&mut MyObject>);

        fn myResetFn(self: Pin<&mut MyObject>);
    }
}
