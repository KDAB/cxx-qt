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

  * Deciding the clang-format style of the generated C++ code
  * Indicating which files should be parsed to look for macros
  * Enabling build as a QQmlExtensionPlugin if required
  * Specifiying a custom C++ namespace for the generated Rust types

A build.rs script could look like the following

```rust,ignore,noplayground
{{#include ../../../examples/qml_minimal/build.rs:book_build_rs}}
```

If you are registering as a plugin, with a different clang-format style, and a non-default C++ namespace it could like the following

```rust,ignore,noplayground
{{#include ../../../examples/qml_extension_plugin/build.rs:book_build_rs}}
```

A non-default C++ namespace could be like the following

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
{{#include ../../../examples/qml_extension_plugin/CMakeLists.txt:book_cmake_generation}}
```

## C++ Registering QML types

There are two options for registering the generated QML types, either as a QQmlExtensionPlugin or registering the types to the engine.

If you are registering the types to the engine, then you can include the generated objects (determined by the name of the Rust module), and register in the normal way.

```cpp,ignore
{{#include ../../../examples/qml_minimal/src/main.cpp:book_cpp_include}}
```

```cpp,ignore
{{#include ../../../examples/qml_minimal/src/main.cpp:book_qml_register}}
```

If you are using a QQmlExtensionPlugin then ensure the generated library is in the import path.

```cpp,ignore
{{#include ../../../examples/qml_extension_plugin/src/main.cpp:book_extension_plugin_register}}
```

Then from QML you can include these like a normal C++ module.

```qml,ignore
{{#include ../../../examples/qml_minimal/src/main.qml:book_qml_import}}
```
