#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
    }
    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt/connection.h");
        #[doc(hidden)]
        #[namespace = "Qt"]
        #[rust_name = "CxxQtConnectionType"]
        #[allow(dead_code)]
        type ConnectionType = cxx_qt::ConnectionType;
        #[doc(hidden)]
        #[namespace = "rust::cxxqt1"]
        #[rust_name = "CxxQtQMetaObjectConnection"]
        #[allow(dead_code)]
        type QMetaObjectConnection = cxx_qt::QMetaObjectConnection;
    }
    unsafe extern "C++" {
        include!("directory/file_ident.cxxqt.h");
    }
    unsafe extern "C++" {
        #[doc = "The C++ type for the QObject "]
        #[doc = "MyObjectRust"]
        #[doc = "\n"]
        #[doc = "Use this type when referring to the QObject as a pointer"]
        #[doc = "\n"]
        #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
        #[namespace = "cxx_qt::my_object"]
        type MyObject;
    }
    extern "Rust" {
        #[namespace = "cxx_qt::my_object"]
        type MyObjectRust;
    }
    extern "Rust" {
        #[cxx_name = "getPrimitive"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn primitive<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPrimitive"]
        #[namespace = "cxx_qt::my_object"]
        fn set_primitive(self: Pin<&mut MyObject>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "getTrivial"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn trivial<'a>(self: &'a MyObject) -> &'a QPoint;
    }
    extern "Rust" {
        #[cxx_name = "setTrivial"]
        #[namespace = "cxx_qt::my_object"]
        fn set_trivial(self: Pin<&mut MyObject>, value: QPoint);
    }
    extern "Rust" {
        #[cxx_name = "getPropAutoCxxName"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn prop_auto_cxx_name<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setPropAutoCxxName"]
        #[namespace = "cxx_qt::my_object"]
        fn set_prop_auto_cxx_name(self: Pin<&mut MyObject>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "getReadonlyProp"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn readonly_prop<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "getRenamedProperty"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn renamed_property<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setRenamedProperty"]
        #[namespace = "cxx_qt::my_object"]
        fn set_renamed_property(self: Pin<&mut MyObject>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "getNamed_prop_2"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn renamed_property_2<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setNamed_prop_2"]
        #[namespace = "cxx_qt::my_object"]
        fn set_renamed_property_2(self: Pin<&mut MyObject>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "getCustomOnChangedProp"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn custom_on_changed_prop<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setCustomOnChangedProp"]
        #[namespace = "cxx_qt::my_object"]
        fn set_custom_on_changed_prop(self: Pin<&mut MyObject>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "getConstProp"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn const_prop<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "getResettableProp"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn resettable_prop<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setResettableProp"]
        #[namespace = "cxx_qt::my_object"]
        fn set_resettable_prop(self: Pin<&mut MyObject>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "getRequiredProp"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn required_prop<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setRequiredProp"]
        #[namespace = "cxx_qt::my_object"]
        fn set_required_prop(self: Pin<&mut MyObject>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "getFinalProp"]
        #[namespace = "cxx_qt::my_object"]
        unsafe fn final_prop<'a>(self: &'a MyObject) -> &'a i32;
    }
    extern "Rust" {
        #[cxx_name = "setFinalProp"]
        #[namespace = "cxx_qt::my_object"]
        fn set_final_prop(self: Pin<&mut MyObject>, value: i32);
    }
    unsafe extern "C++" {
        #[cxx_name = "primitiveChanged"]
        #[doc = "Notify for the Q_PROPERTY"]
        #[namespace = "cxx_qt::my_object"]
        fn primitive_changed(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlerprimitiveChanged = cxx_qt::signalhandler::CxxQtSignalHandler<
            super::MyObjectCxxQtSignalClosureprimitiveChanged,
        >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_primitiveChangedConnect"]
        fn MyObject_connect_primitive_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlerprimitiveChanged,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_primitiveChanged(
            handler: MyObjectCxxQtSignalHandlerprimitiveChanged,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_primitiveChanged(
            handler: &mut MyObjectCxxQtSignalHandlerprimitiveChanged,
            self_value: Pin<&mut MyObject>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "trivialChanged"]
        #[doc = "Notify for the Q_PROPERTY"]
        #[namespace = "cxx_qt::my_object"]
        fn trivial_changed(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlertrivialChanged = cxx_qt::signalhandler::CxxQtSignalHandler<
            super::MyObjectCxxQtSignalClosuretrivialChanged,
        >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_trivialChangedConnect"]
        fn MyObject_connect_trivial_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlertrivialChanged,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_trivialChanged(
            handler: MyObjectCxxQtSignalHandlertrivialChanged,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_trivialChanged(
            handler: &mut MyObjectCxxQtSignalHandlertrivialChanged,
            self_value: Pin<&mut MyObject>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "propAutoCxxNameChanged"]
        #[doc = "Notify for the Q_PROPERTY"]
        #[namespace = "cxx_qt::my_object"]
        fn prop_auto_cxx_name_changed(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlerpropAutoCxxNameChanged =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::MyObjectCxxQtSignalClosurepropAutoCxxNameChanged,
            >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_propAutoCxxNameChangedConnect"]
        fn MyObject_connect_prop_auto_cxx_name_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlerpropAutoCxxNameChanged,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_propAutoCxxNameChanged(
            handler: MyObjectCxxQtSignalHandlerpropAutoCxxNameChanged,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_propAutoCxxNameChanged(
            handler: &mut MyObjectCxxQtSignalHandlerpropAutoCxxNameChanged,
            self_value: Pin<&mut MyObject>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "customFunctionPropChanged"]
        #[doc = "Notify for the Q_PROPERTY"]
        #[namespace = "cxx_qt::my_object"]
        fn custom_function_prop_changed(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlercustomFunctionPropChanged =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::MyObjectCxxQtSignalClosurecustomFunctionPropChanged,
            >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_customFunctionPropChangedConnect"]
        fn MyObject_connect_custom_function_prop_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlercustomFunctionPropChanged,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_customFunctionPropChanged(
            handler: MyObjectCxxQtSignalHandlercustomFunctionPropChanged,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_customFunctionPropChanged(
            handler: &mut MyObjectCxxQtSignalHandlercustomFunctionPropChanged,
            self_value: Pin<&mut MyObject>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "renamedPropertyChanged"]
        #[doc = "Notify for the Q_PROPERTY"]
        #[namespace = "cxx_qt::my_object"]
        fn renamed_property_changed(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlerrenamedPropertyChanged =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::MyObjectCxxQtSignalClosurerenamedPropertyChanged,
            >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_renamedPropertyChangedConnect"]
        fn MyObject_connect_renamed_property_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlerrenamedPropertyChanged,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_renamedPropertyChanged(
            handler: MyObjectCxxQtSignalHandlerrenamedPropertyChanged,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_renamedPropertyChanged(
            handler: &mut MyObjectCxxQtSignalHandlerrenamedPropertyChanged,
            self_value: Pin<&mut MyObject>,
        );
    }
    unsafe extern "C++" {
        #[cxx_name = "named_prop_2Changed"]
        #[doc = "Notify for the Q_PROPERTY"]
        #[namespace = "cxx_qt::my_object"]
        fn renamed_property_2_changed(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlernamed_prop_2Changed =
            cxx_qt::signalhandler::CxxQtSignalHandler<
                super::MyObjectCxxQtSignalClosurenamed_prop_2Changed,
            >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_named_prop_2ChangedConnect"]
        fn MyObject_connect_renamed_property_2_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlernamed_prop_2Changed,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_named_prop_2Changed(
            handler: MyObjectCxxQtSignalHandlernamed_prop_2Changed,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_named_prop_2Changed(
            handler: &mut MyObjectCxxQtSignalHandlernamed_prop_2Changed,
            self_value: Pin<&mut MyObject>,
        );
    }
    extern "Rust" {
        #[cxx_name = "myGetter"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn my_getter(self: &MyObject) -> i32;
    }
    extern "Rust" {
        #[cxx_name = "MyCustomSetter"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn my_setter(self: Pin<&mut MyObject>, value: i32);
    }
    extern "Rust" {
        #[cxx_name = "myResetFn"]
        #[namespace = "cxx_qt::my_object"]
        #[doc(hidden)]
        fn myResetFn(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[cxx_name = "my_on_changed"]
        #[namespace = "cxx_qt::my_object"]
        fn my_on_changed(self: Pin<&mut MyObject>);
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        type MyObjectCxxQtSignalHandlermy_on_changed = cxx_qt::signalhandler::CxxQtSignalHandler<
            super::MyObjectCxxQtSignalClosuremy_on_changed,
        >;
        #[doc(hidden)]
        #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
        #[cxx_name = "MyObject_my_on_changedConnect"]
        fn MyObject_connect_my_on_changed(
            self_value: Pin<&mut MyObject>,
            signal_handler: MyObjectCxxQtSignalHandlermy_on_changed,
            conn_type: CxxQtConnectionType,
        ) -> CxxQtQMetaObjectConnection;
    }
    #[namespace = "cxx_qt::my_object::rust::cxxqtgen1"]
    extern "Rust" {
        #[doc(hidden)]
        fn drop_MyObject_signal_handler_my_on_changed(
            handler: MyObjectCxxQtSignalHandlermy_on_changed,
        );
        #[doc(hidden)]
        fn call_MyObject_signal_handler_my_on_changed(
            handler: &mut MyObjectCxxQtSignalHandlermy_on_changed,
            self_value: Pin<&mut MyObject>,
        );
    }
    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_MyObject"]
        fn create_rs_MyObjectRust() -> Box<MyObjectRust>;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRust"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_unsafeRust(outer: &MyObject) -> &MyObjectRust;
    }
    unsafe extern "C++" {
        #[doc(hidden)]
        #[cxx_name = "unsafeRustMut"]
        #[namespace = "rust::cxxqt1"]
        fn cxx_qt_ffi_MyObject_unsafeRustMut(outer: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "primitive"]
    pub fn primitive(&self) -> &i32 {
        &self.primitive
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "primitive"]
    pub fn set_primitive(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.primitive == value {
            return;
        }
        self.as_mut().rust_mut().primitive = value;
        self.as_mut().primitive_changed();
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "trivial"]
    pub fn trivial(&self) -> &ffi::QPoint {
        &self.trivial
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "trivial"]
    pub fn set_trivial(mut self: core::pin::Pin<&mut Self>, value: ffi::QPoint) {
        use cxx_qt::CxxQtType;
        if self.trivial == value {
            return;
        }
        self.as_mut().rust_mut().trivial = value;
        self.as_mut().trivial_changed();
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "prop_auto_cxx_name"]
    pub fn prop_auto_cxx_name(&self) -> &i32 {
        &self.prop_auto_cxx_name
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "prop_auto_cxx_name"]
    pub fn set_prop_auto_cxx_name(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.prop_auto_cxx_name == value {
            return;
        }
        self.as_mut().rust_mut().prop_auto_cxx_name = value;
        self.as_mut().prop_auto_cxx_name_changed();
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "readonly_prop"]
    pub fn readonly_prop(&self) -> &i32 {
        &self.readonly_prop
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "renamed_property"]
    pub fn renamed_property(&self) -> &i32 {
        &self.renamed_property
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "renamed_property"]
    pub fn set_renamed_property(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.renamed_property == value {
            return;
        }
        self.as_mut().rust_mut().renamed_property = value;
        self.as_mut().renamed_property_changed();
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "renamed_property_2"]
    pub fn renamed_property_2(&self) -> &i32 {
        &self.renamed_property_2
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "renamed_property_2"]
    pub fn set_renamed_property_2(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.renamed_property_2 == value {
            return;
        }
        self.as_mut().rust_mut().renamed_property_2 = value;
        self.as_mut().renamed_property_2_changed();
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "custom_on_changed_prop"]
    pub fn custom_on_changed_prop(&self) -> &i32 {
        &self.custom_on_changed_prop
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "custom_on_changed_prop"]
    pub fn set_custom_on_changed_prop(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.custom_on_changed_prop == value {
            return;
        }
        self.as_mut().rust_mut().custom_on_changed_prop = value;
        self.as_mut().my_on_changed();
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "const_prop"]
    pub fn const_prop(&self) -> &i32 {
        &self.const_prop
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "resettable_prop"]
    pub fn resettable_prop(&self) -> &i32 {
        &self.resettable_prop
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "resettable_prop"]
    pub fn set_resettable_prop(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.resettable_prop == value {
            return;
        }
        self.as_mut().rust_mut().resettable_prop = value;
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "required_prop"]
    pub fn required_prop(&self) -> &i32 {
        &self.required_prop
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "required_prop"]
    pub fn set_required_prop(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.required_prop == value {
            return;
        }
        self.as_mut().rust_mut().required_prop = value;
    }
}
impl ffi::MyObject {
    #[doc = "Getter for the Q_PROPERTY "]
    #[doc = "final_prop"]
    pub fn final_prop(&self) -> &i32 {
        &self.final_prop
    }
}
impl ffi::MyObject {
    #[doc = "Setter for the Q_PROPERTY "]
    #[doc = "final_prop"]
    pub fn set_final_prop(mut self: core::pin::Pin<&mut Self>, value: i32) {
        use cxx_qt::CxxQtType;
        if self.final_prop == value {
            return;
        }
        self.as_mut().rust_mut().final_prop = value;
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "primitiveChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_primitive_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::MyObject_connect_primitive_changed(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    MyObjectCxxQtSignalClosureprimitiveChanged,
                >::new(Box::new(closure)),
                conn_type,
            ),
        )
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "primitiveChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_primitive_changed<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::MyObject_connect_primitive_changed(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    MyObjectCxxQtSignalClosureprimitiveChanged,
                >::new(Box::new(closure)),
                cxx_qt::ConnectionType::AutoConnection,
            ),
        )
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosureprimitiveChanged {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for MyObjectCxxQtSignalClosureprimitiveChanged
{
    type Id = cxx::type_id!(
        "::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerprimitiveChanged"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>) + Send;
}
use core::mem::drop as drop_MyObject_signal_handler_primitiveChanged;
fn call_MyObject_signal_handler_primitiveChanged(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosureprimitiveChanged,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureprimitiveChanged>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosureprimitiveChanged>,
    [usize; 2]
);
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "trivialChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_trivial_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: MyObject_connect_trivial_changed (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < MyObjectCxxQtSignalClosuretrivialChanged > :: new (Box :: new (closure)) , conn_type ,))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "trivialChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_trivial_changed<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: MyObject_connect_trivial_changed (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < MyObjectCxxQtSignalClosuretrivialChanged > :: new (Box :: new (closure)) , cxx_qt :: ConnectionType :: AutoConnection ,))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosuretrivialChanged {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosuretrivialChanged {
    type Id = cxx::type_id!(
        "::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlertrivialChanged"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>) + Send;
}
use core::mem::drop as drop_MyObject_signal_handler_trivialChanged;
fn call_MyObject_signal_handler_trivialChanged(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosuretrivialChanged,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuretrivialChanged>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuretrivialChanged>,
    [usize; 2]
);
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "propAutoCxxNameChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_prop_auto_cxx_name_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_prop_auto_cxx_name_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                MyObjectCxxQtSignalClosurepropAutoCxxNameChanged,
            >::new(Box::new(closure)),
            conn_type,
        ))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "propAutoCxxNameChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_prop_auto_cxx_name_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_prop_auto_cxx_name_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                MyObjectCxxQtSignalClosurepropAutoCxxNameChanged,
            >::new(Box::new(closure)),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosurepropAutoCxxNameChanged {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for MyObjectCxxQtSignalClosurepropAutoCxxNameChanged
{
    type Id = cxx::type_id!(
        "::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerpropAutoCxxNameChanged"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>) + Send;
}
use core::mem::drop as drop_MyObject_signal_handler_propAutoCxxNameChanged;
fn call_MyObject_signal_handler_propAutoCxxNameChanged(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosurepropAutoCxxNameChanged,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurepropAutoCxxNameChanged>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurepropAutoCxxNameChanged>,
    [usize; 2]
);
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "customFunctionPropChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_custom_function_prop_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::MyObject_connect_custom_function_prop_changed(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    MyObjectCxxQtSignalClosurecustomFunctionPropChanged,
                >::new(Box::new(closure)),
                conn_type,
            ),
        )
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "customFunctionPropChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_custom_function_prop_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(
            ffi::MyObject_connect_custom_function_prop_changed(
                self,
                cxx_qt::signalhandler::CxxQtSignalHandler::<
                    MyObjectCxxQtSignalClosurecustomFunctionPropChanged,
                >::new(Box::new(closure)),
                cxx_qt::ConnectionType::AutoConnection,
            ),
        )
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosurecustomFunctionPropChanged {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for MyObjectCxxQtSignalClosurecustomFunctionPropChanged
{
    type Id = cxx::type_id!(
        "::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlercustomFunctionPropChanged"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>) + Send;
}
use core::mem::drop as drop_MyObject_signal_handler_customFunctionPropChanged;
fn call_MyObject_signal_handler_customFunctionPropChanged(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosurecustomFunctionPropChanged,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurecustomFunctionPropChanged>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurecustomFunctionPropChanged>,
    [usize; 2]
);
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "renamedPropertyChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_renamed_property_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_renamed_property_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                MyObjectCxxQtSignalClosurerenamedPropertyChanged,
            >::new(Box::new(closure)),
            conn_type,
        ))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "renamedPropertyChanged"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_renamed_property_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_renamed_property_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                MyObjectCxxQtSignalClosurerenamedPropertyChanged,
            >::new(Box::new(closure)),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosurerenamedPropertyChanged {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for MyObjectCxxQtSignalClosurerenamedPropertyChanged
{
    type Id = cxx::type_id!(
        "::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerrenamedPropertyChanged"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>) + Send;
}
use core::mem::drop as drop_MyObject_signal_handler_renamedPropertyChanged;
fn call_MyObject_signal_handler_renamedPropertyChanged(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosurerenamedPropertyChanged,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurerenamedPropertyChanged>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurerenamedPropertyChanged>,
    [usize; 2]
);
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "named_prop_2Changed"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_renamed_property_2_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_renamed_property_2_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                MyObjectCxxQtSignalClosurenamed_prop_2Changed,
            >::new(Box::new(closure)),
            conn_type,
        ))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "named_prop_2Changed"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_renamed_property_2_changed<
        F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send,
    >(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt::QMetaObjectConnectionGuard::from(ffi::MyObject_connect_renamed_property_2_changed(
            self,
            cxx_qt::signalhandler::CxxQtSignalHandler::<
                MyObjectCxxQtSignalClosurenamed_prop_2Changed,
            >::new(Box::new(closure)),
            cxx_qt::ConnectionType::AutoConnection,
        ))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosurenamed_prop_2Changed {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure
    for MyObjectCxxQtSignalClosurenamed_prop_2Changed
{
    type Id = cxx::type_id!(
        "::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlernamed_prop_2Changed"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>) + Send;
}
use core::mem::drop as drop_MyObject_signal_handler_named_prop_2Changed;
fn call_MyObject_signal_handler_named_prop_2Changed(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosurenamed_prop_2Changed,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurenamed_prop_2Changed>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosurenamed_prop_2Changed>,
    [usize; 2]
);
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "my_on_changed"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    pub fn connect_my_on_changed<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
        conn_type: cxx_qt::ConnectionType,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: MyObject_connect_my_on_changed (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < MyObjectCxxQtSignalClosuremy_on_changed > :: new (Box :: new (closure)) , conn_type ,))
    }
}
impl ffi::MyObject {
    #[doc = "Connect the given function pointer to the signal "]
    #[doc = "my_on_changed"]
    #[doc = ", so that when the signal is emitted the function pointer is executed."]
    #[doc = "\n"]
    #[doc = "Note that this method uses a AutoConnection connection type."]
    pub fn on_my_on_changed<F: FnMut(core::pin::Pin<&mut ffi::MyObject>) + 'static + Send>(
        self: core::pin::Pin<&mut ffi::MyObject>,
        mut closure: F,
    ) -> cxx_qt::QMetaObjectConnectionGuard {
        cxx_qt :: QMetaObjectConnectionGuard :: from (ffi :: MyObject_connect_my_on_changed (self , cxx_qt :: signalhandler :: CxxQtSignalHandler :: < MyObjectCxxQtSignalClosuremy_on_changed > :: new (Box :: new (closure)) , cxx_qt :: ConnectionType :: AutoConnection ,))
    }
}
#[doc(hidden)]
pub struct MyObjectCxxQtSignalClosuremy_on_changed {}
impl cxx_qt::signalhandler::CxxQtSignalHandlerClosure for MyObjectCxxQtSignalClosuremy_on_changed {
    type Id = cxx::type_id!(
        "::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlermy_on_changed"
    );
    type FnType = dyn FnMut(core::pin::Pin<&mut ffi::MyObject>) + Send;
}
use core::mem::drop as drop_MyObject_signal_handler_my_on_changed;
fn call_MyObject_signal_handler_my_on_changed(
    handler: &mut cxx_qt::signalhandler::CxxQtSignalHandler<
        MyObjectCxxQtSignalClosuremy_on_changed,
    >,
    self_value: core::pin::Pin<&mut ffi::MyObject>,
) {
    handler.closure()(self_value);
}
cxx_qt::static_assertions::assert_eq_align!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuremy_on_changed>,
    usize
);
cxx_qt::static_assertions::assert_eq_size!(
    cxx_qt::signalhandler::CxxQtSignalHandler<MyObjectCxxQtSignalClosuremy_on_changed>,
    [usize; 2]
);
#[doc(hidden)]
pub fn create_rs_MyObjectRust() -> std::boxed::Box<MyObjectRust> {
    std::boxed::Box::new(core::default::Default::default())
}
impl ::core::ops::Deref for ffi::MyObject {
    type Target = MyObjectRust;
    fn deref(&self) -> &Self::Target {
        ffi::cxx_qt_ffi_MyObject_unsafeRust(self)
    }
}
impl ::cxx_qt::CxxQtType for ffi::MyObject {
    type Rust = MyObjectRust;
    fn rust(&self) -> &Self::Rust {
        ffi::cxx_qt_ffi_MyObject_unsafeRust(self)
    }
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust> {
        ffi::cxx_qt_ffi_MyObject_unsafeRustMut(self)
    }
}
