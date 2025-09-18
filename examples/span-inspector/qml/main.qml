// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import QtQuick
import QtQuick.Controls
import QtQuick.Window
import QtQuick.Layouts

import com.kdab.cxx_qt.span_inspector 1.0

ApplicationWindow {
    id: appWindow
    color: palette.window
    property color textColor: color.lightness < 128 * "black", "white"
    height: 480
    title: qsTr("Span Inspector")
    visible: true
    width: 640

    SpanInspector {
        id: inspector
    }

    SplitView {
        anchors.fill: parent
        Item {
            SplitView.preferredWidth: parent.width / 2
            TextArea {
                id: inputEdit
                SplitView.preferredWidth: parent.width / 2
                wrapMode: TextArea.Wrap
                anchors.fill: parent
                clip: true
                color: appWindow.textColor
                Component.onCompleted: {
                    inspector.input = textDocument;
                    inspector.rebuildOutput(cursorPosition);
                }

                onCursorPositionChanged: {
                    inspector.rebuildOutput(cursorPosition);
                }

                onTextChanged: {
                    inspector.rebuildOutput(cursorPosition);
                }
            }
        }

        ScrollView {
            SplitView.preferredWidth: parent.width / 2
            TextEdit {
                clip: true
                color: appWindow.textColor
                text: "Hello World"
                readOnly: true
                Component.onCompleted: inspector.output = textDocument
            }
        }
    }
}
