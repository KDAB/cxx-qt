<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Build Tooling

Here we describe how to integrate and build a Rust CXX-Qt project into a CMake Qt project.

A folder structure might look like the following if you have a clear split between Rust backend and QML frontend.

```ignore
src/
 - core/
   - build.rs
   - Cargo.toml
   - src/
     - lib.rs
 - ui/
   - main.qml
   - qml.qrc
 CMakeLists.txt
 main.cpp
```

Also see the examples folder for other ways of building the project.

## build.rs

We need to specify a build.rs so that we can parse the macros and generate relevant C++ code.

The following options are available

  * Indicating which files should be parsed to look for macros
  * Enable building as a QQmlExtensionPlugin
  * Deciding the clang-format style of the generated C++ code
  * Specifiying a custom C++ namespace for the generated Rust types

A build.rs script could look like the following

```rust,ignore,noplayground
{{#include ../../../examples/qml_minimal/build.rs:book_build_rs}}
```

If you are registering as a plugin it could like the following

```rust,ignore,noplayground
{{#include ../../../examples/qml_extension_plugin/core/build.rs:book_build_rs}}
```

A non-default C++ namespace could be like the following

Note that the namespace is a list, so `vec!["a", "b", "c"]` would become `a::b::c`

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/build.rs:book_build_rs}}
```

## CMake

We need to add cmake to generate the C++ code and then link to it, ensure that `CxxQt.cmake` is in CMake's path.

```cmake,ignore
{{#include ../../../examples/qml_minimal/CMakeLists.txt:book_cmake_generation}}
```

If you are using a QQmlExtensionPlugin then we build a library and use that in the normal way.

```cmake,ignore
{{#include ../../../examples/qml_extension_plugin/core/CMakeLists.txt:book_cmake_generation}}
```

## C++ Registering QML types

There are two options for registering the generated QML types, either as a QQmlExtensionPlugin or registering the types to the engine.

### Registering to the engine

If you are registering the types to the engine, firstly you include the generated objects (determined by the name of the Rust module).

```cpp,ignore
{{#include ../../../examples/qml_minimal/src/main.cpp:book_cpp_include}}
```

Then you register the QML Type in the normal way.

```cpp,ignore
{{#include ../../../examples/qml_minimal/src/main.cpp:book_qml_register}}
```

Note in the future there may be a helper to call which could register all the types even when not using a plugin ( [https://github.com/KDAB/cxx-qt/issues/33](https://github.com/KDAB/cxx-qt/issues/33) ).

### Using QQmlExtensionPlugin

If you are using a QQmlExtensionPlugin then ensure the generated library is in the import path.

```cpp,ignore
{{#include ../../../examples/qml_extension_plugin/main.cpp:book_extension_plugin_register}}
```

### QML

Once you have used either of the methods above for registering the types to the engine, then from QML you can include these like a normal C++ module.

```qml,ignore
{{#include ../../../examples/qml_minimal/src/main.qml:book_qml_import}}
```
