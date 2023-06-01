<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Signals enum

The `cxx_qt::qsignals` attribute is used on an `extern "C++"` block to define [signals](https://doc.qt.io/qt-6/signalsandslots.html) for the a QObject.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_block}}
```

For every function signature in the extern block, CXX-Qt will generate a signal on the corresponding QObject.
If the function has parameters, they will become the parameters for the corresponding signal.

If a signal is defined on the base class of the QObject then `#[inherit]` can be used to indicate to CXX-Qt that the `Q_SIGNAL` does not need to be created in C++.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/custom_base_class.rs:book_qsignals_inherit}}
```

Note that `#[cxx_name = "..."]` can also be used on a signal to declare a different name in C++ to Rust.

## Connecting to a signal

For every signal defined in the enum, two methods are generated.

  1. `on_<signal_name>`
  2. `connect_<signal_name>`

The `on_<signal_name>` method takes a handler function as the parameter, which will be called when the signal is emitted.
That handler function's first argument is the qobject and the remaining arguments are the signal parameters.

The `connect_<signal_name>` function additionally takes the [Qt connection type](https://doc.qt.io/qt-6/qt.html#ConnectionType-enum) as a parameter.

Note that by using the `#[inherit]` macro on a signal, connections can be made to property changes
using the signal name `<property>Changed` with no parameters.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_connect}}
```

Each connection returns a [`QMetaObject::Connection`](https://doc.qt.io/qt-6/qmetaobject-connection.html) which is used to manage the connection.

Note that the `QMetaObjectConnection` returned by CXX-Qt behaves a bit different from the Qt C++ implementation.

When the `QMetaObjectConnection` is dropped, it automatically disconnects the connection, similar to how a C++ `std::unique_ptr` or Rusts `Box` behaves.
If you don't want to store the QMetaObjectConnection, call `release`, which will drop the object without disconnecting.
In this case, it is no longer possible to disconnect later.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_disconnect}}
```

## Emitting a signal

Call the function signature defined in the `extern "C++` block to emit the signal.

Note that these are defined on the generated QObject [`qobject::T`](./generated-qobject.md), so can be called from any mutable `#[qinvokable]`.

The function will immediately emit the signal.
Depending on the connection type, the connected slots will be called either immediately or from the event loop (See [the different connection types](https://doc.qt.io/qt-6/qt.html#ConnectionType-enum)).
To queue the call until the next cycle of the Qt event loop, you can use the [`CxxQtThread`](./cxxqtthread.md).

### [Example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/signals.rs)

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_macro_code}}
```
