<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# clang-format

A basic clang-format Rust wrapper.

This allows for formatting a given input using `clang-format` from the system.

```rust
use clang_format::{clang_format, ClangFormatStyle, CLANG_FORMAT_STYLE};

fn main() {
    CLANG_FORMAT_STYLE.set(ClangFormatStyle::Mozilla);

    let input = r#"
        struct Test {

        };
    "#;
    let output = clang_format(input);
    assert!(output.is_ok());
    assert_eq!(output.unwrap(), "\nstruct Test\n{};\n");
}
```
