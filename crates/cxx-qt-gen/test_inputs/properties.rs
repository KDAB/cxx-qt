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
        #[qproperty(i32, custom_function_prop, read = my_getter, write = my_setter, notify)]
        #[qproperty(i32, readonly_prop, read)]
        #[qproperty(i32, custom_on_changed_prop, read, write, notify = myOnChanged)]
        #[qproperty(i32, const_prop, read, constant)]
        #[qproperty(i32, resettable_prop, read, write, reset = myResetFn)]
        #[qproperty(i32, required_prop, read, write, required)]
        type MyObject = super::MyObjectRust;
    }
}
