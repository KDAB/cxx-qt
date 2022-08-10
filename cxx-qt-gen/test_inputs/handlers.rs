#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QString = cxx_qt_lib::QString;
    }

    #[derive(Default)]
    pub struct Data {
        number: i32,
        string: QString,
    }

    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct RustObj;

    impl UpdateRequestHandler<CppObj> for RustObj {
        fn handle_update_request(&mut self, _cpp: &mut CppObj) {
            println!("update")
        }
    }
}
