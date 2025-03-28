<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Traits

CXX-Qt uses multiple traits to cleanly encode its behavior and supported features into Rusts type system.

Some of these traits use a special syntax inside the [`#[cxx_qt::bridge]`](../bridge/index.md)
similarly to [explicit shim trait impls](https://cxx.rs/extern-c++.html#explicit-shim-trait-impls) in CXX.
Depending on the trait, this either tells CXX-Qt that it should automatically implement the trait, or that it should use an existing trait implementation for code generation.

```rust,ignore
impl UniquePtr<A> {} // explicit CXX trait implementation of UniquePtr for A

impl cxx_qt::Trait for A {} // explicit CXX-Qt trait implementation of Trait for A
```

For further documentation, refer to the documentation of the individual traits:

- [CxxQtType](https://docs.rs/cxx-qt/latest/cxx_qt/trait.CxxQtType.html) - trait to reach the Rust implementation of a `QObject`
  - This trait is automatically implemented for any `#[qobject]` type inside `extern "RustQt"` blocks.
- [Constructor](https://docs.rs/cxx-qt/latest/cxx_qt/trait.Constructor.html) - custom constructor
- [Initialize](https://docs.rs/cxx-qt/latest/cxx_qt/trait.Initialize.html) - execute Rust code when the object is constructed, or as shorthand for an empty constructor
- [Threading](https://docs.rs/cxx-qt/latest/cxx_qt/trait.Threading.html) - marker trait whether CXX-Qt threading should be enabled

> ⚠️ These traits should only be implemented if you are sure you need to, they are automatically implemented for RustQt types.

- [Upcast](https://docs.rs/cxx-qt/latest/cxx_qt/trait.Upcast.html) - Allows a type to access its parent class if there is one
- [Downcast](https://docs.rs/cxx-qt/latest/cxx_qt/trait.Downcast.html) - Allows a type to access its child class if there is one
