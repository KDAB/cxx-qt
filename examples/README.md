<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

Welcome to our examples folder, here you will find various full examples that demonstrate different features and are used as part of the unit tests.

A minimal example can be found in the [`qml_minimal`](./qml_minimal/) folder which shows how to expose basic properties and invokables.

Then we have multiple other examples availabe inside the projects

  * [Defining properties and invokables](./qml_features/src/rust_obj_invokables.rs)
  * [Handling property changes on the Rust side](./qml_features/src/handler_property_change.rs)
  * [How to (de)serialise QObjects](./qml_features/src/serialisation.rs)
  * [Using threaded logic](./qml_with_threaded_logic/src/lib.rs)
  * [Using Qt types such as QVariant](./qml_features/src/types.rs)
  * [Exposing the Rust objects via a QQmlExtensionPlugin](./qml_extension_plugin/core/)

For documentation on how to use these features please visit our Book [https://kdab.github.io/cxx-qt/book/](https://kdab.github.io/cxx-qt/book/).
