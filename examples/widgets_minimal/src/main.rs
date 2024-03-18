// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example provides demostrations of building a Cargo only CXX-Qt application

mod qapplication_cxx;
mod qpushbutton_cxx;

use cxx_qt_lib::QString;
use qapplication_cxx::QApplication;
use qpushbutton_cxx::QPushButton;

fn main() {
    let mut app = QApplication::new();

    let mut push_button = QPushButton::new();

    if let Some(mut push_button) = push_button.as_mut() {
        push_button
            .as_mut()
            .set_text(&QString::from("Hello World!"));

        push_button
            .as_mut()
            .on_clicked(|_, _| {
                println!("Button Clicked!");
            })
            .release();

        push_button.show();
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
