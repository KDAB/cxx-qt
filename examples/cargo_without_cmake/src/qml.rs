// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qml.qrc.cpp");
        fn qInitResources() -> i32;
    }

    unsafe extern "C++" {
        include!("register_types.cpp");
        fn register_types();
    }
}
