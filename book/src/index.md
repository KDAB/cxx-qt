<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# CXX-Qt - Safe interop between Rust and Qt

<p align=center><a href="./getting-started/index.md">TLDR: Click here for "Getting Started" guide</a></p>

This library provides a safe mechanism for bridging between Qt code and Rust code differently from typical Rust Qt bindings.

We acknowledge that Qt code and Rust code have different idioms so cannot be directly wrapped from one to another.

Instead of one-to-one bindings we use [CXX](https://cxx.rs/) to [bridge](./bridge/index.md) between, this allows for normal Qt code and normal Rust code.

We feel this is more powerful than typical bindings as this allows us to provide a safe API and safe multi-threading between Qt and Rust.

To aid integration of Qt and Rust code we provide common [Qt types](./concepts/types.md) for Rust which can pass across the bridge and provide ways to express common Qt idioms.

Through the use of macros and code generation as seen in the figure below, the developer describes a `QObject` with CXX-Qt macro annotations. Then CXX-Qt generates the C++ representation of the object and uses macro expansion to define the [CXX](https://cxx.rs/) bridge for interop between C++ and Rust.

<div style="background-color: white; padding: 1rem; text-align: center;">

![Overview of CXX-Qt concept](./images/overview_abstract.svg)

</div>

If you are new to CXX-Qt, we recommend you visit our [Getting Started Guide](./getting-started/index.md).

To get detailed information on which features are available in CXX-Qt, see the [bridge chapter](./bridge/index.md).
Should you be interested in a deeper dive into the concepts of CXX-Qt, take a look at the [concepts chapter](./concepts/index.md), which explains the concepts CXX-Qt introduces in detail.

**Note:** CXX-Qt is tested on CI on Linux, Windows, and macOS (all on x86_64). It should work on other platforms that Qt and Rust both support, however, these are not tested regularly.
