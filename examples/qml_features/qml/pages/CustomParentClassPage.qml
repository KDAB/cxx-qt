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

        // TODO: qmllint fails here as in the qmltypes CustomParentClass
        // is missing a prototype of QQuickPaintedItem it is not yet clear why
        // this is missed.
        //
        // qmltyperegistrar claims the following
        // QQuickPaintedItem is used as base type but cannot be found.
        //
        // The type QQuickPaintedItem is not embedded in the moc JSON,
        // does it come from elsewhere?
        //
        // qmllint disable incompatible-type Quick.attached-property-type
        CustomParentClass {
            id: customPainter
            color: "red"
            Layout.alignment: Qt.AlignHCenter
            Layout.preferredHeight: 200
            Layout.preferredWidth: 200
        }

        // qmllint enable incompatible-type Quick.attached-property-type

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("In this demo the Rectangle is rendered in Rust by implementing a QQuickPaintedItem.")
            wrapMode: Text.Wrap
        }
    }
}
