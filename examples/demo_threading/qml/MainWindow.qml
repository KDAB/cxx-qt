// SPDX-FileCopyrightText: 2021, 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Nuno Pinheiro <nuno@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Window 2.12

import com.kdab.energy 1.0
import com.kdab.energy_cpp 1.0

Window {
    id: root
    height: 720
    title: qsTr("CXX-Qt - Beach House Demo")
    visibility: Window.FullScreen
    visible: true
    width: 1280

    property int currentLevel: 1

    EnergyUsageProxyModel {
        id: energyModel
        sourceModel: energyUsage
    }

    EnergyUsage {
        id: energyUsage
    }

    Image {
        id: background
        source: "../images/bg.png"
    }

    Image {
        id: ocean
        x: Math.max(1008 - ((slideHouse.contentX - 600) / 9.2), 1008)
        y: -1
        source: "../images/ocean.png"
    }

    Image {
        id: beach1
        x: 955 - ((slideHouse.contentX - 600) / 7.9)
        y: -1
        source: "../images/beach1.png"
        width: sourceSize.width * ((slideHouse.contentX / 6500) + 0.9)
        smooth: true
    }

    Image {
        id: beach2
        x:  890 - (slideHouse.contentX - 600) / 6.9
        y: -1
        source: "../images/beach2.png"
        width: sourceSize.width * ((slideHouse.contentX / 5000) + 0.8)
        smooth: true
    }

    Image {
        id: level0
        x: Math.min(0, -168 - (slideHouse.contentX - 600) / 5.9)
        y: -128
        source: "../images/level0.png"
        opacity: currentLevel=== 0 ? 1 : 0

        Behavior on opacity { NumberAnimation { duration: 900; easing.type: Easing.InOutQuad } }

        SensorUI {
            x: 895
            y: 375
            max: 200
            value: s01.power
            online: s01.online
        }
    }

    Image {
        id: level0i
        x: level0.x
        y: 0
        source: "../images/level0i.png"
        opacity: 1 - Math.pow(level0.opacity, 2)
    }
    Image {
        id: level1
        property int xi: currentLevel > 0 ? 0 : -200
        property real opi: currentLevel > 1 ? 0 : 1
        x: level0.x + 86 + xi
        source: "../images/level1.png"
        y: 79
        opacity: (1 + (xi / 200)) * opi

        Behavior on opi { NumberAnimation { duration: 900; easing.type: Easing.InOutQuad } }
        Behavior on xi { NumberAnimation { easing.overshoot: 1.06; duration: 900; easing.type: Easing.OutExpo } }

        SensorUI {
            x: 205
            y: 275
            max: 2000
            value: s11.power
            online: s11.online
        }
        SensorUI {
            x: 620
            y: 265
            max: 20
            value: s12.power
            online: s12.online
        }
        SensorUI {
            x: 310
            y: 165
            max: 1500
            value: s13.power
            online: s13.online
        }
    }
    Image {
        id: level1i
        x: level1.x
        source: "../images/level1i.png"
        y: 79
        opacity: currentLevel === 0 ? 0 : (1 - Math.pow(level1.opacity, 2))
    }
    Image {
        id: level2
        property int xi: currentLevel > 1 ? 0 : -200
        property real opi: currentLevel > 2 ? 0 : 1
        x: level0.x + 86 + xi
        source: "../images/level2.png"
        y: 79
        opacity: (1 + xi / 200) * opi

        Behavior on xi { NumberAnimation { easing.overshoot: 1.06; duration: 900; easing.type: Easing.OutExpo } }
        Behavior on opi { NumberAnimation { duration: 900; easing.type: Easing.InOutQuad } }

        SensorUI {
            x: 85
            y: 320
            max: 100
            value: s21.power
            online: s21.online
        }
        SensorUI {
            x: 520
            y: 320
            max: 150
            value: s22.power
            online: s22.online
        }
        SensorUI {
            x: 270
            y: 110
            max: 120
            value: s23.power
            online: s23.online
        }
        SensorUI {
            x: 450
            y: 185
            max: 350
            value: s24.power
            online: s24.online
        }
    }

    Image {
        id: level2i
        x: level1.x
        source: "../images/level2i.png"
        y: 79
        opacity: currentLevel < 2 ? 0 : (1 - Math.pow(level2.opacity, 2))
    }

    Image {
        id: level3
        property int xi: currentLevel > 2 ? 0: -200
        x: level0.x + 86 + xi
        source: "../images/level3.png"
        y: 79
        opacity: (1 + (xi / 200))

        Behavior on xi { NumberAnimation { easing.overshoot: 1.06; duration: 900; easing.type: Easing.OutExpo } }

        SensorUI {
            id: sensorUI
            x: 60
            y: 300
            max: 200
            value: s31.power
            online: s31.online
        }
    }
    Image {
        id: sideShadow
        source: "../images/sideshadow.png"
    }

    Flickable {
        id: slideHouse
        anchors.fill: parent
        contentHeight: height
        contentWidth: width + 600
        //contentX: 100
        flickDeceleration: 7000
    }

    Column {
        x: 19
        y: 635 - (currentLevel * 8)

        Behavior on y {
            NumberAnimation {
                duration: 400
                easing.type: Easing.InOutQuad
            }
        }
        spacing: 4

        SideText {
            text: qsTr("Beach access")
            scale: currentLevel === 0 ? 1.3 : 1
        }

        SideText  {
            text: qsTr("First floor")
            scale: currentLevel === 1 ? 1.3 : 1
        }

        SideText  {
            text: qsTr("Second floor")
            scale: currentLevel === 2 ? 1.3 : 1
        }

        SideText  {
            text: qsTr("Roof and Road")
            scale: currentLevel === 3 ? 1.3 : 1
        }
    }

    // For now these sensors are fixed from uuids, later this can be listmodel driven
    // with location and floor as roles on the model
    Sensor {
        id: s01
        model: energyModel
        uuid: "45b0836f-ae80-46f4-a311-0044cdf26e3d"
    }
    Sensor {
        id:s11
        model: energyModel
        uuid: "acd42f5b-6056-4310-a746-7a8d9ebe7127"
    }
    Sensor {
        id:s12
        model: energyModel
        uuid: "1f085cca-4008-4784-87c7-f6c21ac0369f"
    }
    Sensor {
        id:s13
        model: energyModel
        uuid: "c48169e3-9b7f-4c51-8838-2e027c85ead3"
    }
    Sensor {
        id:s21
        model: energyModel
        uuid: "452ba07a-f798-4f82-b76e-0fb11b926cf4"
    }
    Sensor {
        id:s22
        model: energyModel
        uuid: "5c293d20-870b-4f85-8e71-49eebb34bf3e"
    }
    Sensor {
        id:s23
        model: energyModel
        uuid: "ee1c3343-83ed-43b0-98d1-8d59cd7291ae"
    }
    Sensor {
        id:s24
        model: energyModel
        uuid: "fb7f706d-ee88-41a7-862b-4002fdeb9fc8"
    }
    Sensor {
        id:s31
        model: energyModel
        uuid: "3e3f1174-6aaf-4357-93ac-b3d9285d7af8"
    }

    Column {
        spacing: 20
        height: 720 - 40
        width: 200
        x: 1280 - width - 20
        y: 20

        Image {
            fillMode: Image.PreserveAspectFit
            source: "../images/qt-logo.png"
            width: parent.width
        }

        Image {
            fillMode: Image.PreserveAspectFit
            source: "../images/rust-logo-white.png"
            width: parent.width
        }
    }

    Row {
        id: levelsUI
        x: 160
        y: 635
        spacing: 60

        Button {
            id: button0
            property int numberT: s01.online ? 1 : 0

            text: "0"
            sidetext: numberT === 0 ? "" : numberT
            activeF: currentLevel === 0

            onClicked: currentLevel = 0
        }

        Button {
            id: button1
            property int numberT: (s11.online ? 1 : 0) + (s12.online ? 1 : 0) + (s13.online ? 1 : 0)

            text: "1"
            sidetext: numberT === 0 ? "" : numberT
            activeF: currentLevel === 1

            onClicked: currentLevel = 1
        }

        Button {
            id: button2
            property int numberT: (s21.online ? 1 : 0) + (s22.online ? 1 : 0) + (s23.online ? 1 : 0) + (s24.online ? 1 : 0)

            sidetext: numberT===0 ? "" : numberT
            text: "2"
            activeF: currentLevel === 2

            onClicked: currentLevel = 2
        }

        Button {
            id: button3
            property int numberT: s31.online ? 1 : 0

            text: "3"
            sidetext: numberT === 0 ? "" : numberT
            activeF: currentLevel === 3

            onClicked: currentLevel = 3
        }
    }

    Image {
        id: wireless
        source: "../images/iconwirless.png"
        x: 19
        y: 22
    }

    SideText {
        id: powerusageT
        text: qsTr("Total used Power: <i>%1 kW</i> Average: <i>%2 kW</i>").arg((energyUsage.totalUse / 1000.0).toPrecision(3)).arg((energyUsage.averageUse / 1000.0).toPrecision(3))
        color: "#a9deff"
        font.pixelSize: 16
        anchors.verticalCenter: wireless.verticalCenter
        anchors.left: wireless.right
        anchors.leftMargin: 10
        font.italic: false
        font.weight: Font.Normal
    }

    Image {
        id: sensors
        source: "../images/iconSensors.png"
        anchors.left: powerusageT.right
        anchors.leftMargin: 16
        y: 22

        SideText {
            text: qsTr("nº Online Sensors: <i><b>%1</b></i>").arg(energyUsage.sensors)
            color: "#a9deff"
            font.pixelSize: 16
            anchors.verticalCenter: parent.verticalCenter
            anchors.left: parent.right
            anchors.leftMargin: 10
            font.italic: false
            font.weight: Font.Normal
        }
    }

    Button {
        id: buttoninfo
        y: levelsUI.y
        x: Math.min (750, level0.x + 940)
        activeF: true

        onClicked: panel.enabled = !panel.enabled

        Image {
            id: rLogo
            source: "../images/RLogo.png"
            anchors.centerIn: buttoninfo
            opacity: buttoninfo.pressed ? 1 : 0.7
            scale: 0.9
        }
    }

    Panel {
        id: panel
        anchors.fill: parent
    }

    MouseArea {
        anchors.fill: parent
        propagateComposedEvents: true

        onPressed: (mouse) => {
            demoTimerDelay.restart();
            mouse.accepted = false;
        }

        Timer {
            id: demoTimerDelay
            interval: 30000
            running: true
        }

        Timer {
            interval: 5000
            repeat: true
            running: !demoTimerDelay.running && !panel.enabled
            triggeredOnStart: true

            onTriggered: currentLevel = (currentLevel + 1) % 4
        }
    }
}
