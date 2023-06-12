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

As with all things Rust, we'll want to create a cargo project, run the following command inside the `tutorial` folder to initialise the Rust part of the project.
```bash
cargo init --lib rust
```
Note the `--lib` option here. For this example, we will create a static library in Rust and use CMake to
link this into a C++ executable. We'll discuss details of this later, when we [integrate our Rust project with CMake](./4-cmake-integration.md).

As outlined in the previous section, to define a new QObject subclass, we'll create a Rust module within this library crate.
First, in the `rust/src/lib.rs`, we tell Cargo about the module we're about to create:

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/lib.rs:book_mod_statement}}
```

Now, we need to create a file `rust/src/cxxqt_object.rs` for that module.
It will include our `#[cxx_qt::bridge]` that allows us to create our own qobjects in Rust:

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_cxx_qt_module}}
```

This is a lot to take in, so let's go one step at a time.

## CXX-Qt bridge module
Starting with the module definition:
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_bridge_macro}}
    // ...
}
```

A `#[cxx_qt::bridge]` is the same as a `#[cxx::bridge]` and you can use all features of CXX in it.
Additionally, a `#[cxx_qt::bridge]` gives you a few more features that allow you to create QObjects.

## QObject struct

To create a new QObject subclass, we can define a struct within our module and mark it with `#[cxx_qt::qobject]`.

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_struct}}
```

Optionally, add `qml_uri` and `qml_version` inside `#[cxx_qt::qobject]` to tell the Rust build script to generate a QML plugin
that will register the QObject with QML engine at startup. If you want the name of the QML type and the Rust type to be different,
you can also add `qml_name = "OtherName"`. This takes the place of the
[qt_add_qml_module CMake function](https://doc.qt.io/qt-6/qt-add-qml-module.html) (because that doesn't work with CXX-Qt's build system).

Additionally, we need to either `impl Default` or `#[derive(Default)]` for our struct.
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_default}}
```

The Rust struct can be defined just like a normal Rust struct and can contain any kind of field, even Rust-only types.
If a field is tagged as `#[qproperty]` it will be exposed to the C++ side as a `Q_PROPERTY`.

That means the newly created QObject subclass will have two properties as members: `number` and `string`. For names that contain multiple words, like `my_number`, CXX-Qt will automatically rename the field from snake_case to camelCase to fit with C++/QML naming conventions (e.g. `myNumber`).

### Types

Do note though that any fields tagged as `#[qproperty]` must be types that CXX can translate to C++ types.
In our case that means:
- `number: i32` -> `::std::int32_t number`
- `string: QString` -> `QString string`

For `i32`, CXX-Qt already knows how to translate it.
A `QString` however is unknown to CXX.
Luckily, the [`cxx_qt_lib`](https://docs.rs/cxx-qt-lib/latest/cxx_qt_lib/) crate already wraps many Qt types for us.
We can just import them like any other CXX type:
``` rust, ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_qstring_import}}
```
For more details on the available types, see the [Qt types page](../concepts/types.md).

## qobject::T

CXX-Qt will then automatically generate a new QObject subclass for our `MyObject` struct and expose it as an [`extern "C++"` opaque type](https://cxx.rs/extern-c++.html#opaque-c-types) to Rust.
For any Rust struct `T` that is marked with `#[cxx_qt::qobject]`, CXX-Qt will expose the corresponding C++ QObject under `qobject::T`.
In our case, this means we can refer to the C++ QObject for our `MyObject` struct, as `qobject::MyObject`.

This type can be used like any other CXX opaque type.
Additionally, CXX-Qt allows us to add functionality to this QObject by referring to the type as the self type of functions in an `extern "RustQt"` block in together with `#[qinvokable]`.
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_invokable_signature}}
```

And then implementing the invokables outside the bridge using `impl qobject::MyObject`.
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_invokable_impl}}
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
Qt/C++ takes care of this.
However, we can still define Rust functions on this type.
They will just be normal Rust functions, but will be executed on a C++ type.
CXX-Qt will already generate getters and setters for all fields of your struct this way.
If you additionally mark any of these functions with `#[qinvokable]`, they will also be callable from C++ and QML.
In this case, the types of the arguments also need to convertable to C++, like with any `#[qproperty]`.

And that's it. We've defined our first QObject subclass in Rust. That wasn't so hard, was it?

Now let's get to [using it in Qt](./3-qml-gui.md).
