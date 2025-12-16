<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Build Systems

CXX-Qt can be integrated into existing CMake projects or built with only cargo. The getting started guide provides documentation on how to set up your project:

- [Cargo Integration](../getting-started/4-cargo-executable.md)
- [CMake Integration](../getting-started/5-cmake-integration.md)

CXX-Qt could work with any C++ build system so long as the `QMAKE`, `CXX_QT_EXPORT_DIR` and `CXX_QT_EXPORT_CRATE_<CRATE-NAME>` environment variables are set before calling Cargo.
Take a look at our CMake code for how this can be used.
However, using C++ build systems besides Cargo or CMake with CXX-Qt is untested and the use of these environment variables is SemVer-exempt!

For information on building for WebAssembly (wasm), see: [Building for Webassembly](./wasm-builds.md)

## `CxxQtBuilder`

With both build systems a build script (`build.rs`) file needs to be used,
so that CXX-Qt knows which files to look for bridges and to build a Qt C++ library for linking later.

See [`CxxQtBuilder` documentation](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.CxxQtBuilder.html) for more details.

## QML Modules

When using QML with CXX-Qt [QML modules](https://doc.qt.io/qt-6/qtqml-writing-a-module.html) can be output.
This allows for attributes such as `#[qml_element]` to register the `QObject` with the QML type system without any C++ code.

See [`QmlModule` documentation](https://docs.rs/cxx-qt-build/latest/cxx_qt_build/struct.QmlModule.html) for more details.

### Dynamic QML module plugins

Qt allows building QML modules into dynamic plugins that are loaded on-demand at runtime.

By default, CXX-Qt uses static QML plugins, which are linked directly into the application.
As Rust prefers static linking in general, we recommend sticking with this approach when using CXX-Qt.

However, if you really need a dynamic QML plugin, CXX-Qt supports generating them **with CMake only**.
For an example, see the [qml_minimal_plugin example](https://github.com/KDAB/cxx-qt/tree/main/examples/qml_minimal_plugin) in the CXX-Qt repository.

To build a dynamic QML module plugin, start with a normal static QML module, then make sure you have taken these steps:

- Set the `crate-type` to `cdylib` in your Cargo.toml
- Pass `PluginType::Dynamic` to `QtModule::plugin_type` in your Rust build script
- Make sure you are using `cxx_qt_import_qml_module` in CMake with the correct `OUTPUT_DIR`
  - `OUTPUT_DIR` should be the directory of the main application binary, not the plugin library
  - Note: The `OUTPUT_DIR` defaults to the `CMAKE_CURRENT_BINARY_DIR`
- Build the QML module target separately (`cmake --build /path/to/build/` should build it by default)
- Your main binary no longer needs to link to the QML plugin

In this case, CXX-Qt generates one large dynamic library for the plugin and your Rust code.
Similar to this case described [in the Qt documentation](https://doc.qt.io/qt-6/qt-add-qml-module.html#plugin-target-with-no-backing-target).
Whenever QML code is loaded that references your QML module, it will be loaded automatically.

> ⚠️ Note: If your QML module contains paths above your Cargo.toml (e.g. `../qml/MyObject.qml`), they will not be exported correctly to tools like qmllint/qmlls.
> In general, we do not recommend to structure QML modules this way, as the directory structure in QRC will then include files outside the actual module directory.
> Some of our own examples use this structure for historical reasons - in that case: do as we say, not as we do 😅!
>
> This issue only affects dynamic QML module plugins, but we still recommend to avoid this pattern for static QML modules.

## QML Language Server (qmlls)

When using QML modules within a `build.rs` script CXX-Qt will automatically create a `.qmlls.ini` file, if one does not already exist, in the folder containing your Cargo.toml.

This informs any [qmlls](https://doc.qt.io/qt-6/qtqml-tooling-qmlls.html) instance running in child directories where to find the build directory.

> Note that if your QML files are in a parent or sibling folder you may need to copy or symlink the `.qmlls.ini` or create your own

## QML Lint (qmllint)

When using QML modules `.qmltypes` information is exported to the build folder which allows for both `qmllint` and `qmlls` to understand the types.

For [qmllint](https://doc.qt.io/qt-6/qtqml-tooling-qmllint.html) this can be executed successfully by adding the `cxxqt/qml_modules` folder from the build directory to the QML import paths with the `-I` flag.

```console
qmllint -I /path/to/cxxqt/qml_modules /path/to/test.qml
```

> When using CMake the folder is `${CMAKE_CURRENT_BINARY_DIR}/cxxqt/qml_modules` and when using Cargo the folder is `target/cxxqt/qml_modules`.
