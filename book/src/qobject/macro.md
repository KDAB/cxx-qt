<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Macro

We define a module (which becomes our Qt object name) and then add `cxx_qt::bridge(namespace = "cxx_qt::my_object)` as a macro.

The namespace specified is used in C++ to allow segmenting generated code from your application.

Note that currently each QObject needs to be in its own namespace otherwise there will be collisions from generated free functions.

The example below would export the struct marked with `#[cxx_qt::qobject]` as `DataStructProperties` to Qt / QML.

Note that the object name needs to be unique to avoid clashes, in the future full module paths may be used to aid avoiding collisions [https://github.com/KDAB/cxx-qt/issues/19](https://github.com/KDAB/cxx-qt/issues/19) - but this doesn't prevent attempting to register two QML types with the same name.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/src/data_struct_properties.rs:book_macro_code}}
```

Note: this might change in the future to allow for defining the base class or options when exporting to QML and could become namespaced to `#[cxx_qt::qobject(base = "QAbstractListModel")]` ( [https://github.com/KDAB/cxx-qt/issues/22](https://github.com/KDAB/cxx-qt/issues/22) ).
