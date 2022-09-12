#[cxx_qt::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[cxx_qt::qobject]
    #[derive(Default)]
    pub struct MyObject {
        #[qproperty]
        boolean: bool,
        #[qproperty]
        float_32: f32,
        #[qproperty]
        float_64: f64,
        #[qproperty]
        int_8: i8,
        #[qproperty]
        int_16: i16,
        #[qproperty]
        int_32: i32,
        #[qproperty]
        uint_8: u8,
        #[qproperty]
        uint_16: u16,
        #[qproperty]
        uint_32: u32,
    }
}
