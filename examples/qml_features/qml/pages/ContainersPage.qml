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
                text: qsTr("Insert")

                onClicked: rustContainers.insert(spinBox.value)
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    RustContainers {
        id: rustContainers
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("Container types can be used from Rust to QML as normal.")
            wrapMode: Text.Wrap
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("QSet<i32> values: %1").arg(rustContainers.stringSet)
            wrapMode: Text.Wrap
            visible: rustContainers.stringSet !== ""
        }

        SpinBox {
            id: spinBox
            Layout.alignment: Qt.AlignHCenter
            from: 0
            to: 100
            value: 10
        }
    }
}
