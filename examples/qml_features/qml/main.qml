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

        clip: true
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
                    source: "pages/PropertiesPage.qml"
                }
                ListElement {
                    name: "Invokables"
                    source: "pages/InvokablesPage.qml"
                }
                ListElement {
                    name: "Signals"
                    source: "pages/SignalsPage.qml"
                }
                ListElement {
                    name: "Threading"
                    source: "pages/ThreadingPage.qml"
                }
                ListElement {
                    name: "Custom Base Class"
                    source: "pages/CustomBaseClassPage.qml"
                }
                ListElement {
                    name: "Serialisation"
                    source: "pages/SerialisationPage.qml"
                }
                ListElement {
                    name: "Types"
                    source: "pages/TypesPage.qml"
                }
                ListElement {
                    name: "Containers"
                    source: "pages/ContainersPage.qml"
                }
                ListElement {
                    name: "Multiple QObjects"
                    source: "pages/MultipleQObjectsPage.qml"
                }
                ListElement {
                    name: "Naming"
                    source: "pages/NamingPage.qml"
                }
                ListElement {
                    name: "Nested QObjects"
                    source: "pages/NestedQObjectsPage.qml"
                }
                ListElement {
                    name: "Singleton"
                    source: "pages/SingletonPage.qml"
                }
                ListElement {
                    name: "Custom Parent Class"
                    source: "pages/CustomParentClassPage.qml"
                }
                ListElement {
                    name: "ExternCxxQt"
                    source: "pages/ExternCxxQtPage.qml"
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
