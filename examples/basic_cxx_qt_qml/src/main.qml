import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

import com.kdab.cxx_qt.demo 1.0

Window {
    height: 480
    title: qsTr("Hello World")
    visible: true
    width: 640

    MyObject {
        id: myObject
        number: 1
        string: "My String " + myObject.number
    }

    Column {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 10

        Label {
            text: "Number: " + myObject.number
        }

        Label {
            text: "String: " + myObject.string
        }

        Button {
            text: "Increment Number"

            onClicked: myObject.number = myObject.increment_number(myObject.number)
        }
    }

    Component.onCompleted: myObject.say_hi(myObject.number, myObject.string)
}
