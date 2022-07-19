mod my_object {
    #[derive(Default)]
    pub struct Data {
        number: i32,
        string: String,
    }

    #[derive(Default)]
    pub struct RustObj;

    impl UpdateRequestHandler<CppObj> for RustObj {
        fn handle_update_request(&mut self, _cpp: &mut CppObj) {
            println!("update")
        }
    }
}
