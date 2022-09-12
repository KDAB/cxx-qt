<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Creating our QML GUI

As noted in the [QObjects in Rust](./1-qobjects-in-rust.md) chapter, we always want to use "the right tool for the right job".
For a small modern GUI in Qt, that definitely means using QML.
It's powerful, flexible, declarative, and allows us to iterate very quickly.

So let's add a `main.qml` next to our two other files in the `src` folder:
```qml,ignore
{{#include ../../../examples/qml_minimal/qml/main.qml:book_main_qml}}
```

If you're not familiar with QML, I recommend you take a look at the [Qt QML intro](https://doc.qt.io/qt-5/qmlapplications.html).

This code will create a pretty simple GUI that consists of two Labels and two Buttons.
The important part here is the use of the `MyObject` type.
As you can see, the class we defined earlier is now usable in QML.

As it is just another QObject subclass, it can be used in Qts property binding system, as is done with the `myObject.string`, which is bound to `myObject.number`.

The labels then simply display the data defined in the `MyObject` class.
We can use the two buttons to interact with the `MyObject` instance.
As you can see here, CXX-Qt has converted the snake_case of the function names to camelCase - `incrementNumber` and `sayHi`.
This way the `MyObject` doesn't seem at all out of place in QML.

It is again important to emphasize here that `MyObject` is just another QObject subclass and can be used just like any other `QObject` subclass.
The only difference being that any invokable functions that are defined are defined in Rust, instead of C++.
For QML, this doesn't make a difference though.

But enough of that, let's get this project [building and running](./5-cmake-integration.md).
