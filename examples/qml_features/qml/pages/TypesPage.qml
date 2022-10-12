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
                text: qsTr("Toggle Boolean")

                onClicked: types.toggleBoolean()
            }

            ToolButton {
                text: qsTr("Load from Variant")

                property int counter: 0
                property var booleanVariant: types.boolean
                property var pointVariant: types.point
                property url url: types.url
                readonly property var urlVariant: url

                onClicked: {
                    types.loadFromVariant((() => {
                        switch (counter) {
                            case 0:
                                booleanVariant = !types.boolean;
                                return booleanVariant;
                            case 1:
                                pointVariant = Qt.point(types.point.x + 1, types.point.y + 1);
                                return pointVariant;
                            case 2:
                                url = types.url == "https://kdab.com" ? "https://github.com/kdab/cxx-qt" : "https://kdab.com"
                                return urlVariant;
                            default:
                                return null;
                        }
                    })());

                    counter = (counter + 1) % 3;
                }
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    Types {
        id: types
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("Boolean: %1").arg(types.boolean)
            wrapMode: Text.Wrap
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("QPoint x: %1, y: %2").arg(types.point.x).arg(types.point.y)
            wrapMode: Text.Wrap
        }

        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: qsTr("QUrl: %1").arg(types.url)
            wrapMode: Text.Wrap
        }
    }
}
