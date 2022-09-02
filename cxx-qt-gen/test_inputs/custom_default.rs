#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[cxx_qt::qobject]
    pub struct MyObject {
        #[qproperty]
        public: i32,

        private: i32,
    }

    impl Default for MyObject {
        fn default() -> Self {
            Self {
                public: 32,
                private: 64,
            }
        }
    }
}
