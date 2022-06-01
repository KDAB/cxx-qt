// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12

Text {
    id: sideText

    font.family: "Open Sans"
    font.italic: true
    font.pixelSize: 12
    color: "#a9deff"
    font.weight: Font.Light
    transformOrigin: Item.Left
    height: (scale * paintedHeight)
    opacity: (scale * scale) - 0.5

    Behavior on scale {
        NumberAnimation {
            duration: 570
            easing.type: Easing.OutQuad
        }
    }
}
