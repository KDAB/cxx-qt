<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CXX-Qt - Getting Started

In comparison to other Qt-Rust-bindings, CXX-Qt does not aim to simply expose Qt functionality to Rust, but rather to completely integrate Rust into the Qt ecosystem.

In this guide we'll go through a minimal, but complete example that uses CXX-Qt to create your own QObject in Rust and integrate it with a small QML-based GUI.
As CXX-Qt aims to integrate Rust into the existing Qt ecosystem, you should have basic knowledge of Qt and QML before attempting to follow this guide.
If you're not familiar with Qt/QML yet, take a look at the [Qt Getting started guide](https://doc.qt.io/qt-5/gettingstarted.html) or the [QML intro](https://doc.qt.io/qt-5/qmlapplications.html) respectively.

During this getting-started guide we'll first take a look at how CXX-Qt integrates with Qt's object system to allow the [definition of QObjects in Rust](./1-qobjects-in-rust.md).
Then we'll dive straight into practice and define our first [QObject as a Rust module](./2-our-first-cxx-qt-module.md).
Once we've done that, its time to [expose the defined QObject to QML](./3-exposing-to-qml.md).
Followed by actually [defining our GUI using QML](./4-qml-gui.md).
And finally we [integrate our code with CMake](./5-cmake-integration.md) so we can build and run it.

So, without further ado - let's [Get Started](./1-qobjects-in-rust.md)
