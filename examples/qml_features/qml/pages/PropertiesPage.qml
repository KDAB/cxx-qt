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
                enabled: root.rustProperties.connected
                text: qsTr("Disconnect")

                onClicked: root.rustProperties.connectedUrl = undefined
            }

            ToolButton {
                enabled: !root.rustProperties.connected
                text: qsTr("Connect")

                onClicked: root.rustProperties.connectedUrl = urlTextField.text
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    readonly property RustProperties rustProperties: RustProperties {
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("Properties can be used from Rust to indicate state to QML as normal.")
            wrapMode: Text.Wrap
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("In this demo connecting to a page from https://kdab.com will result in a successful connection, otherwise an error will occur.")
            wrapMode: Text.Wrap
        }

        TextField {
            id: urlTextField
            Layout.alignment: Qt.AlignHCenter
            selectByMouse: true
            placeholderText: qsTr("Enter a URL")
        }

        Label {
            id: statusLabel
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: root.rustProperties.connected ? qsTr("%1: %2").arg(root.rustProperties.statusMessage).arg(root.rustProperties.connectedUrl) : root.rustProperties.statusMessage
            wrapMode: Text.Wrap
        }
    }
}
