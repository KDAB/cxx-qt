<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Locking

CXX-Qt locking is enabled by default to ensure that Rust compiler assumptions remain true.
With these assumptions CXX-Qt can then also provide features such as [threading](./threading.md) safely.

For example the locking prevents C++ from triggering two Rust functions in the same QObject at the same time.

In certain scenarios it might be useful to disable locking that occurs when the context switches from C++ to Rust.

To disable the generation of locking use an unsafe negation inside the [`#[cxx_qt::bridge]`](../bridge/index.md).

```rust,ignore
{{#include ../../../tests/basic_cxx_qt/rust/src/locking.rs:book_disable_locking}}
```

> If locking is disabled the [`cxx_qt::Threading`](./threading.md) trait can not be enabled on the object.
