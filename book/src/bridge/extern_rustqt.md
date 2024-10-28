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

The `extern "RustQt"` section of a CXX-Qt bridge declares Rust types and signatures to be made available to Qt and C++.

The CXX-Qt code generator uses your `extern "RustQt"` section(s) to produce a C++ header file containing the corresponding C++ declarations. The generated header has the same file name as the input rust file but with `.cxxqt.h` file extension.

A bridge module may contain zero or more `extern "RustQt"` blocks.

This complements the [`extern "Rust"` CXX section](https://cxx.rs/extern-rust.html)
but allows for declaring Qt specific features on C++ types.

Automatically converting to camel or snake case can be done through an [attribute](./attributes.md#automatic-case-conversion) at the block level.

## `QObject`s

The `#[qobject]` attribute may be placed on a type alias to generate a [`QObject`](https://doc.qt.io/qt-6/qobject.html) type in C++.

The left side of the type alias specifies the QObject type generated in C++.
When referring to the C++ context this should be used.
The right side of the type specifies which Rust type provides the inner implementation of the type (for example fields).

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

> **📝 Note**: At the moment, only `super::` is allowed as the path for the inner Rust type.
> Therefore, the Rust type must be available just outside the bridge module.
> You can bring any type into scope with a `pub use` directive if you want to reuse an existing type.

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

- [`#[qml_element]`](https://doc.qt.io/qt-6/qqmlengine.html#QML_NAMED_ELEMENT): Declare type as a qml element. An alternative type name for QML can be used like `#[qml_element = "MyName"]`
- [`#[qml_uncreatable]`](https://doc.qt.io/qt-6/qqmlengine.html#QML_UNCREATABLE): Mark the type as uncreatable from QML. It may still be returned by C++/Rust code.
- [`#[qml_singleton]`](https://doc.qt.io/qt-6/qqmlengine.html#QML_SINGLETON): An instance of the `QObject` will be instantiated as a singleton in QML.

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
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_struct_signature}}
```

If no other attributes are specified on the property, CXX-Qt will generate setters and getters, as well as a "changed" signal automatically.
The type and name of the property must then match a field in the inner Rust struct.

```rust,ignore,noplayground
{{#include ../../../examples/qml_minimal/rust/src/cxxqt_object.rs:book_rustobj_struct}}
```

CXX-Qt will then generate these functions:

|                | C++                 | Rust                 |
|----------------|---------------------|----------------------|
| setter         | `set<Property>`[^1] | `set_<property>`     |
| getter         | `get<Property>`[^1] | `<property>`         |
| changed signal | `<property>Changed` | `<property>_changed` |

As with any [signal](#signals), CXX-Qt will generate the corresponding connection functions on the Rust side:

- connect: `connect_<property>_changed`
- on: `on_<property>_changed`

Where `<property>` is the name of the property.

These setters and getters assure that the changed signal is emitted every time the property is edited.

[^1]: For the C++ getters and setters, the first character of the property name will automatically be upper-cased. For single-word property names, this leads to camelCase naming, which is the default in Qt.

### Custom Properties

In case the automatically generated functions do not work for your use-case, you can disable CXX-Qts auto-generation and write a totally custom property.
For example, this could be the case if your property doesn't correspond to any single field in the inner Rust struct.

You can specify custom getters, setters and notify signals, using flags passed like so:
`#[qproperty(TYPE, NAME, READ = myGetter, WRITE = mySetter, NOTIFY = myOnChanged)]`
> **📝 Note**: the key for the flags use all capitals like in the Qt version of qproperty

It is possible to use any combination of flags or omit some of them entirely, but if any flags are specified, the `READ` flag must be included.

If a custom function is specified for a flag, the function must be declared in the bridge and a corresponding implementation must exist.

Some of the flags may be passed with or without specifying a function (e.g. `READ` and `READ=...`).
For these flags CXX-Qt will auto-generate the implementation if no function was provided, as outlined in the previous section.
E.g. `#[qproperty(i32, num, READ)]` will automatically generate a getter function called `get_num` in Rust, and `getNum` in C++.
Therefore, `#[qproperty(i32, num)]` is just shorthand for `#[qproperty(i32, num, READ, WRITE, NOTIFY)]`.

Additionally, using `cxx_name` and `rust_name` is possible similarly to the attributes available on other items. e.g. `#[qproperty(i32, num, cxx_name = "numberProp")]`

### Examples

- `#[qproperty(TYPE, NAME, READ)]` A read only property with auto-generated getter
- `#[qproperty(TYPE, NAME, READ = myGetter, WRITE, NOTIFY)]` custom getter provided, but auto-generated setter and changed signal
- `#[qproperty(TYPE, NAME)]` is shorthand for `#[qproperty(TYPE, NAME, READ, WRITE, NOTIFY)]`
- `#[qproperty(TYPE, NAME, WRITE)]` is an error as the `READ` flag is required

### Available Flags

- `READ` or `READ = my_getter`
  - Specifies that the property should be readable (*always required if flags are passed*), with optional user defined getter
- `WRITE` or `WRITE = my_setter`
  - Specifies that the property should be writeable, with optional user defined setter
- `NOTIFY` or `NOTIFY = my_on_changed`
  - Specifies that the property should emit a notify signal on change, with optional user defined signal name
- `CONSTANT`
  - Specifies that the property should be constant (implication is that the getter returns the same value every time for that particular instance)
  - **`CONSTANT` is not available for properties which use `WRITE` or `NOTIFY` and will not compile**
- `REQUIRED`
  - Specifies that the property must be set by a user of the class, useful in QML as the class cannot be instantiated unless the property has been set
- `FINAL`
  - Specifies that the property will not be overriden by a derived class
- `RESET = my_reset`
  - Specifies a function to reset the property to a default value, user function **must** be provided or it will not compile
- `cxx_name = "myCxxName"`
  - Specifies an alternative name to use on the C++ side, applying to the property name as well as autogenerated functions
- `rust_name = "my_rust_name"`
  - Specifies an alternative name to use on the rust side, applying to the property name as well as autogenerated functions

## Methods

Any signature with a `self` parameter is interpreted as a Rust method and exposed to C++ method for the given type.
The type must be either a shared reference `self: &T` or a pinned mutable reference `self: Pin<&mut T>`, where `T` is the [QObject](#qobjects) type.

``` rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/invokables.rs:book_cpp_method_signature}}
```

Implementations of the method are then written as normal outside the bridge.

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
Signal functions do not need to be implemented manually.

If a signal is defined on the base class of the `QObject` then `#[inherit]` can be used, which will cause CXX-Qt to access the existing `Q_SIGNAL` from the base class.

A full example can be found in the [qml features example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/signals.rs).

> **📝 Note**: `#[cxx_name="..."]` and `#[rust_name="..."]` can be used on a signal to declare a different name in C++ to Rust

> **📝 Note**: using `pub(self)` as the visibility of the signal
> allows for declaring private signals

### Connecting to a signal

For every signal, CXX-Qt will generate two methods to connect to it.

  1. `on_<signal_name>`
  2. `connect_<signal_name>`

The `on_<signal_name>` method takes a handler function as the parameter, which will be called when the signal is emitted.
That handler function's first argument is the `QObject` that emitted the signal and the remaining arguments are the signal parameters.

The `connect_<signal_name>` function additionally takes the [Qt connection type](https://doc.qt.io/qt-6/qt.html#ConnectionType-enum) as a parameter.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_connect}}
```

Each connection returns a `QMetaObjectConnectionGuard`, which is a RAII wrapper around the [`QMetaObject::Connection`](https://doc.qt.io/qt-6/qmetaobject-connection.html) and automatically disconnects the connection when the guard is dropped.
This is similar to C++ `std::lock_guard`, `std::unique_ptr`, or Rusts `Box`.

Example:

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_disconnect}}
```

If you don't want to store the `QMetaObjectConnectionGuard`, call `release`, which will turn it into the internal `QMetaObjectConnection`, which is a direct wrapper of `QMetaObject::Connection` and doesn't disconnect on drop.

> **📝 Note**: The `QMetaObjectConnection` has a `disconnect` method which can be called manually later

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
