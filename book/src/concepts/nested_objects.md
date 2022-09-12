<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Nested Objects

During the transition to CXX-Qt 0.4, Nested object support is temporarily removed.

We are currently reworking the way nested objects work.
They will be back, and better than ever 😃.

<!--
Rust Qt objects can be nested as properties or parameters of each other.

A nested object is referred to by it's path relative to `crate`, the second last segment needs `cxx_qt_` as the start of the module name, and then `CppObj` as the last segment. Eg `crate::mymod::cxx_qt_secondary_object::CppObj` refers a `mymod.rs` which contains a module `secondary_object` with [CXX-Qt macros](../qobject/macro.md).

To use this as a property in another object write `secondary_object: crate::mymod::cxx_qt_secondary_object::CppObj` as the property.

For use as a parameter in an invokable write `secondary_object: &mut crate::mymod::cxx_qt_secondary_object::CppObj` as the parameter. Then the `secondary_object` parameter can be used via the normal [`CppObj`](../qobject/cpp_object.md) methods.

The following example shows a nested object as a property and parameter.

```rust,ignore,noplayground
#include ../../../examples/qml_features/rust/src/nested.rs:book_macro_code
```

Note that until nested objects are `UniquePtr<T>` on the Rust side we need to use `cxx_qt_` as a prefix in the last module path to reach the correct `CppObj`.

Note that nested objects cannot be used as return types yet ( [https://github.com/KDAB/cxx-qt/issues/66](https://github.com/KDAB/cxx-qt/issues/66) ).

Note that nested objects are ignored from (de)serialisation ( [https://github.com/KDAB/cxx-qt/issues/35](https://github.com/KDAB/cxx-qt/issues/35) ).

Note that nested objects cannot be used in signals ( [https://github.com/KDAB/cxx-qt/issues/73](https://github.com/KDAB/cxx-qt/issues/73) ).

Note that we may allow for `super::` to be used in the future ( [https://github.com/KDAB/cxx-qt/issues/44](https://github.com/KDAB/cxx-qt/issues/44) ).

TODO: once we have borrow_rust_obj() explain it's purpose of reaching the other objects RustObj [https://github.com/KDAB/cxx-qt/issues/30](https://github.com/KDAB/cxx-qt/issues/30) ).
-->
