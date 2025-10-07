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
    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            ToolButton {
                text: qsTr("Serialise Values")

                onClicked: {
                    lastErrorLabel.errorMessage = "";
                    root.serialisation.number = numberSpinBox.value;
                    root.serialisation.string = stringTextField.text;
                    jsonTextField.text = root.serialisation.asJsonStr();
                }
            }

            ToolButton {
                text: qsTr("Load JSON Input")

                onClicked: {
                    lastErrorLabel.errorMessage = "";
                    root.serialisation.fromJsonStr(jsonTextField.text);
                }
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    readonly property Serialisation serialisation: Serialisation {
                onError: (message) => {
                    lastErrorLabel.errorMessage = message;
                }
    }

    GridLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter
        anchors.margins: 5
        columns: 2

        Label {
            text: qsTr("Number Property")
        }

        SpinBox {
            id: numberSpinBox

            Binding {
                target: numberSpinBox
                property: "value"
                value: root.serialisation.number
            }
        }

        Label {
            text: qsTr("String Property")
        }

        TextField {
            id: stringTextField
            Layout.fillWidth: true
            selectByMouse: true

            Binding {
                target: stringTextField
                property: "text"
                value: root.serialisation.string
            }
        }

        Label {
            text: qsTr("JSON")
        }

        TextField {
            id: jsonTextField
            Layout.fillWidth: true
            selectByMouse: true
            placeholderText: qsTr("eg {\"number\": 1, \"string\": \"KDAB!\"}")
        }

        Label {
            id: lastErrorLabel

            property string errorMessage

            horizontalAlignment: Text.AlignHCenter
            Layout.fillWidth: true
            Layout.columnSpan: 2
            text: errorMessage !== "" ? qsTr("Error: %1").arg(errorMessage) : ""
            wrapMode: Text.Wrap
        }
    }
}
