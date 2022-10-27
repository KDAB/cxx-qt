<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# `#[cxx_qt::bridge]` Macro

The `#[cxx_qt::bridge]` macro functions very similarly to the [`#[cxx::bridge]`](https://docs.rs/cxx/latest/cxx/attr.bridge.html). This macro needs to be written above a Rust module definition.
This Rust module will then function like a normal CXX bridge, whilst also supporting the additional features added by CXX-Qt. Refer to the [the CXX documentation](https://cxx.rs/) for details on how to describe the language boundary.
Also don't forget to add the Rust source file to the CxxQtBuilder in your build.rs script.
For instructions, see the [Getting-started guide](../getting-started/5-cmake-integration.md).

## Filename
A C++ header file will be generated for every Rust file with a `#[cxx_qt::bridge]` module listed with [`CxxQtBuilder::file`](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.CxxQtBuilder.html#method.file).

By default, the name of the generated C++ header file will be the name of the module, followed by `.cxxqt.h`.
Our plan is to change this to use the Rust file name instead. Progress on this can be tracked in [#200](https://github.com/KDAB/cxx-qt/pull/200).

This filename can also be changed using the `cxx_file_stem` attribute.
The following example results in a header file named: `types.cxxqt.h`.
``` rust, ignore
{{#include ../../../examples/qml_features/rust/src/types.rs:book_cxx_file_stem}}
  // ...
}
```

Currently, cxx-qt-gen writes all generated header files into a single folder.
Therefore you need to be careful to not produce two header files with the same filename.
In future we plan to use the entire module path to disambiguate this.
Progress on this can be tracked in [#19](https://github.com/KDAB/cxx-qt/issues/19).

## C++ namespace
Just like on a `#[cxx::bridge]`, the C++ namespace of the bridge can be changed using the `namespace` attribute.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/threading.rs:book_namespace_macro}}
  // ...
}
```
This will generate a header file named `threading_website.cxxqt.h` with all C++ items included in the `cxx_qt::website` namespace.

When accessing a type from the bridge module in C++, access it through the C++ namespace:
```rust,ignore,noplayground
{{#include ../../../examples/qml_features/cpp/main.cpp:book_namespace_register}}
```
