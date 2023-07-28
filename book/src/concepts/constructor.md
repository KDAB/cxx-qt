<!-- 
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Defining a custom C++/QML constructor

By default, CXX-Qt will generate a constructor that takes a single `QObject` pointer as optional argument (usually the QObject parent).

It then:
1. Calls the base class constructor with the optional parent pointer.
2. uses [Rust's default trait][default-trait] to construct the inner Rust struct.

For most cases this is good enough, however there are a few cases in which this will not be sufficient, for example:
* the base class constructor expects arguments other than the parent QObject pointer, or a different type than QObject is used for the parent (for example QQuickItem's constructor takes a QQuickItem pointer).
* the QObject needs to run code after it is initialized (e.g. open a connection to a server, etc.).
* the Rust struct can't have a `default` implementation because it needs certain arguments for construction.

To facilitate these use-cases CXX-Qt provides the [`Constructor` trait][constructor-trait].

## Implementing & Declaring a Constructor
In order for CXX-Qt to generate a custom constructor, the `qobject::T` must implement the [`cxx_qt::Constructor` trait][constructor-trait].
Additionally, the constructor must be declared within the [cxx_qt::bridge](./bridge.md).

This declaration uses Rust's `impl ... for` syntax, but with a few special rules:
* The implementation block must be empty
* If any of the associated types in the constructor are not `()`, they
    must be listed using `Type=(...)` in the constructor generics.

Example:
```rust,ignore,noplayground
#[cxx_qt::bridge]
mod qobject {
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_struct}}

{{#include ../../../examples/qml_features/rust/src/signals.rs:book_constructor_decl}}
}
// Don't forget to actually implement the trait **outside** the bridge!
```
[Full example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/signals.rs)

## Routing arguments of the Constructor
A C++ constructor is more complex than a Rust struct initialization.

Like a normal function, it takes a list of arguments.
However, before the actual code inside the constructor is executed, C++ ensures the following:
* The base class is constructed
* All members of the class or struct are initialized (for CXX-Qt, this includes the underlying Rust struct).

Both of these steps may need any of the arguments given to the constructor.
The code that is actually run *inside* the constructor may also need to access some of the arguments.

`cxx_qt::Constructor` allows controlling how the arguments are passed around in the constructor using the `route_arguments` function.
For this it uses three associated types as well as one generic type:
* Constructor generic `Arguments` - argument list that the constructor takes.
* `type BaseArguments` - argument list that is passed to the base class constructor.
* `type NewArguments` - argument list used to construct the Rust struct using the `new` function.
* `type InitializeArguments` - argument list that is passed to the code run within the constructor (the `initialize` function).

These types must all be tuple types and can therefore support an arbitrary list of arguments.

The `route_arguments` function takes the Arguments provided to the constructor and distributes them to the three different types in the above list.
Doing this in Rust allows calling any base class constructor and controlling movement, copying or other operations on the arguments.
Every constructor defined using `cxx_qt::Constructor` will execute this function once, before anything else in the Constructor happens and then distributes the resultant values to the appropriate steps of the QObject construction.

As actual construction takes place in C++, all types used must be compatible with CXX so they can pass the FFI barrier.

## Constructing the Rust struct with arguments
If the construction of the inner Rust struct requires some arguments, the `new` function of the Constructor can be given arguments by using the `NewArguments` associated type.

Example:
```rust,ignore
#[cxx_qt::bridge]
mod qobject {
    // ...

{{#include ../../../examples/qml_features/rust/src/properties.rs:book_constructor_new_decl}}
}

{{#include ../../../examples/qml_features/rust/src/properties.rs:book_constructor_new_impl}}
```

## Initializing the QObject
In addition to running code before constructing the inner Rust struct, it may be useful to run code from the context of the QObject itself (i.e. inside the Constructor implementation).

The `initialize` function can be used to run code inside a constructor.
It is given a pinned mutable self reference to the QObject and the list of `InitializeArguments`.

### Using the `Initialize` trait
The QML engine creates QML elements using their default constructor, so for most QML types only the `initialize` part of the constructor is of interest.
To reduce the boilerplate of this use-case, CXX-Qt provides the [`Initialize`][initialize-trait] trait.

If a QObject implements the `Initialize` trait, and the inner Rust struct is [Default][default-trait]-constructible it will automatically implement `cxx_qt::Constructor<()>`.

Example:
```rust,ignore
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_initialize_impl}}
        // ...
    }
}
```
[Full example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/signals.rs)

Then just remember to declare the `cxx_qt::Constructor<()>` inside the `cxx_qt::bridge`.
```rust,ignore
#[cxx_qt::bridge]
mod qobject {
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_struct}}

{{#include ../../../examples/qml_features/rust/src/signals.rs:book_initialize_decl}}
}
```

[constructor-trait]: https://docs.rs/cxx-qt/latest/cxx_qt/trait.Constructor.html
[initialize-trait]: https://docs.rs/cxx-qt/latest/cxx_qt/trait.Initialize.html
[default-trait]: https://doc.rust-lang.org/std/default/trait.Default.html
