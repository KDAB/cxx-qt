#[attrA]
#[cxx_qt::bridge]
#[attrB]
pub mod my_object {
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

    #[derive(Default)]
    pub struct Data {
        number: i32,
    }

    impl MyTrait for Data {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        fn test_angled(&self, optional: Option<bool>) -> Option<bool> {
            optional
        }

        fn test_unknown(&self, unknown: MyType) -> MyType {
            unknown
        }
    }

    impl MyTrait for RustObj {
        fn my_func() -> String {
            "Hello".to_owned()
        }
    }
}
