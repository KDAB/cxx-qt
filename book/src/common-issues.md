<!--
SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->
# Common Issues

## Cargo Linker Error: Undefined reference to `cxx_qt_init_`

CXX-Qt recreates Qt's resource initialization system within a mix of Cargo and CMake.

This initialization system generates functions that are prefixed with `cxx_qt_init_crate` or `cxx_qt_init_qml_module`.

When building with Cargo, under certain crate setups you may encounter errors that the linker cannot find these functions, e.g.:

```shell
= note: /.../out/cxx-qt-build/qml_module_com_kdab_cxx_qt_demo/call-initializers.cpp:2:
          error: undefined reference to 'cxx_qt_init_qml_module_com_kdab_cxx_qt_demo'
        /.../out/cxx-qt-build/initializers/crate_another_crate/call-initializers.cpp:2:
          error: undefined reference to 'cxx_qt_init_crate_another_crate'
        clang: error: linker command failed with exit code 1 (use -v to see invocation)

  = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib)
```

To fix this issue, you need to make sure of two things:

### 1. Ensure dependencies are used

If a dependency is not used by the target currently being built, the Rust toolchain will not link to it.
This is particularly common if a dependency provides a QML module creates types for use in QML that aren't actually needed by the Rust code of downstream crates.

To fix this, force the Rust compiler to link to the crate by adding:

```rust,ignore
extern crate another_crate;
```

(where another_crate is replaced by the name of the dependency that isn't otherwise used).

### 2. Include the initializers in your code

Next we need to ensure the initializers can be found by the linker.

If you followed step 1, modern linkers like `mold` or `lld` should already be able to link everything correctly.
We encourage switching to such a linker if you're still using the (now deprecated) `ld.gold` on Linux.

With older linkers, you can force initialization manually by calling the corresponding `init_` macros from the cxx_qt crate at startup.

```rust,ignore
fn main() {
  cxx_qt::init_crate!(another_crate);
  cxx_qt::init_qml_module!("com.kdab.cxx_qt.demo");
}
```

Note that you will have to do the same in tests and doc-tests:

````rust,ignore
/// ```
/// # cxx_qt::init_crate!(another_crate);
/// # cxx_qt::init_qml_module!(another_crate);
///
/// X::do_something();
/// ```
struct X {}

#[cfg(test)]
mod tests {
  #[test]
  fn initialize_eependencies() {
    cxx_qt::init_crate!(another_crate);
    cxx_qt::init_qml_module!("com.kdab.cxx_qt.demo");
  }
}
````
