import QtQuick 2.12
import QtGraphicalEffects 1.15


Item{
    id: sensorUI
    width: 61
    height: 61
    readonly property real __value: value/(max)
    property int max: 3500
    property real value: 2500
    property bool online: true
    Behavior on value {
        NumberAnimation {
            duration: 6200
            easing.type: Easing.InOutQuad
        }
        enabled: parent.opacity===1
    }
    opacity: online? 1 : 0
    Behavior on opacity {
        NumberAnimation {
            duration: 600
            easing.type: Easing.InOutQuad
        }
    }

    Image {
        source: "./images/sensor.png"
        anchors.verticalCenter: parent.verticalCenter
        x:-6
    }
    Item {
        id: efect
        visible: false
        anchors.fill: parent
        anchors.margins: -15
        Image {
            source: "./images/sensorefect.png"
            anchors.verticalCenter: parent.verticalCenter
            x:15
        }

    }
    ConicalGradient {
        id: angle
        anchors.margins: -15
        anchors.fill: parent
        angle: 153
        gradient: Gradient {
            GradientStop { position: 0.0; color: "transparent" }
            GradientStop { position: (1 - __value*0.35); color: "transparent" }
            GradientStop { position: (1 - __value*0.3491); color: "#87d9f6" }
            GradientStop { position: 1 ; color: "#4032abb5" }
        }
        visible: false
    }
    OpacityMask {
        anchors.fill: efect
        maskSource: efect
        source: angle
    }
    Text {
        id: wh
        text: (Math.ceil(value/10)*10).toString().replace(/\B(?=(\d{3})+(?!\d))/g, " " )+" <i>Wh</i>"
        font.family: "Open Sans"
        styleColor: "#003362"
        style: Text.Outline
        anchors.verticalCenter: parent.verticalCenter
        font.pixelSize: 11
        anchors.verticalCenterOffset: 12
        anchors.horizontalCenterOffset: 0
        anchors.horizontalCenter: parent.horizontalCenter
        color: "#a9deff"
        Behavior on color {ColorAnimation {duration: 200}}
        z:4
    }

}
