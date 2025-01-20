// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.12

import com.kdab.tutorial 1.0

ApplicationWindow {
  width: 640
  height: 480
  visible: true

  Greeter {
    id: greeter
  }

  RowLayout {
    anchors.fill: parent

    Label { text: qsTr("Greeting") }
    ComboBox {
      textRole: "name"
      valueRole: "value"
      onActivated: greeter.greeting = currentValue

      model: [{ name: qsTr("Hello World!"), value: Greeter.Hello },
              { name: qsTr("Bye!"), value: Greeter.Bye }]

      Component.onCompleted: currentIndex = indexOfValue(greeter.greeting)
    }

    Label { text: qsTr("Language") }
    ComboBox {
      textRole: "name"
      valueRole: "value"
      onActivated: greeter.language = currentValue

      model: [{ name: qsTr("English"), value: Greeter.English },
              { name: qsTr("French"), value: Greeter.French },
              { name: qsTr("German"), value: Greeter.German }]

      Component.onCompleted: currentIndex = indexOfValue(greeter.language)
    }

    Button {
      text: qsTr("Greet")
      onClicked: console.warn(greeter.greet())
    }
  }
}
