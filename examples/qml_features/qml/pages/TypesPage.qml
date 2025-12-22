// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

import com.kdab.cxx_qt.demo 1.0
// C++ code is not declarative as it still supports Qt 5
// qmllint disable import
import com.kdab.cxx_qt.demo_cpp 1.0

// qmllint enable import

Page {
    id: root
    header: ToolBar {
        RowLayout {
            anchors.fill: parent

            ToolButton {
                text: qsTr("Toggle Boolean")

                onClicked: root.types.toggleBoolean()
            }

            ToolButton {
                text: qsTr("Load from Variant")

                property int counter: 0
                property var booleanVariant: root.types.boolean
                property var pointVariant: root.types.point
                property url url: root.types.url
                // C++ code is not declarative as it still supports Qt 5
                // qmllint disable import missing-property unresolved-type
                property CustomObject customObject: CustomObject {
                    value: 0
                }
                // qmllint enable import missing-property unresolved-type
                readonly property var urlVariant: url

                onClicked: {
                    root.types.loadFromVariant((() => {
                            switch (counter) {
                            case 0:
                                booleanVariant = !root.types.boolean;
                                return booleanVariant;
                            case 1:
                                pointVariant = Qt.point(root.types.point.x + 1, root.types.point.y + 1);
                                return pointVariant;
                            case 2:
                                url = root.types.url === "https://kdab.com" ? "https://github.com/kdab/cxx-qt" : "https://kdab.com";
                                return urlVariant;
                            case 3:
                                customObject.value += 1;
                                return customObject.asStruct();
                            default:
                                return null;
                            }
                        })());

                    counter = (counter + 1) % 4;
                }
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    readonly property Types types: Types {}

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("Boolean: %1").arg(root.types.boolean)
            wrapMode: Text.Wrap
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("QPoint x: %1, y: %2").arg(root.types.point.x).arg(root.types.point.y)
            wrapMode: Text.Wrap
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("QUrl: %1").arg(root.types.url)
            wrapMode: Text.Wrap
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("CustomValue: %1").arg(root.types.customValue)
            wrapMode: Text.Wrap
        }
    }
}
