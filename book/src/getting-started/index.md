<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CXX-Qt - Getting Started

In comparison to other Qt-Rust-bindings, CXX-Qt does not aim to simply expose Qt functionality to Rust, but rather to completely integrate Rust into the Qt ecosystem.

In this guide we'll go through a minimal example that uses CXX-Qt to create your own QObject in Rust and integrate it with a small QML-based GUI.

### Prerequisites
This guide won't be able to explain everything to you, but it will try its best to make sure everyone can follow along.
However, a few things you should make sure you're familiar with before attempting to follow this guide, as it may be confusing otherwise.

First of all, you should be familiar with Rust. There are many great resources for learning Rust, like [the book](https://doc.rust-lang.org/book/).

As CXX-Qt aims to integrate Rust into the existing Qt ecosystem, you should have basic knowledge of Qt and QML.
If you're not familiar with Qt/QML yet, take a look at the [Qt Getting started guide](https://doc.qt.io/qt-6/gettingstarted.html) or the [QML intro](https://doc.qt.io/qt-6/qmlapplications.html) respectively.

CXX-Qt (as the name suggests) is built on top of [CXX](https://cxx.rs).
You should have a basic idea of how CXX works before attempting to follow this guide.
Take a look at the CXX documentation here: [https://cxx.rs/](https://cxx.rs/)

### What this guide covers

During this getting started guide we'll first take a look at how CXX-Qt integrates with Qt's object system to allow the [definition of QObjects in Rust](./1-qobjects-in-rust.md).
Then we'll dive straight into practice and define our first [QObject in Rust](./2-our-first-cxx-qt-module.md).
Followed by actually [defining our GUI using QML](./3-qml-gui.md).

We will show two different ways to build the project.
First, we will show how to integrate cxx-qt into a C++ application by [building with CMake](./4-cmake-integration.md).
We will also demonstrate how to build the same cxx-qt code [as a Rust executable](./5-cargo-executable.md) without requiring a C++ build system.

**Note:** CXX-Qt is tested on CI on Linux, Windows, and macOS (all on x86_64). It should work on other platforms that Qt and Rust both support, however, these are not tested regularly.

So, without further ado - let's [Get Started](./1-qobjects-in-rust.md)
