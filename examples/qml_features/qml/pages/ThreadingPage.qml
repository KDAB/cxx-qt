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
                text: qsTr("Change Url")

                onClicked: website.changeUrl()
            }

            ToolButton {
                text: qsTr("Fetch Title")

                onClicked: website.fetchTitle()
            }

            Item {
                Layout.fillWidth: true
            }
        }
    }

    ThreadingWebsite {
        id: website
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter

        Label {
            horizontalAlignment: Text.AlignHCenter
            Layout.fillWidth: true
            text: qsTr("Url: %1").arg(website.url)
            wrapMode: Text.Wrap
        }

        Label {
            horizontalAlignment: Text.AlignHCenter
            Layout.fillWidth: true
            text: qsTr("Title: %1").arg(website.title)
            wrapMode: Text.Wrap
        }
    }
}
