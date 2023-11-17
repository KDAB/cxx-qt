// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12

import com.kdab.cxx_qt.demo 1.0
import com.kdab.cxx_qt.demo_cpp 1.0

Page {
    property int amount: 5

    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            ToolButton {
                text: qsTr("Trigger")

                onClicked: rustExternCxxQt.triggerOnExternal(externalQObject, amountSpinBox.value)
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    ExternalQObject {
        id: externalQObject
    }

    ExternalCxxQtHelper {
        id: rustExternCxxQt
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("CXX-Qt supports closures when connecting to signals.\n" +
                       "In this example the 'Trigger' button can be used to trigger a signal.\n" +
                       "A different object is captured by the invoked closure and this object increments the count.\n" +
                       "This way easy bindings between objects are possible with CXX-Qt.")
            wrapMode: Text.Wrap
        }

        SpinBox {
            id: amountSpinBox
            Layout.alignment: Qt.AlignHCenter
            from: 1
            to: 10
            value: 5
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("Count: %1").arg(rustExternCxxQt.count)
            wrapMode: Text.Wrap
        }
    }

    Component.onCompleted: rustExternCxxQt.connectToExternal(externalQObject)
}
