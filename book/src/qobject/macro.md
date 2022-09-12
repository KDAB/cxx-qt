<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Macro

We define a module (which becomes our Qt object name) and then add `cxx_qt::bridge(namespace = "cxx_qt::my_object)` as a macro.

The namespace is optional and can be used allow segmenting generated C++ code from your application.

The example below would export the struct marked with `#[cxx_qt::qobject]` as `DataStructProperties` to Qt / QML.

Note that the object name needs to be unique to avoid clashes, in the future full module paths may be used to aid avoiding collisions [https://github.com/KDAB/cxx-qt/issues/19](https://github.com/KDAB/cxx-qt/issues/19) - but this doesn't prevent attempting to register two QML types with the same name.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/struct_properties.rs:book_macro_code}}
```

The threaded logic example shows how you can use a namespace to segment the generated C++ code.

```rust,ignore,noplayground
{{#include ../../../examples/qml_with_threaded_logic/rust/src/lib.rs:book_namespace_macro}}
```

Then when registering the type you use the type with your namespace as usual.

```rust,ignore,noplayground
{{#include ../../../examples/qml_with_threaded_logic/rust/src/lib.rs:book_namespace_register}}
```

You can also specify the base class by using `#[cxx_qt::qobject(base = "QStringListModel")]`. Note that you need to use CXX to include the header that the base class is declared in.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/custom_base.rs:book_macro_code}}
```
