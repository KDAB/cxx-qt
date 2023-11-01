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
                text: qsTr("Add Row")

                onClicked: customBaseClass.add()
            }

            ToolButton {
                text: qsTr("Add in background")

                onClicked: customBaseClass.addOnThread(5)
            }

            ToolButton {
                enabled: listView.currentIndex > -1 && listView.count > 0
                text: qsTr("Double Selected")

                onClicked: customBaseClass.multiply(listView.currentIndex, 2.0)
            }

            ToolButton {
                enabled: listView.currentIndex > -1 && listView.count > 0
                text: qsTr("Remove Selected")

                onClicked: customBaseClass.remove(listView.currentIndex)
            }

            ToolButton {
                enabled: listView.count > 0
                text: qsTr("Clear")

                onClicked: customBaseClass.clear()
            }

            Item {
                Layout.fillWidth: true
            }

            Label {
                text: qsTr("Count: %1").arg(listView.count)
            }
        }
    }

    ScrollView {
        id: scrollView
        anchors.fill: parent
        clip: true
        ScrollBar.vertical.policy: ScrollBar.vertical.size === 1.0 ? ScrollBar.AlwaysOff : ScrollBar.AlwaysOn

        ListView {
            id: listView
            currentIndex: -1
            model: CustomBaseClass {
                id: customBaseClass
            }
            delegate: ItemDelegate {
                highlighted: ListView.isCurrentItem
                text: model.id + ": " + model.value
                width: ListView.view.width

                onClicked: ListView.view.currentIndex = index
            }
        }
    }

    // ANCHOR: book_qenum_access
    BusyIndicator {
        anchors {
            right: scrollView.right
            bottom: scrollView.bottom
            margins: 15
        }
        running: customBaseClass.state === CustomBaseClass.Running
    }
    // ANCHOR_END: book_qenum_access
}
