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
                text: qsTr("Increment")

                onClicked: outerObject.inner.counter += 1
            }


            ToolButton {
                text: qsTr("Reset")

                onClicked: outerObject.reset()
            }

            ToolButton {
                text: qsTr("Print")

                onClicked: outerObject.printCount(innerObject)
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    InnerObject {
        id: innerObject
        counter: 10
    }

    OuterObject {
        id: outerObject
        inner: innerObject

        onCalled: (inner) => console.warn("Signal called, inner value: ", inner.counter)
    }

    Label {
        anchors.centerIn: parent
        text: innerObject.counter
    }
}
