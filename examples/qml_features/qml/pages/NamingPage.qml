// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

import com.kdab.cxx_qt.demo 1.0

Page {
    id: root
    readonly property RenamedObject renamedObject: RenamedObject {
        numberProp: 1
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 15

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("Counter: %1").arg(root.renamedObject.numberProp)
            wrapMode: Text.Wrap
        }

        Button {
            text: qsTr("Increment Counter")
            onClicked: root.renamedObject.increment()
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("Meaning of life: %1").arg(root.renamedObject.getNum())
            wrapMode: Text.Wrap
        }
    }
}
