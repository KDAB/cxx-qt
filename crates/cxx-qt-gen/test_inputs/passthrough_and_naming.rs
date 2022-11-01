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

    impl Default for MyObject {
        fn default() -> Self {
            Self { property_name: 32 }
        }
    }

    #[cxx_qt::qsignals(MyObject)]
    pub enum MySignals {
        Ready,
    }

    impl qobject::MyObject {
        #[qinvokable]
        pub fn invokable_name(self: Pin<&mut Self>) {
            println!("Bye from Rust!");
            self.as_mut().set_property_name(5);
        }
    }

    impl MyObject {
        fn test_angled(&self, optional: Option<bool>) -> Option<bool> {
            optional
        }

        fn test_unknown(&self, unknown: MyType) -> MyType {
            unknown
        }
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

    impl Default for SecondObject {
        fn default() -> Self {
            Self { property_name: 32 }
        }
    }

    #[cxx_qt::qsignals(SecondObject)]
    pub enum SecondSignals {
        Ready,
    }

    impl qobject::SecondObject {
        #[qinvokable]
        pub fn invokable_name(self: Pin<&mut Self>) {
            println!("Bye from second Rust!");
            self.as_mut().set_property_name(5);
        }
    }
}
