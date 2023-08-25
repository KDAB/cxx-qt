#[attrA]
#[cxx_qt::bridge(namespace = "cxx_qt::multi_object", cxx_file_stem = "multi_object")]
#[attrB]
pub mod ffi {
    // ItemConst
    const MAX: u16 = 65535;

    // ItemEnum
    enum Event {
        MyEvent,
    }

    // ItemExternCrate
    extern crate serde;

    // ItemFn
    fn do_something() {
        println!("I am a free function");
    }

    // ItemForeignMod
    extern "C" {}

    #[namespace = "namespace"]
    extern "C" {}

    #[namespace = "namespace"]
    #[custom_attr = "test"]
    extern "C" {}

    unsafe extern "C++" {}

    #[namespace = "namespace"]
    unsafe extern "C++" {}

    #[namespace = "namespace"]
    #[custom_attr = "test"]
    unsafe extern "C++" {}

    // ItemMacro
    macro_rules! macro1 {
        () => {
            0
        };
    }

    // ItemMacro2
    macro macro2() {
        0
    }

    // ItemMod
    mod m {}

    // ItemStatic
    static BIKE: Event = Event::MyEvent;

    // ItemTrait
    pub trait CustomTrait {
        fn method();
    }

    // ItemTraitAlias
    pub trait SharableIterator = CustomTrait + Sync;

    // ItemType
    type Result<T> = std::result::Result<T, Event>;

    // ItemUnion
    union Foo<A, B> {
        x: A,
        y: B,
    }

    // ItemUse
    use super::MyTrait;

    unsafe extern "C++" {
        include!(<QtCore/QStringListModel>);
    }

    #[namespace = ""]
    unsafe extern "C++Qt" {
        type QPushButton;

        #[qsignal]
        fn clicked(self: Pin<&mut QPushButton>, checked: bool);

        #[namespace = "mynamespace"]
        #[cxx_name = "ExternObjectCpp"]
        type ExternObject;

        #[qsignal]
        #[cxx_name = "dataReady"]
        fn data_ready(self: Pin<&mut ExternObject>);

        #[qsignal]
        #[rust_name = "error_occurred"]
        fn errorOccurred(self: Pin<&mut ExternObject>);
    }

    extern "RustQt" {
        #[qobject]
        #[base = "QStringListModel"]
        #[qproperty(i32, property_name)]
        type MyObject = super::MyObjectRust;
    }

    unsafe extern "RustQt" {
        #[qsignal]
        fn ready(self: Pin<&mut MyObject>);

        #[qinvokable]
        #[cxx_name = "invokableName"]
        fn invokable_name(self: Pin<&mut MyObject>);
    }

    extern "RustQt" {
        #[qobject]
        #[namespace = "second_object"]
        #[qproperty(i32, property_name)]
        type SecondObject = super::SecondObjectRust;
    }

    unsafe impl !cxx_qt::Locking for SecondObject {}

    unsafe extern "RustQt" {
        #[my_attribute]
        #[qsignal]
        fn ready(self: Pin<&mut SecondObject>);

        #[qinvokable]
        #[cxx_name = "invokableName"]
        fn invokable_name(self: Pin<&mut SecondObject>);
    }
}
