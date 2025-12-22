// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

import com.kdab.cxx_qt.demo 1.0

Page {
    id: root
    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            ToolButton {
                text: qsTr("Increment First")

                onClicked: root.first.increment()
            }

            ToolButton {
                text: qsTr("Increment Second")

                onClicked: root.second.increment()
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    readonly property FirstObject first: FirstObject {
        onAccepted: console.warn("First Accepted")
        onRejected: console.warn("First Rejected")
    }

    readonly property SecondObject second: SecondObject {
        onAccepted: console.warn("Second Accepted")
        onRejected: console.warn("Second Rejected")
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("Multiple QObjects can be defined in a single CXX-Qt bridge macro and used as normal.")
            wrapMode: Text.Wrap
        }

        Label {
            color: root.first.color
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("First Object Count: %1, color: %2").arg(root.first.counter).arg(root.first.color)
            wrapMode: Text.Wrap
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("Second Object Count: %1, URL: %2").arg(root.second.counter).arg(root.second.url)
            wrapMode: Text.Wrap
        }
    }
}
