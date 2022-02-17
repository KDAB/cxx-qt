// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Dialogs 1.1
import QtQuick.Window 2.12

import com.kdab.cxx_qt.demo 1.0

Window {
    height: 300
    title: qsTr("Hello World")
    visible: true
    width: 300

    MyObject {
        id: myObject
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
            text: "Increment"

            onClicked: myObject.increment()
        }

        Button {
            text: "Reset"

            onClicked: myObject.reset()
        }

        Button {
            text: "Serialize"

            onClicked: {
                serializedMessageDialog.text = myObject.serialize();
                serializedMessageDialog.open();
            }
        }
    }

    MessageDialog {
        id: serializedMessageDialog
        title: qsTr("Serialized Object")

        onAccepted: close()
    }
}
