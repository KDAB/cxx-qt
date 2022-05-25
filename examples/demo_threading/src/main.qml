// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick.Window 2.12

import com.kdab.energy 1.0

Window {
    id: root
    height: 300
    title: qsTr("Energy Usage")
    visible: true
    width: 250

    EnergyUsage {
        id: energyUsage

        onSensorAdded: (uuid) => console.warn("Added", uuid, sensorPower(uuid))
        onSensorChanged: (uuid) => console.warn("Changed", uuid, sensorPower(uuid))
        onSensorRemoved: (uuid) => console.warn("Removed", uuid)

        // FIXME: have the ability to HandleInit so we can start the server
        // https://github.com/KDAB/cxx-qt/issues/13
        Component.onCompleted: startServer()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            font.pointSize: 20
            horizontalAlignment: Text.AlignHCenter
            Layout.fillWidth: true
            text: root.title
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            Label {
                horizontalAlignment: Text.AlignLeft
                Layout.fillWidth: true
                text: qsTr("Average Energy Use")
            }

            Label {
                color: energyUsage.averageUse > 50.0 ? "red" : "black"
                horizontalAlignment: Text.AlignRight
                text: qsTr("%1 kW").arg(energyUsage.averageUse)
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            Label {
                horizontalAlignment: Text.AlignLeft
                Layout.fillWidth: true
                text: qsTr("Total Energy Use")
            }

            Label {
                horizontalAlignment: Text.AlignRight
                text: qsTr("%1 kW").arg(energyUsage.totalUse)
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            Label {
                horizontalAlignment: Text.AlignLeft
                Layout.fillWidth: true
                text: qsTr("Energy Sensors")
            }

            Label {
                horizontalAlignment: Text.AlignRight
                text: energyUsage.sensors
            }
        }

        Item {
            Layout.fillHeight: true
        }
    }
}
