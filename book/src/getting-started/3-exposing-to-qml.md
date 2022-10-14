<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Exposing our QObject subclass to QML

After [defining our first CXX-Qt module](./2-our-first-cxx-qt-module.md), we're ready to create our Qt application and export our new `MyObject` class to QML.

The easiest way to do this is to add a `main.cpp` file in a `cpp` folder to clearly separate the C++ and Rust code:
```cpp,ignore
{{#include ../../../examples/qml_minimal/cpp/main.cpp:book_main_cpp}}
```

This C++ file creates a basic Qt application and executes it.
If you're unfamiliar with this, I recommend you take a look at the [Qt documentation](https://doc.qt.io/qt-5/gettingstarted.html).

There are two notable changes compared to a normal Qt application though:
```cpp,ignore
{{#include ../../../examples/qml_minimal/cpp/main.cpp:book_cpp_include}}
```

```cpp,ignore
{{#include ../../../examples/qml_minimal/cpp/main.cpp:book_qml_register}}
```

For every `#[cxx_qt::bridge]` that we define in Rust, CXX-Qt will generate a corresponding C++ header file.
This file is included by the first code snippet.
They will always be in the `cxx-qt-gen/` include path and use the snake_case naming convention.
The name of the header file will be the name of the Rust module of your `#[cxx_qt::bridge]`, followed by `.cxxqt.h`.
So in our case: `my_object.cxxqt.h`

The second code snippet then exports our `MyObject` class to QML.
This works the same as it would for any other QObject subclass, as that is exactly what `MyObject` is, as far as Qt is concerned.

As we later want to include our QML GUI in a `main.qml` file inside the [Qt resource system](https://doc.qt.io/qt-5/resources.html), we'll have to add a `qml.qrc` file in the `qml` folder as well:
```qrc,ignore
{{#include ../../../examples/qml_minimal/qml/qml.qrc:book_rcc_block}}
```
You can also omit this, but then you should change the url of the `main.qml` file, so that Qt can find it on your hard drive.
``` cpp, ignore
{{#include ../../../examples/qml_minimal/cpp/main.cpp:book_qml_url}}
```

And that's it. We can now [use our cool new class from QML](./4-qml-gui.md).
