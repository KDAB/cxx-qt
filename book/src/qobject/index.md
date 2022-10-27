<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# QObject

A QObject defined by CXX-Qt supports many features and is made up of quite a few parts.

This section goes into details on these.
For a simpler introduction, take a look at our [Getting-Started guide](../getting-started/index.md).

QObject Features and Parts:
  * [`#[cxx_qt::bridge]` - The macro around the module](./bridge-macro.md)
  * [`#[cxx_qt::qobject]` - Marking a Rust struct as a QObject](./qobject_struct.md)
  * [`#[cxx_qt::qsignals(T)]` - A Signals enum for defining signals](./signals_enum.md)
  * [`qobject:T` - The generated QObject](./cpp_object.md)
  * [`CxxQtThread` - Queueing function pointers onto the Qt thread](./cxxqtthread.md)


