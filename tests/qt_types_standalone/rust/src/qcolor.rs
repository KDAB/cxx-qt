// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_lib::QColor;

#[cxx::bridge]
mod qcolor_cxx {
    enum ColorTest {
        Rgb_Red,
        Rgb_Green,
        Rgb_Blue,
        Rgb_Transparent,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
    }

    extern "Rust" {
        fn construct_qcolor(test: ColorTest) -> QColor;
        fn read_qcolor(c: &QColor, test: ColorTest) -> bool;
        fn clone_qcolor(c: &QColor) -> QColor;
    }
}

use qcolor_cxx::ColorTest;

fn construct_qcolor(test: ColorTest) -> QColor {
    match test {
        ColorTest::Rgb_Red => QColor::from_rgb(255, 0, 0),
        ColorTest::Rgb_Green => QColor::from_rgb(0, 255, 0),
        ColorTest::Rgb_Blue => QColor::from_rgb(0, 0, 255),
        ColorTest::Rgb_Transparent => QColor::from_rgba(0, 0, 0, 0),
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn read_qcolor(color: &QColor, test: ColorTest) -> bool {
    match test {
        ColorTest::Rgb_Red => {
            color.alpha() == 255 && color.red() == 255 && color.green() == 0 && color.blue() == 0
        }
        ColorTest::Rgb_Green => {
            color.alpha() == 255 && color.red() == 0 && color.green() == 255 && color.blue() == 0
        }
        ColorTest::Rgb_Blue => {
            color.alpha() == 255 && color.red() == 0 && color.green() == 0 && color.blue() == 255
        }
        ColorTest::Rgb_Transparent => {
            color.alpha() == 0 && color.red() == 0 && color.green() == 0 && color.blue() == 0
        }
        _others => panic!("Unsupported test: {}", test.repr),
    }
}

fn clone_qcolor(c: &QColor) -> QColor {
    c.clone()
}
