<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# C++ Registering QML types

There are two options for registering the generated QML types, either as a plugin or registering the types to the engine.

## Registering to the engine

If you are registering the types to the engine, firstly you include the generated objects.

```cpp,ignore
{{#include ../../../examples/qml_minimal/cpp/main.cpp:book_cpp_include}}
```

Then you register the QML Type in the normal way.

```cpp,ignore
{{#include ../../../examples/qml_minimal/cpp/main.cpp:book_qml_register}}
```

## Using a plugin

If you are [creating a plugin for QML](https://doc.qt.io/qt-6/qtqml-modules-cppplugins.html) then ensure any of the normal changes to a CMake projects to build the plugin as a library, add the `qmldir`, have a C++ file which derives from `QQmlExtensionPlugin` and registers the types, and ensure that the generated library is in the import path.

An example of using CXX-Qt with `QQmlExtensionPlugin` can be found in the [`examples/qml_extension_plugin`](https://github.com/KDAB/cxx-qt/tree/main/examples/qml_extension_plugin) folder.

## QML

Once you have used either of the methods above for registering the types to the engine, then from QML you can include these like a normal C++ module.

```qml,ignore
{{#include ../../../examples/qml_minimal/qml/main.qml:book_qml_import}}
```
