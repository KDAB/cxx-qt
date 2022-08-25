// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

import com.kdab.cxx_qt.demo 1.0

Window {
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640

    MyObject {
        id: myObject
        number: 1
        string: "My String with my number: " + myObject.number
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: "Number: " + myObject.number
        }

        Label {
            text: "String: " + myObject.string
        }

        Button {
            text: "Increment Number"

            onClicked: myObject.incrementNumber()
        }

        Button {
            text: "Say Hi!"

            onClicked: myObject.sayHi(myObject.string, myObject.number)
        }
    }
}
