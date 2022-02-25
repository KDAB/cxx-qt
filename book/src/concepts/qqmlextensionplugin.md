<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# QQmlExtensionPlugin

Qt allows for plugins containing object definitions to be loaded at runtime from a directory instead of being embedded into the application.

This allows for a clean split between disciplines of business logic and GUI code.

CXX-Qt allows for generating a plugin and qmldir file so that you can load Rust objects as a plugin into your application.

## Rust build.rs changes

In your `build.rs` specify that you want to use a QQmlExtensionPlugin by invoking the method `qqmlextensionplugin` as seen in the following example.

Here you specify the import name for QML and the name you are using for the resultant plugin target.

```rust,ignore,noplayground
{{#include ../../../examples/qml_extension_plugin/core/build.rs:book_build_rs}}
```

## CMake changes

The following example shows the CMake definition for building an extension plugin.

Note that the folder structure must match the QML import name, eg `import foo.bar 1.0` means the folder structure of `foo/bar` is required that then contains the plugin and qmldir file.

```cmake,ignore
{{#include ../../../examples/qml_extension_plugin/core/CMakeLists.txt:book_cmake_generation}}
```

## Qt C++ changes

To load the plugin at runtime add the directory containing the plugin to the QML import path.

```cpp,ignore
{{#include ../../../examples/qml_extension_plugin/main.cpp:book_extension_plugin_register}}
```
