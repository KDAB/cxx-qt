// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12

ApplicationWindow {
    id: window
    minimumHeight: 480
    minimumWidth: 640
    title: qsTr("CXX-Qt: QML Features")
    visible: true

    header: RowLayout {
        anchors.left: parent.left
        anchors.right: parent.right

        Button {
            flat: true
            font.pixelSize: 24
            text: drawer.position == 1.0 ? "✕" : "☰"
            Layout.preferredWidth: height

            onClicked: {
                if (drawer.position == 1.0) {
                    drawer.close();
                } else {
                    drawer.open();
                }
            }
        }

        Label {
            text: drawer.currentItem.text
            font.pixelSize: 24
            Layout.fillWidth: true
        }
    }

    Drawer {
        id: drawer

        readonly property alias currentItem: drawerView.currentItem

        height: window.height - header.height
        width: Math.min(window.width * 0.66, 200)
        y: header.height

        ListView {
            id: drawerView
            anchors.fill: parent
            currentIndex: 0
            delegate: ItemDelegate {
                highlighted: ListView.isCurrentItem
                text: model.name
                width: ListView.view.width

                readonly property string source: model.source

                onClicked: {
                    ListView.view.currentIndex = index;
                    drawer.close();
                }
            }
            model: ListModel {
                ListElement {
                    name: "Properties"
                    source: "qrc:/pages/PropertiesPage.qml"
                }
                ListElement {
                    name: "Invokables"
                    source: "qrc:/pages/InvokablesPage.qml"
                }
                ListElement {
                    name: "Signals"
                    source: "qrc:/pages/SignalsPage.qml"
                }
                ListElement {
                    name: "Threading"
                    source: "qrc:/pages/ThreadingPage.qml"
                }
                ListElement {
                    name: "Custom Base Class"
                    source: "qrc:/pages/CustomBaseClassPage.qml"
                }
                ListElement {
                    name: "Serialisation"
                    source: "qrc:/pages/SerialisationPage.qml"
                }
                ListElement {
                    name: "Types"
                    source: "qrc:/pages/TypesPage.qml"
                }
            }
        }
    }

    Loader {
        anchors.fill: parent
        asynchronous: true
        source: drawer.currentItem.source
    }
}
