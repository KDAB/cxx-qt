<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Build.rs

We need to specify a build.rs so that we can parse the macros and generate relevant C++ code.

The following options are available

  * Indicating which files should be parsed to look for macros
  * Deciding the clang-format style of the generated C++ code
  * Specifiying a custom C++ namespace for the generated Rust types

A build.rs script could look like the following

```rust,ignore,noplayground
{{#include ../../../examples/qml_minimal/build.rs:book_build_rs}}
```

A non-default C++ namespace could be like the following

Note that the namespace is a list, so `vec!["a", "b", "c"]` would become `a::b::c`

```rust,ignore,noplayground
{{#include ../../../examples/qml_features/build.rs:book_build_rs}}
```

# Cargo.toml

The `Cargo.toml` file of your project needs minimal changes to work with CXX-Qt.

Firstly we currently need to build as a static library (as the Rust library is statically linked into the C++ executable or library).

```cargo
{{#include ../../../examples/qml_minimal/Cargo.toml:book_static_lib}}
```

Then the following dependencies are required for CXX-Qt to be used in the project.

```cargo
{{#include ../../../examples/qml_minimal/Cargo.toml:book_dependencies}}
```

Finally the following build dependencies are required for the build.rs file to function.

```cargo
{{#include ../../../examples/qml_minimal/Cargo.toml:book_build_dependencies}}
```

Note that for the dependencies if you are using [crates.io](https://crates.io/) then you don't need the path parameter and can place the version as usual (eg `cxx-qt = "0.3"`).
