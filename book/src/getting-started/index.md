<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CXX-Qt - Getting Started

In comparison to other Qt-Rust-bindings, CXX-Qt does not aim to simply expose Qt functionality to Rust, but rather to completely integrate Rust into the Qt ecosystem.

In this guide we'll go through a minimal example that uses CXX-Qt to create your own `QObject` in Rust and integrate it with a small QML-based GUI.

## Prerequisites

This guide won't be able to explain everything to you, but it will try its best to make sure everyone can follow along.
However, there are a few things you should be familiar with before reading this guide.
It may be confusing otherwise!

Firstly, you should be familiar with Rust. There are many great resources for learning Rust, like [the book](https://doc.rust-lang.org/book/).

As CXX-Qt aims to integrate Rust into the existing Qt ecosystem, you should have basic knowledge of Qt and QML.
If you're not familiar with Qt/QML yet, take a look at the [Qt Getting started guide](https://doc.qt.io/qt-6/gettingstarted.html) or the [QML intro](https://doc.qt.io/qt-6/qmlapplications.html) respectively.

CXX-Qt (as the name suggests) is built on top of [CXX](https://cxx.rs).
You should have a basic idea of how CXX works before attempting to follow this guide.
Take a look at the CXX documentation here: <https://cxx.rs/>

### Installation

You'll need to have the following tools installed:

- A working C/C++ compiler
- [CMake version 3.24 or above](https://cmake.org/)
- [The Rust toolchain](https://rustup.rs/)
- [Qt 5 or Qt 6](https://www.qt.io/download-open-source) - the open source version will do just fine

> ⚠️ It is vital that the `qmake` executable can be found by CXX-Qt.
> On Windows, you may have to manually add it to your `PATH` in order for this to work automatically.
>
> To check that `qmake` can indeed be found, run:
>
> ```shell
> $ qmake --version
> QMake version 3.1
> Using Qt version 6.5.1 in /usr/lib64
> ```
>
> If you don't want to add `QMAKE` to your path you can always provide Cargo with the right path by
> using the `QMAKE` environment variable.\
> e.g.: `QMAKE=/usr/bin/qmake cargo build`

We unfortunately cannot list every way to install these tools on all platforms.
Please make sure you have installed the right toolchains before following this guide!

### What this guide covers

During this getting started guide we'll first take a look at how CXX-Qt integrates with Qt's object system to allow the [definition of `QObject`s in Rust](./1-qobjects-in-rust.md).
Then we'll dive straight into practice and define our first [`QObject` in Rust](./2-our-first-cxx-qt-module.md).
Followed by actually [defining our GUI using QML](./3-qml-gui.md).

We will show two different ways to build the project.
First we will build the CXX-Qt code [as a Rust executable](./4-cargo-executable.md) without requiring a C++ build system.
Additionally, we will show how to integrate CXX-Qt into a C++ application by [building with CMake](./5-cmake-integration.md).

**Note:** CXX-Qt is tested on CI on Linux, Windows, and macOS (all on x86_64). It should work on other platforms that Qt and Rust both support, however, these are not tested regularly.

So, without further ado - let's [Get Started](./1-qobjects-in-rust.md)
