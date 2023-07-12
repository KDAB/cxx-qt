// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12

import com.kdab.cxx_qt.demo 1.0

Page {
    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            ToolButton {
                text: qsTr("Disconnect")

                onClicked: rustSignals.disconnect()
            }

            ToolButton {
                text: qsTr("Connect")

                onClicked: rustSignals.connect(urlTextField.text)
            }

            ToolButton {
                checkable: true
                checked: rustSignals.loggingEnabled
                text: qsTr("Toggle Logging")

                onClicked: rustSignals.loggingEnabled = !rustSignals.loggingEnabled
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    RustSignals {
        id: rustSignals
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("Signals can be used from Rust to indicate state changes to QML as normal.")
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
            wrapMode: Text.Wrap

            Connections {
                target: rustSignals

                function onConnected(url) {
                    statusLabel.text = qsTr("Connected: %1").arg(url);
                }

                function onDisconnected() {
                    statusLabel.text = qsTr("Disconnected");
                }

                function onError(message) {
                    statusLabel.text = qsTr("Error: %1").arg(message);
                }
            }
        }
    }
}
