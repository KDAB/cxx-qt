#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }

    #[auto_cxx_name]
    extern "RustQt" {
        #[qobject]
        #[qproperty(i32, primitive)]
        #[qproperty(QPoint, trivial)]
        #[qproperty(i32, prop_auto_cxx_name)]
        #[qproperty(i32, custom_function_prop, cxx_name = "customFunctionProp", READ = my_getter, WRITE = my_setter, NOTIFY)]
        #[qproperty(i32, readonly_prop, cxx_name = "readonlyProp", READ)]
        #[qproperty(
            i32,
            named_prop,
            cxx_name = "renamedProperty",
            rust_name = "renamed_property"
        )]
        #[qproperty(i32, named_prop_2, rust_name = "renamed_property_2")]
        #[qproperty(i32, custom_on_changed_prop, cxx_name = "customOnChangedProp", READ, WRITE, NOTIFY = my_on_changed)]
        #[qproperty(i32, const_prop, cxx_name = "constProp", READ, CONSTANT)]
        #[qproperty(i32, resettable_prop, cxx_name = "resettableProp", READ, WRITE, RESET = myResetFn)]
        #[qproperty(i32, required_prop, cxx_name = "requiredProp", READ, WRITE, REQUIRED)]
        #[qproperty(i32, final_prop, cxx_name = "finalProp", READ, WRITE, FINAL)]
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
