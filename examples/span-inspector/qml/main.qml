// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import QtQuick
import QtQuick.Controls
import QtQuick.Window
import QtQuick.Layouts

import com.kdab.cxx_qt.demo 1.0

ApplicationWindow {
    height: 480
    title: qsTr("Span Inspector")
    visible: true
    width: 640
    color: palette.window

    SpanInspector {
        id: inspector
    }

    SplitView {
        anchors.fill: parent
        TextEdit {
            SplitView.preferredWidth: parent.width / 2
            Component.onCompleted: {
                inspector.input = textDocument
                inspector.updateCursorPosition(cursorPosition)
            }

            onCursorPositionChanged: {
                inspector.updateCursorPosition(cursorPosition)
            }
        }
        TextEdit {
            SplitView.preferredWidth: parent.width / 2
            text: "Hello World"
            readOnly: true;
            wrapMode: TextEdit.wrap;
            Component.onCompleted: inspector.output = textDocument
        }
    }
}
