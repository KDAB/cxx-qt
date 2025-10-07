// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_main_qml
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// ANCHOR: book_qml_import
// This must match the uri and version
// specified in the qml_module in the build.rs script.
import com.kdab.cxx_qt.demo 1.0
// ANCHOR_END: book_qml_import

ApplicationWindow {
    id: root
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640
    color: palette.window

    readonly property MyObject myObject: MyObject {
        number: 1
        string: qsTr("My String with my number: %1").arg(number)
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: qsTr("Number: %1").arg(root.myObject.number)
            color: palette.text
        }

        Label {
            text: qsTr("String: %1").arg(root.myObject.string)
            color: palette.text
        }

        Button {
            text: qsTr("Increment Number")

            onClicked: root.myObject.incrementNumber()
        }

        Button {
            text: qsTr("Say Hi!")

            onClicked: root.myObject.sayHi(root.myObject.string, root.myObject.number)
        }

        Button {
            text: qsTr("Quit")

            onClicked: Qt.quit()
        }
    }
}
// ANCHOR_END: book_main_qml
