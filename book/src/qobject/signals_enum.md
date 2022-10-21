<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Signals enum

The `cxx_qt::signals(T)` attribute is used on an [enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) to define [signals](https://doc.qt.io/qt-6/signalsandslots.html) for the QObject `T`.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_signals_enum}}
```

For every enum variant CXX-Qt will generate a signal on the corresponding QObject.
If the enum variant has members, they will become the parameters for the corresponding signal.

Because CXX-Qt needs to know the names of each parameter, only enum variants with named members are supported.
The Signal parameters are generated in order of appearance in the enum variant.

## Emitting a signal

For every generated QObject [`qobject::T`](./generated-qobject.md) that has a signals enum CXX-Qt will generate an `emit` function:
``` rust,ignore,noplayground
fn emit(self: Pin<&mut Self>, signal: /*Signals enum*/)
```
`emit` can therefore be called from any mutable `#[qinvokable]`.

The `emit` function will immediately emit the signal and call any connected slots.
To queue the call to `emit` until the next Event loop cycle, you can use the [`CxxQtThread`](./cxxqtthread.md).

### [Example](https://github.com/KDAB/cxx-qt/blob/main/examples/qml_features/rust/src/signals.rs)
```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/signals.rs:book_macro_code}}
```

