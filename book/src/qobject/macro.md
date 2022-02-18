<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Macro

We define a module (which becomes our Qt object name) and then add `make_qobject` as a macro.

The example below would export the contents of the module as `MyObject` to Qt / QML.

Note that the object name needs to be unique to avoid clashes, in the future full module paths may be used to aid avoiding collisions [https://github.com/KDAB/cxx-qt/issues/19](https://github.com/KDAB/cxx-qt/issues/19) - but this doesn't prevent attempting to register two QML types with the same name.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/src/data_struct_properties.rs:book_macro_code}}
```

Note: this might change in the future to allow for defining the base class or options when exporting to QML and could become namespaced to `#[cxx_qt(QObject)]` ( [https://github.com/KDAB/cxx-qt/issues/22](https://github.com/KDAB/cxx-qt/issues/22) ).
