<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Signals enum

The `cxx_qt::qsignals(T)` attribute is used on an [enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) to define [signals](https://doc.qt.io/qt-6/signalsandslots.html) for the QObject `T`.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_enum}}
```

For every enum variant, CXX-Qt will generate a signal on the corresponding QObject.
If the enum variant has members, they will become the parameters for the corresponding signal.

Because CXX-Qt needs to know the names of each parameter, only enum variants with named members are supported.
The signal parameters are generated in order of appearance in the enum variant.

If a signal is defined on the base class of the QObject then `#[inherit]` can be used to indicate to CXX-Qt that the `Q_SIGNAL` does not need to be created in C++.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/custom_base_class.rs:book_qsignals_inherit}}
```

Note that `#[cxx_name = "..."]` can also be used on a signal to declare a different name in C++ to Rust.

## Connecting to a signal

For every signal defined in the enum, a method is generated called `on_<signal_name>`.
This method takes a function as the first parameter.
That function's first argument is the qobject and the remaining arguments are the signal parameters.
The second parameter of the `on_<signal_name>` method is the [Qt connection type](https://doc.qt.io/qt-6/qt.html#ConnectionType-enum).

Note that by using the `#[inherit]` macro on a signal, connections can be made to property changes
using the signal name `<property>Changed` with no parameters.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_connect}}
```

Each connection returns a [`QMetaObject::Connection`](https://doc.qt.io/qt-6/qmetaobject-connection.html) which can be disconnected later by calling its `disconnect` method.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_disconnect}}
```

## Emitting a signal

For every generated QObject [`qobject::T`](./generated-qobject.md) that has a signals enum, CXX-Qt will generate an `emit` function:
``` rust,ignore,noplayground
fn emit(self: Pin<&mut Self>, signal: /*Signals enum*/)
```
`emit` can therefore be called from any mutable `#[qinvokable]`.

The `emit` function will immediately emit the signal.
Depending on the connection type, the connected slots will be called either immediately or from the event loop (See [the different connection types](https://doc.qt.io/qt-6/qt.html#ConnectionType-enum)).
To queue the call to `emit` until the next cycle of the Qt event loop, you can use the [`CxxQtThread`](./cxxqtthread.md).

### [Example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/signals.rs)
```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_macro_code}}
```

