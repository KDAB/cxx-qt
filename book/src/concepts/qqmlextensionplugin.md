<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# QQmlExtensionPlugin

Qt allows for plugins containing object definitions to be loaded at runtime from a directory instead of being embedded into the application.

This allows for a clean split between disciplines of business logic and GUI code.

CXX-Qt allows for generating a plugin and qmldir file so that you can load Rust objects as a plugin into your application.

When using QQmlExtensionPlugin the folder structure of your project may look like the following, you can see the clear split between "core" and "ui".

```ignore
src/
 - core/
   - build.rs
   - Cargo.toml
   - CMakeLists.txt
   - src/
     - lib.rs
   - plugin.cpp
   - qmldir
 - ui/
   - main.qml
   - qml.qrc
 CMakeLists.txt
 main.cpp
```

## CMake changes

The following example shows the CMake definition for building an extension plugin.

Note that the folder structure must match the QML import name, eg `import foo.bar 1.0` means the folder structure of `foo/bar` is required that then contains the plugin and qmldir file.

```cmake,ignore
{{#include ../../../examples/qml_extension_plugin/core/CMakeLists.txt:book_cmake_generation}}
```

## Qt C++ changes

Define a QQmlExtensionPlugin `plugin.cpp` as normal, here you specify the QML names for the Rust types.

```cpp,ignore
{{#include ../../../examples/qml_extension_plugin/core/plugin.cpp:book_qml_plugin}}
```

Then create a `qmldir` file which matches your C++ library name.

```txt,ignore
{{#include ../../../examples/qml_extension_plugin/core/qmldir}}
```

To load the plugin at runtime add the directory containing the plugin to the QML import path.

```cpp,ignore
{{#include ../../../examples/qml_extension_plugin/main.cpp:book_extension_plugin_register}}
```

Then use the plugin from the import uri and version specified in the plugin and qmldir.

```qml,ignore
{{#include ../../../examples/qml_extension_plugin/ui/main.qml:book_qml_import}}
```
