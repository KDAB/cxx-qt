// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Nuno Pinheiro <nuno@kdab.com>

// SPDX-License-Identifier: MIT OR Apache-2.0

import QtQuick 2.12

import "qrc:/compat" as Compat

Item {
    id: sensorUI

    readonly property real __value: value / max
    property int max: 3500
    property real value: 2500
    property bool online: true

    width: 61
    height: 61
    opacity: online ? 1 : 0

    Behavior on value {
        NumberAnimation {
            // Needs to be related to the frequency of sensor updates
            duration: 1440
            easing.type: Easing.InOutQuad
        }
        // enabled: parent.opacity === 1
    }

    Behavior on opacity {
        NumberAnimation {
            duration: 600
            easing.type: Easing.InOutQuad
        }
    }

    Image {
        source: "qrc:/images/sensor.png"
        anchors.verticalCenter: parent.verticalCenter
        x:-6
    }

    Item {
        id: efect
        visible: false
        anchors.fill: parent
        anchors.margins: -15

        Image {
            source: "qrc:/images/sensorefect.png"
            anchors.verticalCenter: parent.verticalCenter
            x:15
        }
    }

    Compat.ConicalGradient {
        id: angle
        anchors.margins: -15
        anchors.fill: parent
        angle: 153
        gradient: Gradient {
            GradientStop { position: 0.0; color: "transparent" }
            GradientStop { position: (1 - (__value * 0.35)); color: "transparent" }
            GradientStop { position: (1 - (__value * 0.3491)); color: "white" }
            GradientStop { position: 1 ; color: "#4032abb5" }
        }
        visible: false
    }

    Compat.OpacityMask {
        anchors.fill: efect
        maskSource: efect
        source: angle
    }

    Text {
        id: wh
        text: "%1 W".arg(value.toPrecision(4))
        font.family: "Open Sans"
        styleColor: "#003362"
        style: Text.Outline
        anchors.verticalCenter: parent.verticalCenter
        font.pixelSize: 11
        anchors.verticalCenterOffset: 12
        anchors.horizontalCenterOffset: 0
        anchors.horizontalCenter: parent.horizontalCenter
        color: "#a9deff"
        z:4

        Behavior on color { ColorAnimation { duration: 200 } }
    }
}
