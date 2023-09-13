<!--
SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CxxQtType

<!--
TODO: link to docs.rs ?
-->

The `cxx_qt::CxxQtType` trait is automatically implemented for all types which are marked with a [`#[qobject]`](../bridge/extern_rustqt.md#qobjects) attribute.

It provides `rust` and `rust_mut` accessors to reach the inner Rust type that is wrapped by the QObject.

> Note that the `rust_mut` method needs a `Pin<&mut T>` where `T` is the QObject type
