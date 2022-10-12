// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

import com.kdab.cxx_qt.demo 1.0

Page {
    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            ToolButton {
                enabled: rustProperties.connected
                text: qsTr("Disconnect")

                onClicked: rustProperties.disconnect()
            }

            ToolButton {
                enabled: !rustProperties.connected
                text: qsTr("Connect")

                onClicked: rustProperties.connect(urlTextField.text)
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    RustProperties {
        id: rustProperties
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
            text: rustProperties.connected ? qsTr("%1: %2").arg(rustProperties.statusMessage).arg(rustProperties.connectedUrl) : rustProperties.statusMessage
            wrapMode: Text.Wrap
        }
    }
}
