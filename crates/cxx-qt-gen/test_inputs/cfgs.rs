#[cxx_qt::bridge]
mod ffi {
    // Enabled C++Qt QObject
    // - disabled and enabled qsignal
    unsafe extern "C++Qt" {
        #[qobject]
        #[cfg(enabled)]
        type QObjectExternEnabled;

        #[qsignal]
        #[cfg(not(enabled))]
        fn signal_disabled1(self: Pin<&mut QObjectExternEnabled>);

        #[qsignal]
        #[cfg(enabled)]
        fn signal_enabled1(self: Pin<&mut QObjectExternEnabled>);
    }

    // Disabled C++Qt QObject
    // - disabled and enabled qsignal
    unsafe extern "C++Qt" {
        #[qobject]
        #[cfg(not(enabled))]
        type QObjectExternDisabled;

        #[qsignal]
        #[cfg(not(enabled))]
        fn signal_disabled2(self: Pin<&mut QObjectExternDisabled>);

        #[qsignal]
        #[cfg(enabled)]
        fn signal_enabled2(self: Pin<&mut QObjectExternDisabled>);
    }

    // Enabled RustQt QObject
    // - disabled and enabled qenum

    #[qenum(QObjectEnabled)]
    #[cfg(not(enabled))]
    enum EnumDisabled1 {
        A,
    }

    #[qenum(QObjectEnabled)]
    #[cfg(enabled)]
    enum EnumEnabled1 {
        A,
    }

    // Enabled RustQt QObject
    // - disabled and enabled inherit
    // - disabled and enabled invokable
    // - disabled and enabled signal
    unsafe extern "RustQt" {
        #[qobject]
        #[cfg(enabled)]
        // TODO: should we allow for disabling properties?
        // #[qproperty(i32, property_disabled, cfg(not(enabled)))]
        type QObjectEnabled = super::QObjectEnabledRust;

        #[inherit]
        #[cfg(not(enabled))]
        fn inherit_disabled(self: &QObjectEnabled);

        #[inherit]
        #[cfg(enabled)]
        fn inherit_enabled(self: &QObjectEnabled);

        #[qinvokable]
        #[cfg(not(enabled))]
        fn invokable_disabled(self: &QObjectEnabled);

        #[qinvokable]
        #[cfg(enabled)]
        fn invokable_enabled(self: &QObjectEnabled);

        #[qsignal]
        #[cfg(not(enabled))]
        fn signal_disabled(self: Pin<&mut QObjectEnabled>);

        #[qsignal]
        #[cfg(enabled)]
        fn signal_enabled(self: Pin<&mut QObjectEnabled>);
    }

    // Dislabed RustQt QObject
    // - disabled and enabled qenum

    #[qenum(QObjectDisabled)]
    #[cfg(not(enabled))]
    enum EnumDisabled2 {
        A,
    }

    #[qenum(QObjectDisabled)]
    #[cfg(enabled)]
    enum EnumEnabled2 {
        A,
    }

    // Disabled RustQt QObject
    // - disabled and enabled inherit
    // - disabled and enabled invokable
    // - disabled and enabled signal
    unsafe extern "RustQt" {
        #[qobject]
        #[cfg(not(enabled))]
        // TODO: should we allow for disabling properties?
        // #[qproperty(i32, property_disabled, cfg(not(enabled)))]
        type QObjectDisabled = super::QObjectDisabledRust;

        #[inherit]
        #[cfg(not(enabled))]
        fn inherit_disabled(self: &QObjectDisabled);

        #[inherit]
        #[cfg(enabled)]
        fn inherit_enabled(self: &QObjectDisabled);

        #[qinvokable]
        #[cfg(not(enabled))]
        fn invokable_disabled(self: &QObjectDisabled);

        #[qinvokable]
        #[cfg(enabled)]
        fn invokable_enabled(self: &QObjectDisabled);

        #[qsignal]
        #[cfg(not(enabled))]
        fn signal_disabled(self: Pin<&mut QObjectDisabled>);

        #[qsignal]
        #[cfg(enabled)]
        fn signal_enabled(self: Pin<&mut QObjectDisabled>);
    }
}
