// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
mod extract;
mod gen_cpp;
mod gen_qqmlextensionplugin;
mod gen_rs;
mod utils;

pub use extract::{extract_qobject, QObject};
pub use gen_cpp::{generate_format, generate_qobject_cpp, CppObject};
pub use gen_qqmlextensionplugin::QQmlExtensionPluginData;
pub use gen_rs::{generate_qobject_cxx, generate_qobject_rs};

/// The complete contents of the "rust/cxx_qt.h" header.
pub static HEADER: &str = include_str!("cxx_qt.h");

/// The complete contents of the "rust/update_requester.cpp" source file.
pub static UPDATE_REQUESTER_SOURCE: &str = include_str!("update_requester.cpp");

#[cfg(test)]
mod test {
    use super::*;

    use clang_format::ClangFormatStyle;

    #[ctor::ctor]
    fn init_tests() {
        // Set the ClangFormatStyle to be Mozilla for our tests
        // so that when they fail the format in the assertions is the same as the files.
        assert!(generate_format(Some(ClangFormatStyle::Mozilla)).is_ok());
    }
}
