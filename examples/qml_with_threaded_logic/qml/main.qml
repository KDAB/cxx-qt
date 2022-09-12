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

    Website {
        id: manager

        onTitleChanged: newTitleValue()
        onUrlChanged: newUrlValue()
    }

    Column{
        spacing: 10
        anchors.centerIn: parent

        Text {
            text: "Url: " + manager.url
            font.pixelSize: 15
        }

        Text {
            text: "Title: " + manager.title
            font.pixelSize: 15
        }

        Button {
            text: "Change url"
            onClicked: manager.changeUrl()
            width: parent.width
        }

        Button {
            text: "Refresh title"
            onClicked: manager.refreshTitle()
            width: parent.width
        }
    }
}
