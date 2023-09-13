<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Attributes

## namespace

The C++ namespace which to emit `extern "RustQt"` items and the namespace to find `extern "C++Qt"` items.

An item will inherit the namespace specified on it's surrounding `extern` block if any,
otherwise the namespace specified with the top level `cxx_qt::bridge` attribute, if any, will be used.

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/rust/src/threading.rs:book_namespace_macro}}
```

> Note that `#[namespace = "..."]` may not work on all items,
> we hope to improve this in future this support in the future.

## cxx_name and rust_name

The `#[cxx_name = "..."]` attribute replaces the name that C++ should use for this item.

The `#[rust_name = "..."]` attribute replaces the name that Rust should use for this item.

> Note that `#[cxx_name = "..."]` and `#[rust_name = "..."]` may not work on all items,
> we hope to improve this in future this support in the future.

If no `#[cxx_name = "..."]` or `#[rust_name = "..."]` is specified, CXX-Qt will perform an automatic conversion to function names as specified in the table below.

|                  | Rust       | C++       |
|------------------|------------|-----------|
| `extern "C++Qt"` | -          | camelCase |
| `extern "RustQt"`| -          | camelCase |

> Note that in some cases `snake_case` conversions may occur for generated functions in Rust (eg `on_<signal>`).

> Note that this table may change to the following conversions in the future.
>
> |                  | Rust       | C++       |
> |------------------|------------|-----------|
> | `extern "C++Qt"` | snake_case | -         |
> | `extern "RustQt"`| -          | camelCase |
