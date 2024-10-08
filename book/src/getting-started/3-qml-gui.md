<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Creating our QML GUI

As noted in the [QObjects in Rust](./1-qobjects-in-rust.md) chapter, we always want to use "the right tool for the right job".
For a small modern GUI in Qt, that definitely means using QML.
It's powerful, flexible, declarative, and allows us to iterate very quickly.

So let's add a `main.qml` file in a `qml` folder:

```qml,ignore
{{#include ../../../examples/qml_minimal/qml/main.qml:book_main_qml}}
```

If you're not familiar with QML, take a look at the [Qt QML intro](https://doc.qt.io/qt-6/qmlapplications.html).
We of course also recommend our [QML Intro Training](https://training.kdab.com/portfolio/introduction-to-qtqml/) and our [Introduction to Qt/QML YouTube series](https://www.youtube.com/playlist?list=PL6CJYn40gN6hdNC1IGQZfVI707dh9DPRc).

This code will create a pretty simple GUI that consists of two `Label`s and two `Button`s.
The important part here is the use of the `MyObject` type.
As you can see, the class we defined earlier is now usable in QML.

As it is just another `QObject` subclass, it can be used in Qt's property binding system, as is done with the `myObject.string`, which is bound to `myObject.number`.

The labels then simply display the data defined in the `MyObject` class.
We can use the two buttons to interact with the `MyObject` instance.

It is again important to emphasize here that `MyObject` is just another `QObject` subclass and can be used just like any other `QObject` subclass.
The only difference being that any invokable functions are defined in Rust, instead of C++.
For QML, this doesn't make a difference though.

Now that we have some application code, let's get this project [building and running](./4-cargo-executable.md)
