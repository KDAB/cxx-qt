mod my_object {
    struct Data {
        public: i32,
    }

    impl Default for Data {
        fn default() -> Self {
            Self { public: 32 }
        }
    }

    struct RustObj {
        private: i32,
    }

    impl Default for RustObj {
        fn default() -> Self {
            Self { private: 64 }
        }
    }
}
