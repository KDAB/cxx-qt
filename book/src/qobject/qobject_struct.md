<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# `#[cxx_qt::qobject]` Macro - Defining QObjects in Rust

Defining QObjects is at the heart of CXX-Qt.
Therefore `#[cxx_qt::qobject]` can be considered the most important macro in CXX-Qt.

## Requirements
- Like most other CXX-Qt macros, it can only be used from within a [`#[cxx_qt::bridge]`](./bridge-macro.md).
- The `#[cxx_qt::qobject]` macro must be placed on a Rust struct.
- The struct must [`impl Default`](#default), so that it can be constructed as part of a QObject.

## Effects
Adding the macro to a Rust struct `MyObject` has a few effects.

However, first it's important to mention that this macro **does not modify the contents of the struct in any way!**
This means you can always rely on the struct to behave like any other Rust struct.
You can use it in normal Rust code, without interacting with Qt in any way.

The macro does multiple other things for you though:
- Generate a C++ QObject subclass that wraps the `MyObject` Rust struct.
- Expose the generated QObject subclass to Rust as [`qobject::MyObject`](./generated-qobject.md)
- Generate getters/setters for all fields.
- Generate `Q_PROPERTY`s for all fields that are marked as `#[qproperty]`.
- Generate signals if paired with a [`#[cxx_qt::qsignals]` enum](./signals_enum.md).

## `base` attribute
Use the `base` attribute to specify a C++ class that the C++ QObject will inherit from.
The base class must inherit from QObject (directly or indirectly). If you do not specify a base attribute, it will inherit directly from QObject.

``` rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/custom_base_class.rs:book_qobject_base}}
      // ...
    }
```

Use the CXX `include!` macro to include the appropriate C++ header for the base class:
``` rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/custom_base_class.rs:book_base_include}}
    }
```

[Full Example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/custom_base_class.rs)

## Properties

Fields within the `#[cxx_qt::qobject]` marked struct can be tagged with `#[qproperty]` to be exposed as [`Q_PROPERTY`s](https://doc.qt.io/qt-6/properties.html) on the generated QObject:

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/properties.rs:book_properties_struct}}
```

Any type that CXX supports may be marked as a `#[qproperty]`.
See the [Types page](../concepts/types.md) for a list of supported types.

For every `#[qproperty]`, CXX-Qt will generate setters and getters, as well as a "changed" signal.

On The C++ side:
  * setter: `set<Property>`
  * getter: `get<Property>`
  * changed: `<Property>Changed`

On the Rust side:
  * setter: `set_<Property>`
  * getter: `<Property>`
  * changed: `<Property>_changed`

where `<Property>` is the name of the property.

These setters and getters assure that the changed signal is emitted every time the property is edited.

Any field that's not marked as `#[qproperty]` won't be accessible from C++, but it will be accessible from Rust.
See the [Private fields section](#private-methods-and-fields)

### `cxx_type`

You can change the C++ type that your property uses by adding the `cxx_type` attribute to the `#[qproperty]` macro.

This is especially useful if your property type is an opaque C++ type and can't be owned directly by Rust.
Then it might be necessary to wrap it in a [cxx::UniquePtr](https://docs.rs/cxx/latest/cxx/struct.UniquePtr.html).

However, you may still want your `Q_PROPERTY` to use the opaque type directly.
This can easily be achieved by using `cxx_type`.

``` rust,ignore,noplayground
#[cxx_qt::qobject]
struct MyStruct {
  #[qproperty(cxx_type="OpaqueExampleType")]
  my_property: cxx::UniquePtr<OpaqueExampleType>
}
```
In this case, CXX-Qt will automatically convert any references to the `std::unique_ptr<OpaqueExampleType>` to references to `OpaqueExampleType`.

For details, see the [page on type conversions](../concepts/type-conversions.md).

## Default

The [`Default` trait](https://doc.rust-lang.org/std/default/trait.Default.html) needs to be implemented for the `#[cxx_qt::qobject]` marked struct either by hand or by using the derive macro `#[derive(Default)]`.

This needs to provide default values for every [`#[qproperty]`](#properties) and [private field](#private-methods-and-fields)

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/properties.rs:book_properties_default}}
```

[Full Example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/properties.rs)

## Invokables

Invokables are functions that are registered with the Qt meta-object system using [`Q_INVOKABLE`](https://doc.qt.io/qt-6/qobject.html#Q_INVOKABLE).
This allows them to be called from QML and JavaScript (running in a [QJSEngine](https://doc.qt.io/qt-6/qjsengine.html)), in addition to C++ and Rust.

CXX-Qt allows you to define invokables using Rust code.
This way you can easily add a Rust-powered backend to your QML frontend.

Invokables, by definition, must be defined on a C++ class however.
This is where the QObject subclass generated by `#[cxx_qt::qobject]` comes into play.
For details on this, see the [`qobject::T` page](./generated-qobject.md).

The important part for invokables is that they need to be implemented on the `qobject::T`, not `T`.
Therefore they have access to both C++ and Rust methods. CXX-Qt adds wrapper code around your invokables to automatically convert between the [C++ and Rust types](../concepts/types.md).

To mark a method as invokable, simply add the `#[qinvokable]` attribute to the Rust method. This tells CXX-Qt to expose the method on the generated C++ class.
`Q_INVOKABLE` will be added to the C++ definition of the method, allowing QML to call the invokable.

``` rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/invokables.rs:book_impl_qobject}}
```

Note that an Invokable may only use `self: Pin<&mut Self>` or `&self` as self types.
It is not possible to have a `self`, or `&mut self` invokable, as that may move the QObject in memory, which would invalidate C++ pointers and references to the QObject.
Furthermore, invokables are restricted to only use types that are compatible with CXX.

It is also possible to define methods in the `impl qobject::T` block that are *not* marked as `#[qinvokable]`.
These methods won't be available from C++ or QML.
But they can still access the QObject features like emitting signals and changing properties by accessing `Pin <&mut Self>`.
These are normal Rust methods, so they aren't restricted to CXX-compatible types.

[Full example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/invokables.rs)

## Private Methods and Fields

Fields within your `#[cxx_qt::qobject]` struct that aren't tagged as `#[qproperty]` are not exposed as properties to Qt. These can be considered as "private to Rust" fields, and are useful for storing channels for threading or internal information for the QObject.
Because they aren't available from C++, they also don't have any special type requirements and can be any Rust type.
For convenience, CXX-Qt generates getters and setters on the `qobject::T` for these fields.

These use the convention:
  * setter: `set_<Property>`
  * getter: `<Property>`
  * mutable accessor: `<Property>_mut`.

In comparison to properties, CXX-Qt generates a mutable accessor to the field.
Because the field doesn't correspond to a property, no changed signal has to be emitted and therefore the field can be mutated freely.

Methods implemented using `impl T` (and not `impl qobject::T`) are just normal Rust member methods.
Therefore they do not have access to any C++ or QObject functionality (e.g. emitting signals, changing properties, etc.)
You will usually only need to use `impl T` if you want to also use your struct as a normal Rust struct that is not wrapped in a QObject.
