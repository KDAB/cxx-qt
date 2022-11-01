<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

Welcome to our examples folder, here you will find various full examples that demonstrate different features and are used as part of the unit tests.

A minimal example can be found in the [`qml_minimal`](./qml_minimal/) folder which shows how to expose basic properties and invokables.
This example is built with CMake.
A minimal example building the same code with Cargo is in the [`cargo_without_cmake`](./cargo_without_cmake/) folder.

Then we have multiple other examples available inside the projects

  * [Defining properties and invokables](./qml_features/rust/src/invokables.rs.rs)
  * [Defining & Emitting Signals](./qml_features/rust/src/signals.rs)
  * [Using threaded logic](./qml_features/rust/src/threading.rs)
  * [How to (de)serialise QObjects](./qml_features/rust/src/serialisation.rs)
  * [Using Qt types such as QVariant](./qml_features/rust/src/types.rs)
  * [Defining multiple QObjects in a single bridge](./qml_features/rust/src/multiple_qobjects.rs)
  * [Exposing the Rust objects via a QQmlExtensionPlugin](./qml_extension_plugin/plugin/)

For documentation on how to use these features please visit our Book [https://kdab.github.io/cxx-qt/book/](https://kdab.github.io/cxx-qt/book/).
