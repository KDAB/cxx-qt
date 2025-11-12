// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12

import com.kdab.cxx_qt.demo 1.0
import com.kdab.cxx_qt.demo.sub1 1.0
import com.kdab.cxx_qt.demo.sub2 1.0
import com.kdab.cxx_qt.demo.sub3 1.0

ApplicationWindow {
    id: window
    minimumHeight: 480
    minimumWidth: 640
    title: qsTr("CXX-Qt: Hello World")
    visible: true

    MainObject {
        id: main
    }

    Sub1Object {
        id: sub1
    }

    Sub2Object {
        id: sub2
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: "Main: " + main.string
        }

        Label {
            text: "Sub1: " + sub1.string
        }

        Label {
            text: "Sub2: " + sub2.string
        }

        Label {
            text: "Sub3: " + Singleton.string
        }

        Button {
            text: "Increment Number"

            onClicked: {
                main.increment();
                sub1.increment();
                sub2.increment();
            }
        }

        BlueRect {
            id: blueRect
        }

        RedRect {
            id: redRect
        }
    }
}
