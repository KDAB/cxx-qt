<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# `extern "RustQt"`

- [`QObject`s](#qobjects)
- [Properties](#properties)
- [Methods](#methods)
- [Signals](#signals)

```rust,ignore,noplayground
#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {

    }
}
```

The `extern "RustQt"` section of a CXX bridge declares Rust types and signatures to be made available to Qt and C++.

The CXX code generator uses your `extern "Rust"` section(s) to produce a C++ header file containing the corresponding C++ declarations. The generated header has a file name matching the module ident or the `cxx_file_stem` field in the `#[cxx_qt::bridge]` attribute and with a `.cxxqt.h` file extension.

A bridge module may contain zero or more `extern "RustQt"` blocks.

This complements the [`extern "Rust"` CXX section](https://cxx.rs/extern-rust.html)
but allows for declaring Qt specific features on C++ types.

## `QObject`s

Types specified with a `#[qobject]` attribute are generated in C++ as a [`QObject`](https://doc.qt.io/qt-6/qobject.html).

The left side of the type specifies the C++ generated type and name, when referring to the C++ context this should be used. The right side of the type specifies which Rust type provides the inner implementation of the type (for example fields).

```rust,ignore,noplayground
#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {
        #[qobject]
        type MyObject = super::MyObjectRust;
    }
}

#[derive(Default)]
struct MyObjectRust;
```

### QML Attributes

`QObject`s can be registered as a QML type directly at build time by using the [`#[qml_element]`](https://doc.qt.io/qt-6/qqmlengine.html#QML_ELEMENT) attribute.

``` rust,ignore,noplayground
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_struct_signature}}
```

Additionally, you can configure the QML registration with these attributes:
<!--
TODO: we need to add https://doc.qt.io/qt-6/qqmlengine.html#QML_ANONYMOUS
TODO: we need to add https://doc.qt.io/qt-6/qqmlengine.html#QML_INTERFACE
-->

- [`qml_name`](https://doc.qt.io/qt-6/qqmlengine.html#QML_NAMED_ELEMENT): Use a different type name for QML.
- [`qml_uncreatable`](https://doc.qt.io/qt-6/qqmlengine.html#QML_UNCREATABLE): Mark the type as uncreatable from QML. It may still be returned by C++/Rust code.
- [`qml_singleton`](https://doc.qt.io/qt-6/qqmlengine.html#QML_SINGLETON): An instance of the `QObject` will be instantiated as a singleton in QML.

> The Rust file must be included within a [QML module in the `build.rs` file](../concepts/build_systems.md#qml-modules)

### `base` attribute

Use the `base` attribute to specify a C++ class that the C++ `QObject` will inherit from.
The base class must inherit from `QObject` (directly or indirectly). If you do not specify a base attribute, it will inherit directly from `QObject`.

``` rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/custom_base_class.rs:book_qobject_base}}
```

Use the CXX `include!` macro to include the appropriate C++ header for the base class:

``` rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/custom_base_class.rs:book_base_include}}
```

For more information on inheritance and how to override methods see the [Inheritance & Overriding](../concepts/inheritance.md) page.

[Full Example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/custom_base_class.rs)

### Traits

The [`Default` trait](https://doc.rust-lang.org/std/default/trait.Default.html) needs to be implemented for the `#[qobject]` marked struct either by hand or by using the derive macro `#[derive(Default)]`. Or the [`cxx_qt::Constructor`](https://docs.rs/cxx-qt/latest/cxx_qt/trait.Constructor.html) trait needs to be implemented for the type.

For further documentation see the [traits page](./traits.md).

## Properties

The `#[qproperty(TYPE, NAME, ...)]` attribute can be specified on a [`#[qobject]` marked type](#qobjects) to expose a [`Q_PROPERTY`](https://doc.qt.io/qt-6/properties.html) on the generated `QObject`.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/properties.rs:book_properties_signature}}
```

The type and name of the

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/properties.rs:book_properties_struct}}
```

For every `#[qproperty]`, CXX-Qt will generate setters and getters, as well as a "changed" signal.

On the C++ side:

- setter: `set<Property>`
- getter: `get<Property>`
- changed: `<Property>Changed`

On the Rust side:

- setter: `set_<Property>`
- getter: `<Property>`
- changed: `<Property>_changed`

The generated Rust methods for [signals](#signals):

- connect: `connect_<Property>_changed`
- on: `on_<Property>_changed`

Where `<Property>` is the name of the property.

These setters and getters assure that the changed signal is emitted every time the property is edited.

> Note that in the future it will be possible to specify custom getters and setters

## Methods

Any signature with a `self` parameter is interpreted as a Rust method and exposed to C++ method for the given type.
The type much be either a shared reference `self: &T` or a pinned mutable reference `self: Pin<&mut T>`, where `T` is the [QObject](#qobjects) type.

``` rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/invokables.rs:book_cpp_method_signature}}
```

Implementations of the method are then written as normal on the C++ type outside the bridge.

``` rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/invokables.rs:book_cpp_method_impl}}
```

> Note how this uses `impl qobject::T` rather than `impl T` where `qobject` is the bridge module name.

### Invokables

The `#[qinvokable]` attribute can be specified on signatures to expose them as a [`Q_INVOKABLE`](https://doc.qt.io/qt-6/qobject.html#Q_INVOKABLE) in C++.

``` rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/invokables.rs:book_invokable_signature}}
```

Implementations then have no difference to non invokable methods.

``` rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/invokables.rs:book_invokable_impl}}
```

### Inheritance

Methods or signals that already exist on the base class of an object can be accessed via the `#[inherit]` attribute.

For documentation see the [inheritance](../concepts/inheritance.md) page.

### Specifiers

Generated methods can have C++ specifiers necessary to implement inheritance.

| C++ keyword | CXX-Qt attribute              |
|-------------|-------------------------------|
| `override`  | `#[cxx_override]` |
| `virtual`   | `#[cxx_virtual]`  |
| `final`     | `#[cxx_final]`    |

These are specified as an attribute on the method signature.

```rust,ignore
{{#include ../../../examples/qml_features/rust/src/custom_base_class.rs:book_inherit_data_signature}}
```

## Signals

The `qsignal` attribute is used in an `extern "RustQt"` block to define [signals](https://doc.qt.io/qt-6/signalsandslots.html) for a `QObject`.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_block}}
```

For every function signature in the `extern` block, CXX-Qt will generate a signal on the corresponding `QObject`.
If the function has parameters, they will become the parameters for the corresponding signal.

If a signal is defined on the base class of the `QObject` then `#[inherit]` can be used to indicate to CXX-Qt that the `Q_SIGNAL` does not need to be created in C++.

A full example can be found in the [qml features](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/signals.rs).

> Note that `#[cxx_name = "..."]` can also be used on a signal to declare a different name in C++ to Rust

> Note using `pub(self)` as the visibility of the signal
> allows for declaring private signals

### Connecting to a signal

For every signal defined in the enum, two methods are generated.

  1. `on_<signal_name>`
  2. `connect_<signal_name>`

The `on_<signal_name>` method takes a handler function as the parameter, which will be called when the signal is emitted.
That handler function's first argument is the `QObject` and the remaining arguments are the signal parameters.

The `connect_<signal_name>` function additionally takes the [Qt connection type](https://doc.qt.io/qt-6/qt.html#ConnectionType-enum) as a parameter.

Note that by using the `#[inherit]` macro on a signal, connections can be made to property changes
using the signal name `<property>Changed` with no parameters.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_connect}}
```

Each connection returns a `QMetaObjectConnectionGuard` which is used to manage the [`QMetaObject::Connection`](https://doc.qt.io/qt-6/qmetaobject-connection.html) connection.

Note that the `QMetaObjectConnection` returned by CXX-Qt behaves a bit different from the Qt C++ implementation.

When the `QMetaObjectConnectionGuard` is dropped, it automatically disconnects the connection, similar to how a C++ `std::unique_ptr` or Rusts `Box` behaves.
If you don't want to store the `QMetaObjectConnectionGuard`, call `release`, which will drop the object without disconnecting and return the internal `QMetaObjecConnection`.

> Note that the `QMetaObjectConnection` has a `disconnect` which can be called manually later

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_disconnect}}
```

### Emitting a signal

Call the function signature defined in the `extern "RustQt"` block to emit the signal.

Note that these are defined on the generated `QObject` [`qobject::T`](../concepts/generated_qobject.md), so can be called from any mutable `#[qinvokable]`.

The function will immediately emit the signal.
Depending on the connection type, the connected slots will be called either immediately or from the event loop (See [the different connection types](https://doc.qt.io/qt-6/qt.html#ConnectionType-enum)).
To queue the call until the next cycle of the Qt event loop, you can use the [`CxxQtThread`](https://docs.rs/cxx-qt/latest/cxx_qt/struct.CxxQtThread.html).

### Signal Inheritance

If a signal is defined on the base class of the `QObject` then the `#[inherit]` attribute can be used to indicate to CXX-Qt that the `Q_SIGNAL` does not need to be created in C++.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/custom_base_class.rs:book_qsignals_inherit}}
```
