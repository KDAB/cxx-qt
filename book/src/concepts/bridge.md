<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Bridge

CXX-Qt uses [CXX](https://cxx.rs/) for bridging between C++ and Rust in a safe way.

CXX-Qt provides macros for declaring Qt objects such as [QObject](../qobject/index.md) while still being idomatic Rust code.

We provide [Qt types](./types.md) to help pass common data types across the bridge between Rust and Qt.

When Rust items are exposed to C++ we automatically perform a conversion between Snake case and Camel case. So that items (such as properties and invokables) appear as Camel case to C++ but Snake case to Rust.

Note that the Rust [`QObject marked struct`](../qobject/qobject_struct.md) of a constructed Qt object is owned by the C++ side of the bridge representing it. So when the C++ object is destroyed the Rust object will be destroyed. In the future there will be [handlers](../qobject/handlers.md) for executing Rust code from the (de)constructor of the C++ object [https://github.com/KDAB/cxx-qt/issues/13](https://github.com/KDAB/cxx-qt/issues/13).
