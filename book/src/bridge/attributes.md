<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Attributes

Most items in the bridge support the `#[namespace]`, `#[cxx_name=...]` and `#[rust_name=...]` attributes.

## `namespace`

The C++ `namespace` which to emit `extern "RustQt"` items and the namespace to find `extern "C++Qt"` items.

An item will inherit the namespace specified on it's surrounding `extern` block if any,
otherwise the namespace specified with the top level `cxx_qt::bridge` attribute, if any, will be used.

> **📝 Note**: The `#[namespace = "..."]` attribute is not allowed on signals, methods or inherited methods

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/threading.rs:book_namespace_macro}}
```

## `cxx_name` and `rust_name`

The `#[cxx_name = "..."]` attribute replaces the name that C++ should use for this item.

The `#[rust_name = "..."]` attribute replaces the name that Rust should use for this item.

For [`#[qproperty]`](./extern_rustqt.md#properties), a CXX or Rust name can be provided inside the attribute like so

`#[qproperty(T, name, cxx_name = "MyName", rust_name = "my_name")]`

> **📝 Note**: If an item has different C++ and Rust identifiers, it is always referenced by its Rust identifier inside the bridge, not its C++ identifier. (e.g. when referring to a QObject inside a `#[qenum(...)]` attribute)

> **⚠️ Deprecation warning**:
> CXX-Qt <0.6 did automatic case conversion if no `#[cxx_name = "..."]` or `#[rust_name = "..."]` is specified.
> Starting with CXX-Qt 0.7, this is no longer the case! Automatic case conversion will be opt-in instead.

### Automatic case conversion

The `#[auto_cxx_name]` and `#[auto_rust_name]` attributes can be used to automatically rename cxx and rust names.
These are placed at a block level on `extern "RustQt"` or `extern "C++Qt"` blocks, and will automatically case convert the items inside, unless they specify either a `rust_name` or `cxx_name`.
By default `#[auto_cxx_name]` will generate a camelCase conversion for`cxx_name` and `#[auto_rust_name]` will generate a snake_case conversion for `rust_name`.
