use std::pin::Pin;

#[cxx_qt::bridge(cxx_file_stem = "rust_naming")]
pub mod qobject {
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element = "RenamedObject"]
        #[qproperty(i32, num)]
        // #[cxx_name = "RenamedObject"]
        type NamedObject = super::NamedObjectRust;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "increment"]
        #[rust_name = "plus_one"]
        fn increment_number(self: Pin<&mut NamedObject>);
    }
}

#[derive(Default)]
pub struct NamedObjectRust {
    pub(crate) num: i32
}

impl qobject::NamedObject {
    pub fn plus_one(self: Pin<&mut Self>) {
        let previous = *self.num();
        self.set_num(previous + 1);
    }
}