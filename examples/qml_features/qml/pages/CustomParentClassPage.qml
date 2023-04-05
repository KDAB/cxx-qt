// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
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
                text: qsTr("Red")

                onClicked: customPainter.color = "red"
            }

            ToolButton {
                text: qsTr("Green")

                onClicked: customPainter.color = "green"
            }

            ToolButton {
                text: qsTr("Blue")

                onClicked: customPainter.color = "blue"
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }


    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter

        CustomParentClass {
            id: customPainter
            color: "red"
            Layout.alignment: Qt.AlignHCenter
            height: 200
            width: 200
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("In this demo the Rectangle is rendered in Rust by implementing a QQuickPaintedItem.")
            wrapMode: Text.Wrap
        }
    }
}
