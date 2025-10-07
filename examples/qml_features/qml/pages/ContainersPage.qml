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
                text: qsTr("Reset")

                onClicked: root.rustContainers.reset()
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    readonly property RustContainers rustContainers: RustContainers {
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter
        anchors.margins: 5

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
                text: qsTr("QHash<QString, QVariant> values: %1").arg(root.rustContainers.stringHash || "Empty")
                wrapMode: Text.Wrap
            }

            Button {
                text: qsTr("Insert")

                onClicked: root.rustContainers.insertHash("Key" + spinBox.value, spinBox.value)
            }
        }

        RowLayout {
            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                text: qsTr("QList<i32> values: %1").arg(root.rustContainers.stringList || "Empty")
                wrapMode: Text.Wrap
            }

            Button {
                text: qsTr("Append")

                onClicked: root.rustContainers.appendList(spinBox.value)
            }
        }

        RowLayout {
            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                text: qsTr("QMap<QString, QVariant> values: %1").arg(root.rustContainers.stringMap || "Empty")
                wrapMode: Text.Wrap
            }

            Button {
                text: qsTr("Insert")

                onClicked: root.rustContainers.insertMap("Key" + spinBox.value, spinBox.value)
            }
        }

        RowLayout {
            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                text: qsTr("QSet<i32> values: %1").arg(root.rustContainers.stringSet || "Empty")
                wrapMode: Text.Wrap
            }

            Button {
                text: qsTr("Insert")

                onClicked: root.rustContainers.insertSet(spinBox.value)
            }
        }

        RowLayout {
            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                text: qsTr("QVector<i32> values: %1").arg(root.rustContainers.stringVector || "Empty")
                wrapMode: Text.Wrap
            }

            Button {
                text: qsTr("Append")

                onClicked: root.rustContainers.appendVector(spinBox.value)
            }
        }
    }
}
