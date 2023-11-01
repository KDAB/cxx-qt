<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Our first CXX-Qt module

We first need to create a folder structure to add the different parts of our project.

```ignore
tutorial
  - cpp
  - qml
  - rust
```

As with all things Rust, we'll want to create a cargo project, run the following command inside the `tutorial` folder to initialize the Rust part of the project.
```bash
cargo init --lib rust
```
Note the `--lib` option here. For this example, we will create a static library in Rust and use CMake to
link this into a C++ executable. We'll discuss details of this later, when we [integrate our Rust project with CMake](./4-cmake-integration.md).

As outlined in the previous section, to use CXX-Qt, we'll create a Rust module within this library crate.
This Rust module will then serve as our interface between Qt and Rust.
First, in the `rust/src/lib.rs`, we tell Cargo about the module we're about to create:

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/lib.rs:book_mod_statement}}
```

Now, we need to create a file `rust/src/cxxqt_object.rs` for that module.
It will include our `#[cxx_qt::bridge]` that allows us to interact with Qt concepts.

This is a lot to take in, so let's go one step at a time.

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_cxx_qt_module}}
```

## CXX-Qt bridge module
Starting with the module definition:
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_bridge_macro}}
    // ...
}
```

A `#[cxx_qt::bridge]` is the same as a `#[cxx::bridge]` and you can use all features of CXX in it.
Additionally, a `#[cxx_qt::bridge]` gives you a few more features that allow you to create QObjects from Rust or declare existing QObjects for access from Rust.

## `extern "RustQt"`

Like `extern "Rust"` and `extern "C++"` in CXX, CXX-Qt provides `extern "RustQt"` and `extern "C++Qt"`.

These `extern` blocks instruct CXX-Qt to where the implementation of our interface lives.
Anything that is marked as `extern "RustQt"` is implemented in Rust and will be exposed to C++.
Conversely anything inside `extern "C++Qt"` is implemented in C++ and will be exposed to Rust.

## QObject struct

First we will create a new QObject subclass.
As we want to implement it in Rust, we need to place our interface inside `extern "RustQt"`.

To create a new QObject subclass that will be defined in Rust, use a type-alias and mark it with `#[qobject]`.
In our case the new class will be named `MyObject` and will be backed by a Rust struct named `MyObjectRust`.

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_struct_signature}}
```

The Rust struct must be defined **outside** the bridge module and is therefore referred to using `super::`.
This just needs to be a normal Rust struct and can contain any kind of field, even Rust-only types that are not compatible with CXX.

Unless we want to use CXX-Qt's [Constructor feature](https://docs.rs/cxx-qt/latest/cxx_qt/trait.Constructor.html) we just need to ensure that this struct implements Rusts `Default` trait
In this case we just use `#[derive(Default)]` on the struct.

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_struct}}
```

Now the `#[qobject]` macro will take care of creating a new QObject subclass named `MyObject`.
Every instance of that class will also include an instance of the `MyObjectRust` struct that the `MyObject` class will defer to for any data or behavior.

To automatically export this new class to QML, we mark it with the `#[qml_element]` attribute.
This is the same as specifying [`QML_ELEMENT`](https://doc.qt.io/qt-6/qqmlengine.html#QML_ELEMENT) in C++.
This takes the place of the [qt_add_qml_module CMake function](https://doc.qt.io/qt-6/qt-add-qml-module.html)
(because that doesn't work with CXX-Qt's build system).

The `#[qproperty]` attribute will create a [`Q_PROPERTY`](https://doc.qt.io/qt-6/properties.html) for the given type and field name.
CXX-Qt will then:
1. Create the `Q_PROPERTY` on the QObject type.
2. Create a `NOTIFY` signal for when the property changes.
3. Generate getters and setters that use the underlying Rust fields and emit the NOTIFY signal on changes.

In this case the newly created QObject subclass will have two properties: `number` and `string`.
CXX-Qt expects a field for each property to exist in the underlying Rust struct.
For names that contain multiple words, like `my_number`, CXX-Qt will automatically rename the field from snake_case to camelCase to fit with C++/QML naming conventions (e.g. `myNumber`).

### Types

Please note that any fields exposed as `#[qproperty]` must have types that CXX can translate to C++ types.
In our case that means:
- `#[qpoperty(i32, number)]` -> `Q_PROPERTY(::std::int32_t number ...)`
- `#[qproperty(QString, string)` -> `Q_PROPERTY(QString string ...)`

For `i32`, CXX-Qt already knows how to translate it.
A `QString` however is unknown to CXX.
Luckily, the [`cxx_qt_lib`](https://docs.rs/cxx-qt-lib/latest/cxx_qt_lib/) crate already wraps many Qt types for us.
We can just include them in the bridge like any other CXX type:
``` rust, ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_qstring_import}}
```
For more details on the available types, see the [Qt types page](../concepts/types.md).

-------

CXX-Qt will then automatically generate a new QObject subclass called `MyObject` and expose it as an [`extern "C++"` opaque type](https://cxx.rs/extern-c++.html#opaque-c-types) back to Rust.
In our case, this means we can refer to the C++ QObject as `qobject::MyObject`, as it is defined inside the `mod qobject`.

This type can be used like any other CXX opaque type.

## Invokables

Additionally, CXX-Qt allows us to add functionality to this QObject by referring to the type as the self type of functions in an `extern "RustQt"` block.
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_invokable_signature}}
```

This works the same as exposing any other [member function with CXX](https://cxx.rs/extern-rust.html#methods) in an `extern "Rust"` block.
Additionally CXX-Qt understands the `#[qinvokable]` attribute and marks the member function as [`Q_INVOKABLE`](https://doc.qt.io/qt-6/qtqml-cppintegration-exposecppattributes.html#exposing-methods-including-qt-slots).
This means they can be called from QML.



These functions then need to be implemented outside the bridge using `impl qobject::MyObject`.

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_invokable_impl}}
```

This setup is a bit unusual, as the type `qobject::MyObject` is actually defined in C++.
However, it is still possible to add member functions to it in Rust and then expose them back to C++.
This is the usual workflow for QObjects in CXX-Qt.
CXX-Qt will define the QObject class itself in C++, but defer to Rust for any behavior.

> Note that we recommend calling the bridge module `qobject` instead of the CXX-typical `ffi`.
> This way accessing the C++ QObject outside the bridge becomes a natural `qobject::MyObject`
> instead of `ffi::MyObject`.
>
> Feel free to choose any module name you like though.

Also do not forget to import everything required for the invokable implementation.

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_use}}
```

In our case, we define two new functions:
- `increment_number`
    - Increments the number of the `MyObject`.
    - The name will be converted to `incrementNumber` in C++.
- `say_hello`
    - Prints a provided number and string.
    - The name will be converted to `sayHello` in C++.

Because we are implementing on the `qobject::MyObject` type instead of the `MyObject` type, `self` here is the C++ QObject that is generated from our `MyObject` struct.
As this type is a CXX Opaque type, we can't actually instantiate it.
Our Qt code will take care of this.
Also, we can't move the QObject, which is why it is behind a Rust [`Pin`](https://doc.rust-lang.org/std/pin/struct.Pin.html).

CXX-Qt will generate getters and setters for all properties of our struct.
That's where the `number()` and `set_number()` functions used by `increment_number()` come from.
For more details on what you can do with the QObject from Rust and what functions CXX-Qt will generate for you, take a look at the [QObject page](../concepts/generated_qobject.md).

And that's it. We've defined our first QObject subclass in Rust. That wasn't so hard, was it?

Now let's get to [using it in Qt](./3-qml-gui.md).
