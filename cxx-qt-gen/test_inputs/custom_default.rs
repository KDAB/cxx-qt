#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    pub struct Data {
        public: i32,
    }

    impl Default for Data {
        fn default() -> Self {
            Self { public: 32 }
        }
    }

    pub struct RustObj {
        private: i32,
    }

    impl Default for RustObj {
        fn default() -> Self {
            Self { private: 64 }
        }
    }
}
