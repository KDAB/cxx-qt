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
    background: Rectangle {
        color: privateState.color
    }
    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            ToolButton {
                enabled: !timerSync.running
                text: qsTr("Read Color")

                onClicked: privateState.load()
            }

            ToolButton {
                checkable: true
                checked: timerSync.running
                text: qsTr("Auto Read")

                onClicked: timerSync.running = !timerSync.running
            }

            ToolButton {
                text: qsTr("Reset")

                onClicked: {
                    timerSync.running = false;
                    root.rustInvokables.reset();
                    privateState.load();
                }
            }

            // ANCHOR: book_namespaced_qenum
            ToolButton {
                text: qsTr("Red")
                onClicked: root.rustInvokables.storeColorWithEnum(Colors.Red);
            }
            // ANCHOR_END: book_namespaced_qenum

            ToolButton {
                text: qsTr("Green")
                onClicked: root.rustInvokables.storeColorWithEnum(Colors.Green);
            }

            ToolButton {
                text: qsTr("Blue")
                onClicked: root.rustInvokables.storeColorWithEnum(Colors.Blue);
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    readonly property RustInvokables rustInvokables: RustInvokables {
    }

    QtObject {
        id: privateState

        property color color
        property bool loaded: false

        function load() {
            color = root.rustInvokables.loadColor();
        }

        Component.onCompleted: {
            load();
            loaded = true;
        }
    }

    Timer {
        id: timerSync
        interval: 16
        repeat: true

        onTriggered: privateState.load()
    }

    ColumnLayout {
        anchors.centerIn: parent

        function storeColor() {
            if (!privateState.loaded) {
                return;
            }
            root.rustInvokables.storeColor(sliderRed.value, sliderGreen.value, sliderBlue.value);
        }

        Slider {
            id: sliderRed
            from: 0
            value: privateState.color.r
            to: 1

            onValueChanged: parent.storeColor()
        }

        Slider {
            id: sliderGreen
            from: 0
            value: privateState.color.g
            to: 1

            onValueChanged: parent.storeColor()
        }

        Slider {
            id: sliderBlue
            from: 0
            value: privateState.color.b
            to: 1

            onValueChanged: parent.storeColor()
        }
    }
}
