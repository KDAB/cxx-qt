<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# `extern "C++Qt"`

- [`extern "C++Qt"`](#extern-cqt)
  - [`QObject`s](#qobjects)
  - [Methods](#methods)
  - [Signals](#signals)

```rust,ignore,noplayground
#[cxx_qt::bridge]
mod ffi {
    extern "C++Qt" {

    }
}
```

The `extern "C++Qt"` section of a CXX-Qt bridge declares Qt types and signatures to be made available to Rust,
and gives the paths of the headers which contain the corresponding Qt declarations.

A bridge module may contain zero or more `extern "C++Qt"` blocks.

This complements the [`extern "C++"` CXX section](https://cxx.rs/extern-c++.html)
but allows for declaring Qt specific features on C++ types.

Automatically converting to camel or snake case can be done through an [attribute](./attributes.md#automatic-case-conversion) at the block level.

## `QObject`s

QObject types that are defined in C++ can be made available to Rust, by declaring them as [opaque types](https://cxx.rs/extern-c++.html#opaque-c-types) with a `#[qobject]` attribute.

> **📝 Note**: Types inside `extern "C++Qt"` are currently required to have a `#[qobject]` attribute

```rust,ignore,noplayground
#[cxx_qt::bridge]
mod ffi {
    extern "C++Qt" {
        include!(<QtWidgets/QPushButton>);
        #[qobject]
        type QPushButton;
    }
}
```

<!--
TODO: use a real example from qml_features once closure support lands
-->

## Methods

Methods can be specified on the QObject type in the same way as [`extern "RustQt"` blocks](./extern_rustqt.md#methods).

This is the same as [CXX Functions and member functions](https://cxx.rs/extern-c++.html#functions-and-member-functions).

```rust,ignore,noplayground
#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "C++Qt" {
        include!(<QtWidgets/QPushButton>);
        #[qobject]
        type QPushButton;

        fn text(self: &QPushButton) -> QString;
        fn setText(self: Pin<&mut QPushButton>, text: &QString);
    }
}
```

<!--
TODO: use a real example from qml_features once closure support lands
-->

## Signals

Signals can be specified on the Qt type in the same way as [`extern "RustQt"` blocks](./extern_rustqt.md#signals).

```rust,ignore,noplayground
#[cxx_qt::bridge]
mod ffi {
    extern "C++Qt" {
        include!(<QtWidgets/QPushButton>);
        #[qobject]
        type QPushButton;

        #[qsignal]
        fn clicked(self: Pin<&mut QPushButton>, checked: bool);
    }
}
```

This then causes CXX-Qt to generate Rust methods to emit and connect to the `#[qsignal]`,
in the same way as a `#[qsignal]` in a [`extern "RustQt"` block](./extern_rustqt.md#signals).

> **📝 Note**: Using `pub(self)` as the visibility of the signal allows for declaring private signals

<!--
TODO: use a real example from qml_features once closure support lands
-->
