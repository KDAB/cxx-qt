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
    readonly property var activeModel: tabs.currentIndex === 0 ? customBaseClass : transitiveInheritance
    readonly property var activeListView: tabs.currentIndex === 0 ? customBaseList : transitiveList

    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            ToolButton {
                text: qsTr("Log")

                onClicked: root.activeModel.log()
            }

            ToolButton {
                text: qsTr("Add Row")

                onClicked: root.activeModel.add()
            }

            ToolButton {
                text: qsTr("Add in background")

                onClicked: root.activeModel.addOnThread(5)
            }

            ToolButton {
                enabled: root.activeListView.currentIndex > -1 && root.activeListView.count > 0
                text: qsTr("Double Selected")

                onClicked: root.activeModel.multiply(root.activeListView.currentIndex, 2.0)
            }

            ToolButton {
                enabled: root.activeListView.currentIndex > -1 && root.activeListView.count > 0
                text: qsTr("Remove Selected")

                onClicked: root.activeModel.remove(root.activeListView.currentIndex)
            }

            ToolButton {
                enabled: root.activeListView.count > 0
                text: qsTr("Clear")

                onClicked: root.activeModel.clear()
            }

            Item {
                Layout.fillWidth: true
            }

            Label {
                text: qsTr("Count: %1").arg(root.activeListView.count)
            }
        }
    }

    ColumnLayout {
        id: content
        anchors.fill: parent

        TabBar {
            id: tabs
            Layout.fillWidth: true
            TabButton {
                text: qsTr("Default Model")
            }
            TabButton {
                text: qsTr("Shorter Delay")
            }
        }

        StackLayout {
            Layout.fillHeight: true
            Layout.fillWidth: true

            currentIndex: tabs.currentIndex

            ScrollView {
                clip: true
                ScrollBar.vertical.policy: ScrollBar.vertical.size === 1.0 ? ScrollBar.AlwaysOff : ScrollBar.AlwaysOn

                ListView {
                    id: customBaseList
                    currentIndex: -1
                    model: CustomBaseClass {
                        id: customBaseClass
                    }
                    delegate: ItemDelegate {
                        required property int id
                        required property int index
                        required property double value

                        highlighted: ListView.isCurrentItem
                        text: id + ": " + value
                        width: ListView.view.width

                        onClicked: ListView.view.currentIndex = index
                    }
                }
            }

            ScrollView {
                clip: true
                ScrollBar.vertical.policy: ScrollBar.vertical.size === 1.0 ? ScrollBar.AlwaysOff : ScrollBar.AlwaysOn

                ListView {
                    id: transitiveList
                    currentIndex: -1
                    model: TransitiveInheritance {
                        id: transitiveInheritance
                    }
                    delegate: ItemDelegate {
                        required property int id
                        required property int index
                        required property double value

                        highlighted: ListView.isCurrentItem
                        text: id + ": " + value
                        width: ListView.view.width

                        onClicked: ListView.view.currentIndex = index
                    }
                }
            }
        }
    }

    // ANCHOR: book_qenum_access
    BusyIndicator {
        anchors {
            right: content.right
            bottom: content.bottom
            margins: 15
        }
        running: root.activeModel.state === CustomBaseClass.Running
    }
    // ANCHOR_END: book_qenum_access
}
