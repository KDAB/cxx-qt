// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "com.kdab.energy",
            rust_files: &["src/lib.rs"],
            qml_files: &[
                "../qml/Button.qml",
                "../qml/MainWindow.qml",
                "../qml/Panel.qml",
                "../qml/SensorUI.qml",
                "../qml/SideText.qml",
            ],
            qrc_files: &[
                "../images/activeInner.png",
                "../images/activeOuter.png",
                "../images/beach1.png",
                "../images/beach2.png",
                "../images/bg.png",
                "../images/iconSensors.png",
                "../images/iconwirless.png",
                "../images/inactiveInner.png",
                "../images/inactiveOuter.png",
                "../images/kdabLogo.png",
                "../images/level0.png",
                "../images/level0i.png",
                "../images/level1.png",
                "../images/level1i.png",
                "../images/level2.png",
                "../images/level2i.png",
                "../images/level3.png",
                "../images/ocean.png",
                "../images/panel.png",
                "../images/qt-logo.png",
                "../images/RLogo.png",
                "../images/RLogolarge.png",
                "../images/rust-logo-white.png",
                "../images/sensor.png",
                "../images/sensorefect.png",
                "../images/sideshadow.png",
            ],
            ..Default::default()
        })
        .with_dependency(cxx_qt_lib::cxx_qt_build_manifest())
        .build();
}
