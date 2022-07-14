mod my_object {
    extern "Qt" {
        #[derive(Default)]
        struct Data {
            number: i32,
            string: String,
        }

        #[derive(Default)]
        struct RustObj;

        impl PropertyChangeHandler<CppObj, Property> for RustObj {
            fn handle_property_change(&mut self, _cpp: &mut CppObj, _property: Property) {
                println!("change")
            }
        }

        impl UpdateRequestHandler<CppObj> for RustObj {
            fn handle_update_request(&mut self, _cpp: &mut CppObj) {
                println!("update")
            }
        }
    }
}
