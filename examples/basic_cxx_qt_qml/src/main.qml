// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
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

    MyData {
        id: myData
        number: myObject.number
        string: myObject.string
    }

    MyObject {
        id: myObject
        number: 1
        string: "My String " + myObject.number
        sub: subObject
    }

    SubObject {
        id: subObject
        number: 2
        string: "substr"
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: "Number: " + myObject.number + " SubNumber: " + myObject.sub.number
        }

        Label {
            text: "String: " + myObject.string + " SubString: " + myObject.sub.string
        }

        Button {
            text: "Increment Number"

            onClicked: myObject.number = myObject.incrementNumber(myObject.number)
        }

        Button {
            text: "Increment Number (self)"

            onClicked: myObject.incrementNumberSelf()
        }

        Button {
            text: "Increment Number (sub) from myObject"

            onClicked: myObject.incrementNumberSub(myObject.sub)
        }

        Button {
            text: "Increment Number (sub) from sub"

            onClicked: myObject.sub.incrementNumberSelf()
        }

        Button {
            text: "Print Data"

            onClicked: console.warn(myData.asJsonStr())
        }
    }

    Component.onCompleted: myObject.sayHi(myObject.string, myObject.number)
}
