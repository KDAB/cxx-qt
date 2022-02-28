<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Exposing our QObject subclass to QML

After [defining our first CXX-Qt module](./2-our-first-cxx-qt-module.md), we're ready to create our Qt application and export our new `MyObject` class to QML.

The easiest way to do this is to add a `main.cpp` file next to our `lib.rs` file in the `src` folder.
```cpp,ignore
{{#include ../../../examples/qml_minimal/src/main.cpp:book_main_cpp}}
```

This C++ file creates a basic Qt application and executes it.
If you're unfamiliar with this, I recommend you take a look at the [Qt documentation](https://doc.qt.io/qt-5/gettingstarted.html).

There are two notable changes compared to a normal Qt application though:
```cpp,ignore
{{#include ../../../examples/qml_minimal/src/main.cpp:book_cpp_include}}
```

```cpp,ignore
{{#include ../../../examples/qml_minimal/src/main.cpp:book_qml_register}}
```

For every QObject subclass that we define in Rust, CXX-Qt will generate a corresponding C++ class.
This class is included by the first code snippet.
They will always be in the `cxx-qt-gen/include/` include path and use the snake_case naming convention.

The second code snippet then exports the class to QML.
This works the same as it would for any other QObject subclass, as that is exactly what `MyObject` is, as far as Qt is concerned.
The only thing to note here is that the class is generated in the `cxx_qt::my_object` namespace.
Where `my_object` is the name of the Rust module we defined earlier.

As we later want to include our QML GUI in a `main.qml` file inside the [Qt resource system](https://doc.qt.io/qt-5/resources.html), we'll have to add a `qml.qrc` file as well:
```qrc,ignore
<RCC version="1.0">
    <qresource prefix="/">
        <file>main.qml</file>
    </qresource>
</RCC>
```

And that's it already. We can now [use our cool new class from QML](./4-qml-gui.md).
