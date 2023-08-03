// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Nuno Pinheiro <nuno@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12

Item {
    id: button
    property bool activeF: true
    property alias pressed: mArea.pressed
    property alias text: centerT.text
    property alias sidetext: sideT.text

    signal clicked()

    width: 52
    height: 52

    Image {
        id: inner
        anchors.centerIn: parent
        source: "../images/activeInner.png"
        opacity: activeF || pressed ? 1 : 0
        Behavior on opacity { NumberAnimation { easing.type: Easing.InOutQuad } }
    }

    Image {
        id: innerInactive
        anchors.centerIn: parent
        source: "../images/inactiveInner.png"
        opacity: 1 - inner.opacity
    }

    Item {
        id: outerConteiner
        anchors.fill: parent
        scale: pressed ? 0.76 : 1
        opacity: Math.pow(scale, 4)

        Behavior on scale { NumberAnimation { duration: 350; easing.type: Easing.OutBack } }

        Image {
            id: outer
            source: "../images/activeOuter.png"
            anchors.centerIn: parent
            opacity: activeF || pressed ? 1 : 0
            Behavior on opacity { NumberAnimation { easing.type: Easing.InOutQuad } }
        }

        Image {
            id: outerInactive
            source: "../images/inactiveOuter.png"
            anchors.centerIn: parent
            opacity: 1 - outer.opacity
        }
    }

    Text {
        id: centerT
        text: ""
        anchors.verticalCenter: parent.verticalCenter
        font.pixelSize: 28
        anchors.verticalCenterOffset: 4
        anchors.horizontalCenter: parent.horizontalCenter
        color: activeF || pressed ? "#a9deff" : "#1b597f"

        Behavior on color { ColorAnimation { duration: 200 } }
    }

    Text {
        id: sideT
        text: ""
        styleColor: "#003362"
        style: Text.Outline
        anchors.verticalCenter: parent.verticalCenter
        font.pixelSize: 16
        anchors.horizontalCenterOffset: 27
        anchors.verticalCenterOffset: 26
        anchors.horizontalCenter: parent.horizontalCenter
        color: "#a9deff"
        z: 4

        Behavior on color { ColorAnimation { duration: 200 } }
    }

    Rectangle {
        id: efect
        anchors.fill: parent
        anchors.margins: -7
        color: "transparent"
        border.color: "#40a9deff"
        radius: height/2
        z: -1
        opacity: 0

        ParallelAnimation {
            id: efectAnim

            SequentialAnimation {
                PauseAnimation {
                    duration: 50
                }
                NumberAnimation {
                    target: efect
                    property: "anchors.margins"
                    duration: 600
                    easing.type: Easing.OutCubic
                    from: -7
                    to: -25
                }
            }
            SequentialAnimation {
                PauseAnimation {
                    duration: 50
                }
                NumberAnimation {
                    target: efect
                    property: "opacity"
                    duration: 20
                    easing.type: Easing.InOutQuad
                    from: 0
                    to: 1
                }
                NumberAnimation {
                    target: efect
                    property: "opacity"
                    duration: 660
                    easing.type: Easing.InOutQuad
                    from: 1
                    to: 0
                }
            }
        }
    }

    MouseArea {
        id: mArea
        anchors.centerIn: parent
        height: 100
        width: 100

        onClicked: {
            button.clicked();
            efectAnim.restart();
        }
    }
}
