#[cxx_qt::bridge]
mod ffi {
    #[qenum(MyObject)]
    #[cfg(not(enabled))]
    enum EnumDisabled {
        A,
    }

    unsafe extern "C++Qt" {}
    unsafe extern "RustQt" {
        // TODO: should we allow for disabling qobjects?
        #[qobject]
        // TODO: should we allow for disabling properties?
        // #[qproperty(i32, property_disabled, cfg(not(enabled)))]
        type MyObject = super::MyObjectRust;

        #[inherit]
        #[cfg(not(enabled))]
        fn inherit_disabled(self: &MyObject);

        #[qinvokable]
        #[cfg(not(enabled))]
        fn invokable_disabled(self: &MyObject);

        #[qsignal]
        #[cfg(not(enabled))]
        fn signal_disabled(self: Pin<&mut MyObject>);
    }
}
