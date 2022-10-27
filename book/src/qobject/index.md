<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# QObject

A QObject defined by CXX-Qt supports many features and is made up of quite a few parts.

This chapter goes into details on these.
For a simpler introduction, take a look at our [Getting Started guide](../getting-started/index.md).

QObject Features and Parts:
  * [`#[cxx_qt::bridge]` - The macro around the module](./bridge-macro.md)
  * [`#[cxx_qt::qobject]` - Marking a Rust struct as a QObject](./qobject_struct.md)
  * [`#[cxx_qt::qsignals(T)]` - An enum for defining signals](./signals_enum.md)
  * [`qobject:T` - The generated QObject](./generated-qobject.md)
  * [`CxxQtThread` - Queueing closures onto the Qt event loop](./cxxqtthread.md)


