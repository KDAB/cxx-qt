<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Cpp Object

To access and mutate the C++ side, eg properties, we need a handle to access the C++ object. To do this safely CXX-Qt provides a `CppObj` type which is a safe wrapper around the generated C++ object.

## Invokables

To use the `CppObj` add `cpp: &mut CppObj` to your parameters of an invokable.

If the [`Data` struct](./data_struct.md) has a field called `number: i32`, then you can access properties by using `number(&self) -> i32` and `set_number(&mut self, number: i32)` on the `CppObj`.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/src/rust_obj_invokables.rs:book_cpp_obj}}
```

If there is a [`Signals` enum](./signals_enum.md) then you can call `emit_queued(&mut self, Signals)` or `unsafe emit_immediate(&mut self, Signals)` on the `CppObj` to emit a signal.

Note that `emit_immediate` is unsafe as it can cause deadlocks if the `Q_EMIT` is `Qt::DirectConnection` connected to a Rust invokable on the same QObject that has caused the `Q_EMIT`, as this would then try to lock the `RustObj` which is already locked.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/src/signals.rs:book_rust_obj_impl}}
```

## Threading

The `CppObj` is used for [threading](../concepts/threading.md) to access the `UpdateRequester` via the `update_requester(&self) -> cxx_qt_lib::update_requester::UpdateRequester` method.

```rust,ignore,noplayground
{{#include ../../../examples/qml_with_threaded_logic/src/lib.rs:book_cpp_update_requester}}
```

The `UpdateRequester` is moved into the Rust thread, then when `request_update(&self) -> bool` is called it triggers the [`UpdateRequestHandler`](./handlers.md) on the Qt thread.

```rust,ignore,noplayground
{{#include ../../../examples/qml_with_threaded_logic/src/lib.rs:book_request_update}}
```

## Deserialisation and serialisation

As described in the (de)serialisation section of the [Data struct](./data_struct.md) the `CppObj` has a `grab_values_from_data` for loading values from `Data` into the C++ instance.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/src/serialisation.rs:book_grab_values}}
```

## Type Wrappers

When using the getters or setters to access the C++ property values, the Rust getter and setter automatically perform any conversion between the [C++ and Rust types](../concepts/types.md). This allows for the Rust code to use the Rust representation of the types without needing to convert to or from the C++ type.

TODO: explain how we can use this for borrowRustObj later from a sub object etc (and note threading here) eg nested_object() could return `Borrow<T>`.

TODO: once we have borrow_rust_obj() explain how this can be used to reach another objects RustObj [https://github.com/KDAB/cxx-qt/issues/30](https://github.com/KDAB/cxx-qt/issues/30) ).
