<!--
SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Instantiating `QObject`s directly in Rust

Your `QObject` types will most likely be instantiated via QML, but it is possible to create them in Rust via a template.
By adding

```rust,ignore
#[namespace = "rust::cxxqtlib1"]
unsafe extern "C++" {
    include!("cxx-qt-lib/common.h");

    #[cxx_name = "make_unique"]
    #[doc(hidden)]
    fn myobject_make_unique() -> UniquePtr<MyObject>;
}
```

You can directly create an instance of your object wrapped in a `UniquePtr` within Rust, should you wish.
The included header file contains some wrapper templates for constructing `unique_ptr<T>`, `shared_ptr<T>` and `*T`.
By exposing this to the bridge with the correct namespace, constructing these structs is possible in Rust.

## Possible Methods

| Name          | C++ Return Type | Rust Return Type |
|---------------|-----------------|------------------|
| `make_unique` | `unique_ptr<T>` | `UniquePtr<T>`   |
| `make_shared` | `shared_ptr<T>` | `SharedPtr<T>`   |
| `new_ptr`     | `*T`            | `*mut T`         |
