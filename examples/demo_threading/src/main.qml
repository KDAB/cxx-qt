// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQuick.Window 2.12
import com.kdab.energy 1.0
import QtGraphicalEffects 1.15



Window {
    id: root
    height: 720
    title: qsTr("Rust Beach House Sensors")
    visible: true
    width: 1280
    property int currentLevel: 3
    property bool about: false

    EnergyUsageProxyModel {
        id: energyModel
        sourceModel: energyUsage
    }

    EnergyUsage {
        id: energyUsage

        onSensorAdded: (uuid) => console.warn("Added", uuid, sensorPower(uuid))
        onSensorChanged: (uuid) => console.warn("Changed", uuid, sensorPower(uuid))
        onSensorRemoved: (uuid) => console.warn("Removed", uuid)

        // FIXME: have the ability to HandleInit so we can start the server
        // https://github.com/KDAB/cxx-qt/issues/13
        Component.onCompleted: startServer()
    }
    Image {
        id: background
        source: "./images/bg.png"
    }

    Image {
        id: ocean
        x: Math.max(1008-(slideHouse.contentX-600)/9.2, 1008 )
        y: -1
        source: "./images/ocean.png"

    }
    Image {
        id: beach1
        x: 955-(slideHouse.contentX-600)/7.9
        y: -1
        source: "./images/beach1.png"
        width: sourceSize.width * ((slideHouse.contentX/6500)+0.9)
        smooth: true
    }
    Image {
        id: beach2
        x:  890-(slideHouse.contentX-600)/6.9
        y: -1
        source: "./images/beach2.png"
        width: sourceSize.width * ((slideHouse.contentX/5000)+0.8)
        smooth: true
    }
    Image {
        id: level0
        x: Math.min(0,-168-(slideHouse.contentX-600)/5.9)
        y: -128
        source: "./images/level0.png"
        opacity: currentLevel=== 0 ? 1 : 0
        Behavior on opacity {  NumberAnimation {duration: 900; easing.type: Easing.InOutQuad} }
        SensorUI {
            x: 895
            y: 375
            max: s01.max
            value: s01.power
            online: s01.online
        }
    }
    Image {
        id: level0i
        x: level0.x
        y: 0
        source: "./images/level0i.png"
        opacity: 1 - Math.pow(level0.opacity,2)
    }
    Image {
        id: level1
        property int xi: currentLevel>0? 0: -200
        property real opi: currentLevel>1? 0: 1
        Behavior on opi {  NumberAnimation {duration: 900; easing.type: Easing.InOutQuad} }
        Behavior on xi {NumberAnimation { easing.overshoot: 1.06;duration: 900;easing.type: Easing.OutExpo}}
        x: level0.x+86+xi
        source: "./images/level1.png"
        y: 79
        opacity: (1+xi/200) * opi
        SensorUI {
            x: 205
            y: 275
            max: s11.max
            value: s11.power
            online: s11.online
        }
        SensorUI {
            x: 620
            y: 265
            max: s12.max
            value: s12.power
            online: s12.online
        }
        SensorUI {
            x: 310
            y: 165
            max: s13.max
            value: s13.power
            online: s13.online
        }
    }
    Image {
        id: level1i
        x: level1.x
        source: "./images/level1i.png"
        y: 79
        opacity: currentLevel===0? 0 : (1 - Math.pow(level1.opacity,2))
    }
    Image {
        id: level2
        property int xi: currentLevel>1? 0: -200
        property real opi: currentLevel>2? 0 : 1
        Behavior on xi { NumberAnimation { easing.overshoot: 1.06;duration: 900;easing.type: Easing.OutExpo}}
        Behavior on opi { NumberAnimation {duration: 900; easing.type: Easing.InOutQuad} }
        x: level0.x+86+xi
        source: "./images/level2.png"
        y: 79
        opacity: (1+xi/200) * opi

        SensorUI {
            x: 85
            y: 320
            max: s21.max
            value: s21.power
            online: s21.online
        }
        SensorUI {
            x: 520
            y: 320
            max: s22.max
            value: s22.power
            online: s22.online
        }
        SensorUI {
            x: 270
            y: 110
            max: s23.max
            value: s23.power
            online: s23.online
        }
        SensorUI {
            x: 450
            y: 185
            max: s24.max
            value: s24.power
            online: s24.online
        }
    }
    Image {
        id: level2i
        x: level1.x
        source: "./images/level2i.png"
        y: 79
        opacity: currentLevel<2? 0 : (1 - Math.pow(level2.opacity,2))
    }

    Image {
        id: level3
        property int xi: currentLevel>2? 0: -200
        Behavior on xi {NumberAnimation { easing.overshoot: 1.06;duration: 900;easing.type: Easing.OutExpo}}
        x: level0.x+86+xi
        source: "./images/level3.png"
        y: 79
        opacity: (1+xi/200)

        SensorUI {
            id: sensorUI
            x: 60
            y: 300
            max: s31.max
            value: s31.power
            online: s31.online
        }
    }
    Image {
        id: sideShadow
        source: "./images/sideshadow.png"
    }

    Flickable {
        id: slideHouse
        anchors.fill: parent
        contentHeight: height
        contentWidth: width+600
        //contentX: 100
        flickDeceleration: 7000


    }
    Column {
        x: 19
        y: 635 - currentLevel*8
        Behavior on y {
            NumberAnimation {
                duration: 400
                easing.type: Easing.InOutQuad
            }
        }
        spacing: 4

        SideText {
            id: sideText
            text: "Beach access"
            scale: currentLevel===0? 1.3: 1
        }
        SideText  {
            text: "First floor"
            scale: currentLevel===1? 1.3: 1
            color: "#a9deff"
            font.weight: Font.Light
        }
        SideText  {
            text: "Second floor"
            scale: currentLevel===2? 1.3: 1
            color: "#a9deff"
            font.weight: Font.Light
        }

        SideText  {
            text: "Roof and Road"
            scale: currentLevel===3? 1.3: 1
            color: "#a9deff"
            font.weight: Font.Light
        }
    }

    Senso {
        id:s01
        max:1300
        min:1000
    }
    Senso {
        id:s11
        max:800
        min:400
    }
    Senso {
        id:s12
        max:1800
        min:20
    }
    Senso {
        id:s13
        max:2500
        min:20
    }
    Senso {
        id:s21
        max:500
        min:20
    }
    Senso {
        id:s22
        max:1500
        min:20
    }
    Senso {
        id:s23
        max:600
        min:320
    }
    Senso {
        id:s24
    }
    Senso {
        id:s31
        max:300
        min:50
    }

    Row {
        id: levelsUI
        x: 160
        y: 635
        spacing: 60

        Button {
            id: button0
            text: "0"
            property int numberT: s01.online? 1 : 0
            sidetext: numberT===0 ? "" : numberT
            activeF: currentLevel === 0
            onClicked: currentLevel = 0
        }
        Button {
            id: button1
            text: "1"
            property int numberT: (s11.online? 1 : 0) + (s12.online? 1 : 0) + (s13.online? 1 : 0)
            sidetext: numberT===0 ? "" : numberT
            activeF: currentLevel === 1
            onClicked: currentLevel = 1
        }
        Button {
            id: button2
            property int numberT: (s21.online? 1 : 0) + (s22.online? 1 : 0) + (s23.online? 1 : 0) + (s24.online? 1 : 0)
            sidetext: numberT===0 ? "" : numberT
            text: "2"
            activeF: currentLevel === 2
            onClicked: currentLevel = 2
        }

        Button {
            id: button3
            text: "3"
            property int numberT: s31.online? 1 : 0
            sidetext: numberT===0 ? "" : numberT
            activeF: currentLevel === 3
            onClicked: currentLevel = 3
        }
    }

    Image {
        id: wireless
        source: "./images/iconwirless.png"
        x: 19
        y: 22

    }
    SideText  {
        id: powerusageT
        text: "Total used Power: " + ("<i>%1 kW</i>").arg(energyUsage.averageUse)
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
        source: "./images/iconSensors.png"
        anchors.left: powerusageT.right
        anchors.leftMargin: 16
        y: 22
        SideText  {
            text: "nº Online Sensors: " + "<i><b>%1</i></b>".arg(button0.numberT+button1.numberT+button2.numberT+button3.numberT)  //please replace with somthing rusty
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
        x: Math.min (750,level0.x+940)
        activeF: true
        onClicked: about=!about
        Image {
            id: rLogo
            source: "./images/RLogo.png"
            anchors.centerIn: buttoninfo
            opacity: buttoninfo.pressed? 1 : 0.7
            scale: 0.9
        }
    }
    Rectangle {
        anchors.fill: parent
        color: "black"
        opacity: about? 0.5 : 0
        Behavior on opacity {
            NumberAnimation {
                duration: 800
                easing.type: Easing.OutBack
            }
        }
    }

    Panel{
        id: panel
        x: about? -100: -1960
        Behavior on x {
            NumberAnimation {
                easing.overshoot: 1.1
                duration: 800
                easing.type: Easing.OutBack
            }
        }
    }
}

