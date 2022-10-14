<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Our first CXX-Qt module

As with all things Rust, we'll first want to create a cargo project.
```bash
cargo new --lib qml-minimal
```
Note the `--lib` option here. For this example, we will create a static library in Rust and use CMake to
link this into a C++ executable. We'll discuss details of this later, when we [integrate our Rust project with CMake](./5-cmake-integration.md).

As outlined in the previous section, to define a new QObject subclass, we'll create a Rust module within this library crate.
First, in the `src/lib.rs`, we tell Cargo about the module we're about to create:

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/lib.rs:book_mod_statement}}
```

Now, we need to create a file `src/cxxqt_object.rs` for that module.
It will include our `#[cxx_qt::bridge]` that allows us to create our own qobjects in Rust:

```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_cxx_qt_module}}
```

This is a lot to take in, so let's go one step at a time.
Starting with the module definition:
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_bridge_macro}}
    // ...
}
```

A `#[cxx_qt::bridge]` is the same as a `#[cxx::bridge]` and you can use all features of CXX in it.
Additionally, a `#[cxx_qt::bridge]` gives you a few more features that allow you to create QObjects.

To create a new QObject subclass, we can define a struct within our module and mark it with `#[cxx_qt::qobject]`.
Additionally, we need to either `impl Default` or `#[derive(Default)]` for our struct.
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_struct}}
```
The Rust struct can be defined just like a normal Rust struct and can contain any kind of field, even Rust-only types.
If a field is marked as `#[qproperty]` it will be exposed to the C++ side as a `Q_PROPERTY`.

That means the newly created QObject subclass will have two properties as members: `number` and `string`. For names that contain multiple words, like `my_number`, CXX-Qt will automatically rename the field from snake_case to camelCase to fit with C++/QML naming conventions (e.g. `myNumber`).

Do note though that any fields marked as `#[qproperty]` must be types that CXX can translate to C++ types.
In our case that means:
- `number: i32` -> `int number`
- `string: QString` -> `QString string`

For `i32`, CXX already knows how to translate it.
A `QString` however is unknown to CXX.
Luckily, the `cxx_qt_lib` crate already wraps many Qt types for us.
We can just import them like any other CXX type:
``` rust, ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_qstring_import}}
```
For more details on the available types, see the [Qt types page](../concepts/types.md).

CXX-Qt will then automatically generate a new QObject subclass for our `MyObject` struct and expose it as an [`extern "C++"` opaque type](https://cxx.rs/extern-c++.html#opaque-c-types) to Rust.
For any Rust struct `T` that is marked with `#[cxx_qt::qobject]`, CXX-Qt will expose its QObject wrapper under `qobject::T`.
In our case, this means we can refer to the QObject wrapper for our `MyObject` struct, as `qobject::MyObject`.

This type can be used like any other CXX opaque type.
Additionally, CXX-Qt allows us to add functionality to this QObject by using `impl qobject::MyObject` together with `#[qinvokable]`.
```rust,ignore
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_impl}}
```

In our case, we define two new functions:
- `increment_number`
    - Increments the number of the `MyObject`.
    - The name will be converted to `incrementNumber` in C++.
- `say_hello`
    - Prints a provided number and string.
    - The name will be converted to `sayHello` in C++.

Apart from functions marked with the `#[qinvokable]` macro, you can also define normal helper functions on this struct that won't be exposed to QML.
These functions may still be called from Rust and can be defined by omitting the `#[qinvokable]` macro.

And that's it. We've defined our first QObject subclass in Rust. That wasn't so hard, was it?

Now let's get to [using it in Qt](./3-exposing-to-qml.md).
