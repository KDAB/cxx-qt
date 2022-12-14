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
                text: qsTr("Reset")

                onClicked: rustContainers.reset()
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

        SpinBox {
            id: spinBox
            Layout.alignment: Qt.AlignHCenter
            from: 0
            to: 100
            value: 10
        }

        RowLayout {
            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                text: qsTr("QHash<QString, QVariant> values: %1").arg(rustContainers.stringHash || "Empty")
                wrapMode: Text.Wrap
            }

            Button {
                text: qsTr("Insert")

                onClicked: rustContainers.insertHash("Key" + spinBox.value, spinBox.value)
            }
        }

        RowLayout {
            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                text: qsTr("QList<i32> values: %1").arg(rustContainers.stringList || "Empty")
                wrapMode: Text.Wrap
            }

            Button {
                text: qsTr("Append")

                onClicked: rustContainers.appendList(spinBox.value)
            }
        }

        RowLayout {
            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                text: qsTr("QMap<QString, QVariant> values: %1").arg(rustContainers.stringMap || "Empty")
                wrapMode: Text.Wrap
            }

            Button {
                text: qsTr("Insert")

                onClicked: rustContainers.insertMap("Key" + spinBox.value, spinBox.value)
            }
        }

        RowLayout {
            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                text: qsTr("QSet<i32> values: %1").arg(rustContainers.stringSet || "Empty")
                wrapMode: Text.Wrap
            }

            Button {
                text: qsTr("Insert")

                onClicked: rustContainers.insertSet(spinBox.value)
            }
        }

        RowLayout {
            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                text: qsTr("QVector<i32> values: %1").arg(rustContainers.stringVector || "Empty")
                wrapMode: Text.Wrap
            }

            Button {
                text: qsTr("Append")

                onClicked: rustContainers.appendVector(spinBox.value)
            }
        }
    }
}
