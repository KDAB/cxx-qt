<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Shared types

## `#[qenum]` - Support for Q_ENUM and Q_ENUM_NS

Qt allows exposing enums to Qt's meta-object system, and thereby QML, with a set of macros:

- [Q_ENUM][qenum] is used to expose an enum that is a member of a [QObject](../concepts/generated_qobject.md)
- [Q_ENUM_NS][qenum-ns] is used to expose an enum that is in a namespace to the meta-object system.

CXX-Qt has support for both of these macros through the `#[qenum]` attribute.

## QObject class enum (`Q_ENUM`)

CXX-Qt relies on CXX to expose enums from Rust to C++ and vice-versa.
However, CXX only supports free enums that are not defined as part of a class.
CXX-Qt doesn't change this, it only additionally exposes the enum as part of a QObject type to the meta-object system.
So any `#[qenum]` in CXX-Qt is available as both a normal shared CXX enum, as well as a Q_ENUM inside the associated QObject.

To expose a [shared CXX enum][shared-cxx-enums] as a [`Q_ENUM`][qenum] inside a QObject class, add the `#[qenum(...)]` attribute to the enum definition in CXX.
The argument to `#[qenum(...)]` must be the name of a `#[qobject]` that is defined in a `extern "RustQt"` block.

It is currently not possible to add a `#[qenum(...)]` to any `extern "C++Qt"` QObjects or a QObject that is defined in another `#[cxx_qt::bridge]`.

Example:
```rust,ignore,noplayground
#[cxx_qt::bridge(cxx_file_stem="custom_base_class")]
pub mod qobject {
{{#include ../../../examples/qml_features/rust/src/custom_base_class.rs:book_qenum_in_qobject}}

{{#include ../../../examples/qml_features/rust/src/custom_base_class.rs:book_qobject_base}}
}
```

### Registering the enum with QML

Note that Qt provides access to enum variants through the name of the class it is registered with, not the enum name itself.
A side-effect of this behavior is that the enum itself doesn't have to be registered with QML.
Only the QObject class has to be registered.
In the previous example, the `#[qml_element]` attribute on the `#[qobject]` takes care of the registration.

Usage from QML:
```qml
{{#include ../../../examples/qml_features/qml/pages/CustomBaseClassPage.qml:book_qenum_access}}
```


## Namespaced enum (`Q_ENUM_NS`)

If there is no class that the enum should be associated with, Qt still allows exposing the enum to the meta-object system, as long as it is inside a namespace.

If there is a namespace associated with a [shared CXX enum][shared-cxx-enums] simply add the `#[qenum]` attribute and CXX-Qt will expose it using [`Q_ENUM_NS`][qenum-ns].

Note that the namespace doesn't have to be specified on the enum directly, the enum can inherit the namespace from the surrounding bridge.
This follows normal [CXX namespacing rules](https://cxx.rs/attributes.html#namespace).

Example:
```rust,ignore,noplayground
#[cxx_qt::bridge]
pub mod qobject {
{{#include ../../../examples/qml_features/rust/src/invokables.rs:book_namespaced_qenum}}
}
```

Unfortunately, an important Qt limitation also applies to CXX-Qt.
Namely, for any given namespace, there must be at most **one** bridge that exposes `#[qenum]` enums through that namespace.
One bridge may expose enums through multiple namespaces however.

### Registering the enum with QML

Whilst `Q_ENUM_NS` creates the appropriate meta-objects, it doesn't add them to QML automatically.
Like with `Q_ENUM`, access to the enum variants also doesn't happen through the enum directly, but rather the surrounding namespace.

Therefore, the namespace must be registered with the meta-object system and then exposed to QML.
CXX-Qt automatically registers the namespace of a namespaced `#[qenum]` with the meta-object system.

Registration with QML can then be done by placing a `qnamespace!("...")` macro inside the bridge that defines the namespaced `#[qenum]` and adding a `#[qml_element]` attribute.

```rust,ignore,noplayground
#[cxx_qt::bridge]
pub mod qobject {
{{#include ../../../examples/qml_features/rust/src/invokables.rs:book_qnamespace}}

{{#include ../../../examples/qml_features/rust/src/invokables.rs:book_namespaced_qenum}}
}
```

Usage from QML:
```qml
{{#include ../../../examples/qml_features/qml/pages/InvokablesPage.qml:book_namespaced_qenum}}
```

[shared-cxx-enums]:https://cxx.rs/shared.html#shared-structs-and-enums
[qenum-ns]:https://doc.qt.io/qt-6/qobject.html#Q_ENUM_NS
[qenum]:https://doc.qt.io/qt-6/qobject.html#Q_ENUM
