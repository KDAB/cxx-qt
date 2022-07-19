// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
mod nested {
    #[derive(Default)]
    pub struct Data {
        nested: crate::rust_obj_invokables::rust_obj_invokables::CppObj,
    }

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        #[invokable]
        pub fn nested_parameter(
            &self,
            nested: &mut crate::rust_obj_invokables::rust_obj_invokables::CppObj,
        ) {
            println!("Number: {}", nested.number());
            // TODO: we can't reach the nested object's RustObj yet
            // for this we will need `nested.borrow_rust_obj()` later
            // https://github.com/KDAB/cxx-qt/issues/30
        }

        #[invokable]
        pub fn nested_take_give(&self, cpp: &mut CppObj) {
            // We now own the nested object and QML would be null
            //
            // TODO: should this return a OwnedCppObj which derefs to the CppObj ?
            // (so that we don't need to do the CppObj::new(obj))
            // and holds the UniquePtr internally so that OwnedCppObj can be moved back in the give ?
            // https://github.com/KDAB/cxx-qt/issues/30
            let mut nested = cpp.take_nested();

            crate::rust_obj_invokables::rust_obj_invokables::CppObj::new(nested.pin_mut())
                .set_number(10);

            // The nested object is now back in QML
            cpp.give_nested(nested);
        }
    }
}
// ANCHOR_END: book_macro_code
