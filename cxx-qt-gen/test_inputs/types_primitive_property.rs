#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod my_object {
    #[derive(Default)]
    pub struct Data {
        boolean: bool,
        float_32: f32,
        float_64: f64,
        int_8: i8,
        int_16: i16,
        int_32: i32,
        uint_8: u8,
        uint_16: u16,
        uint_32: u32,
    }

    #[derive(Default)]
    pub struct RustObj;
}
