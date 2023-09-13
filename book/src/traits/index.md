<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Reference: traits

Traits can be implemented (or in some cases negated) inside the [`#[cxx_qt::bridge]`](../bridge/index.md)
in a similar way to [explicit shim trait impls](https://cxx.rs/extern-c++.html#explicit-shim-trait-impls) in CXX.

Except instead of the `T` being a generic it is the struct the trait is implemented for.
This is because some of the traits themselves require generics.

```rust,ignore
impl UniquePtr<A> {} // explicit CXX trait implementation of UniquePtr for A

impl cxx_qt::Trait for A {} // explicit CXX-Qt trait implementation of Trait for A
```

- [CxxQtType](./cxxqttype.md) - trait to reach the Rust implementation of a QObject
- [Constructor](./constructor.md) - custom constructor
- [Initialize](./initialize.md) - execute Rust code when the object is constructed
- [Locking](./locking.md) - marker trait whether locking is enabled
- [Threading](./threading.md) - marker trait whether CXX-Qt threading should be enabled
