#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    pub struct Data {
        public: i32,
    }

    impl Default for Data {
        fn default() -> Self {
            Self { public: 32 }
        }
    }

    #[cxx_qt::qobject]
    pub struct MyObject {
        private: i32,
    }

    impl Default for MyObject {
        fn default() -> Self {
            Self { private: 64 }
        }
    }
}
