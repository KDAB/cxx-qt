<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Qt

The main purpose of CXX-Qt is to expose Qt's extensions to the C++ language to CXX.

## Invokables

Invokables can be defined using the [QObject Struct](../qobject/qobject_struct.md), these will be exposed as methods on the C++ class with `Q_INVOKABLE` so that they are accessible for QML too.

## Properties

Properties can be defined using the [QObject struct](../qobject/qobject_struct.md), these will be exposed as a getter and setter method, a changed signal, and a `Q_PROPERTY` on the C++ class and therefore as QML properties too.

## Signals

Signals can be defined using the [QSignals macros](../qobject/signals.md), these will be exposed as `Q_SIGNALS` on the C++ class and therefore to QML as well.
