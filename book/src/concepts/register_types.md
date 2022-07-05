<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# C++ Registering QML types

There are two options for registering the generated QML types, either as a [QQmlExtensionPlugin](./qqmlextensionplugin.md) or registering the types to the engine.

## Registering to the engine

If you are registering the types to the engine, firstly you include the generated objects (determined by the name of the Rust module).

```cpp,ignore
{{#include ../../../examples/qml_minimal/src/main.cpp:book_cpp_include}}
```

Then you register the QML Type in the normal way.

```cpp,ignore
{{#include ../../../examples/qml_minimal/src/main.cpp:book_qml_register}}
```

Note in the future there may be a helper to call which could register all the types even when not using a plugin ( [https://github.com/KDAB/cxx-qt/issues/33](https://github.com/KDAB/cxx-qt/issues/33) ).

## Using QQmlExtensionPlugin

If you are using a [QQmlExtensionPlugin](./qqmlextensionplugin.md) then ensure the generated library is in the import path.

```cpp,ignore
{{#include ../../../examples/qml_extension_plugin/main.cpp:book_extension_plugin_register}}
```

## QML

Once you have used either of the methods above for registering the types to the engine, then from QML you can include these like a normal C++ module.

```qml,ignore
{{#include ../../../examples/qml_minimal/src/main.qml:book_qml_import}}
```
