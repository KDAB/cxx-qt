// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Nuno Pinheiro <nuno@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtGraphicalEffects 1.10

Item {
    id: root

    property bool enabled: false

    Rectangle {
        anchors.fill: parent
        color: "black"
        opacity: root.enabled ? 0.5 : 0

        Behavior on opacity {
            NumberAnimation {
                duration: 800
                easing.type: Easing.OutBack
            }
        }
    }

    BorderImage {
        id: panel
        source: "./images/panel.png"
        width: 900
        clip: true
        height: parent.height - 30
        anchors.verticalCenter: parent.verticalCenter
        border.left: 0; border.top: 25
        border.right: 25; border.bottom: 25
        x: root.enabled ? -100 : -1960

        Behavior on x {
            NumberAnimation {
                easing.overshoot: 1.1
                duration: 800
                easing.type: Easing.OutBack
            }
        }

        Text {
            font.family: "Open Sans"
            font.italic: true
            font.pixelSize: 45
            color: "#a9deff"
            font.weight: Font.Light
            text: "CXX-Qt - Beach House Demo"
            x: kdabL.x
            y: 20
        }

        Image {
            id: kdabL
            source: "./images/kdabLogo.png"
            x: (-panel.x / 1.5) + 60
            y: 120
        }

        Text {
            font.family: "Open Sans"
            horizontalAlignment: Text.AlignJustify
            width: 520
            wrapMode: Text.Wrap
            font.pixelSize: 16
            font.weight: Font.Light
            color: "#a9deff"
            text: "<p><b>CXX-Qt - Safe Rust bindings for Qt</b></p>
            <p>KDAB has been working on bridging between Rust and Qt. CXX-Qt allows you to define Qt objects from Rust, so that the business logic can be written in Rust while Qt handles the frontend.</p>
            <p>In this beach house demo, Rust provides a threaded network server for sensors, accumulates data in a background thread, and safely exposes summary and sensor data to QML.</p>"
            anchors.left: kdabL.right
            anchors.leftMargin: 25
            y: kdabL.y - 10
        }

        Image {
            id: rust
            source: "./images/RLogolarge.png"
            anchors.horizontalCenter: kdabL.horizontalCenter
            anchors.top: kdabL.bottom
            anchors.topMargin: 100
        }

        Text {
            font.family: "Open Sans"
            width: 520
            wrapMode: Text.Wrap
            horizontalAlignment: Text.AlignJustify
            font.pixelSize: 16
            font.weight: Font.Light
            color: "#a9deff"
            text:"<p><b>Rust</b></p>
            <p>A programming language empowering everyone to build reliable and efficient software.</p>
            <p>Blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.</p>
            <p>Rust's rich type system and ownership model guarantee memory-safety and thread-safety - enabling you to eliminate many classes of bugs at compile-time.</p>"
            anchors.left: kdabL.right
            anchors.leftMargin: 25
            y: rust.y - 10
        }
    }

    Text {
        anchors.left: panel.right
        anchors.leftMargin: 15
        anchors.top: panel.top
        color: "#a9deff"
        font.family: "Open Sans"
        font.pixelSize: 48
        text: "x"
        textFormat: Text.RichText
    }

    MouseArea {
        anchors.fill: parent
        enabled: root.enabled

        onClicked: root.enabled = false
    }
}
