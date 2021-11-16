<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Qt

## Invokables

Invokables can be defined using the [RustObj Struct](../qobject/rustobj_struct.md), these will be exposed as methods on the C++ class with `Q_INVOKABLE` so that they are accessible for QML too.

## Properties

Properties can be defined using the [Data struct](../qobject/data_struct.md), these will be exposed as a getter and setter method, a changed signal, and a `Q_PROPERTY` on the C++ class and therefore as QML properties too.

## Signals

Signals can be defined using the [Signals enum](../qobject/signals_enum.md), these will be exposed as `Q_SIGNALS` on the C++ class and therefore to QML as well.

Note: signals are not implemented yet [https://github.com/KDAB/cxx-qt/issues/31](https://github.com/KDAB/cxx-qt/issues/31).

## Change events

You can listen to property changes via the [handlers](../qobject/handlers.md) available in the RustObj Struct. These handlers are called from the Qt event loop thread to remain [thread safe](./threading.md).
