// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx_test.h");

        fn get_cpp_number() -> i32;
    }

    extern "Rust" {
        fn get_numbers_sum() -> i32;
    }
}

fn get_numbers_sum() -> i32 {
    ffi::get_cpp_number() + 2
}

#[cfg(test)]
mod tests {
    use super::ffi::get_cpp_number;

    #[test]
    fn test_get_numbers_sum() {
        assert_eq!(get_cpp_number(), 100);
    }
}
