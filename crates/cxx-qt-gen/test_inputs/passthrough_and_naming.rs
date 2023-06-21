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

    #[cxx_qt::qobject(base = "QStringListModel")]
    pub struct MyObject {
        #[qproperty]
        property_name: i32,
    }

    unsafe extern "RustQt" {
        #[qsignal]
        fn ready(self: Pin<&mut qobject::MyObject>);

        #[qinvokable]
        fn invokable_name(self: Pin<&mut qobject::MyObject>);
    }

    impl MyTrait for MyObject {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }

    #[cxx_qt::qobject]
    pub struct SecondObject {
        #[qproperty]
        property_name: i32,
    }

    unsafe impl !cxx_qt::Locking for qobject::SecondObject {}

    unsafe extern "RustQt" {
        #[my_attribute]
        #[qsignal]
        fn ready(self: Pin<&mut qobject::SecondObject>);

        #[qinvokable]
        fn invokable_name(self: Pin<&mut qobject::SecondObject>);
    }
}
