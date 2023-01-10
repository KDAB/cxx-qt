<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->
# Type Conversions

As the primary use for CXX-Qt is to describe a Qt API in Rust, this means that for technical reasons the required Qt API will sometimes be slightly different from the Rust API.

For example, when dealing with [opaque types](./types.html#opaque-type-conversions), these can never be passed by-value from/to Rust.
In most cases, you'll therefore use a UniquePtr in Rust when you need ownership of such an object.

However, Qt often requires values to be returned by-value, especially when compatibility with QML is required.
CXX-Qt therefore allows you to automatically convert certain Rust types to certain C++ types.

For example, CXX-Qt can convert a `::std::unique_ptr<QColor>` to a `QColor` for you.

CXX-Qt by default provides conversions for:

| From                  | To            |                                           |
|-----------------------|---------------|-------------------------------------------|
| any type `T`          | any type `R`  | If `T` can be implicitly converted to `R` |
| `::std::unique_ptr<T>`  | T             | if T is moveable |
| `::std::unique_ptr<T>`  | T&            | |

## Defining your own converter
CXX-Qt uses the templated `::rust::cxxqtlib1::cxx_qt_convert` struct for these conversions.
In fact, an error message on this struct might have brought you here 😉.

Before starting to define your own type converter, make sure this is actually what you want!!!
Is the C++ compiler complaining about two types that you think conversion should be possible between?
If not, you might have a bug in your `cxx_qt::bridge`.
Double-check that you know where the error originates from and that the types are correct.

However, if you're confident that there is a conversion missing, consider [opening an issues on the CXX-Qt repo](https://github.com/KDAB/cxx-qt/issues/new).
Then you can use template specialization on the `cxx_qt_convert` struct to provide your own type conversion that CXX-Qt will use.

Example:
``` c++
namespace rust {
namespace cxxqtlib1 {

template <>
struct cxx_qt_convert</*TO*/int, /*FROM*/MyCustomType> {
    int operator()(MyCustomType value) {
        return /*YOUR CONVERSION HERE*/;
    }
}

}
}
```

Then make sure to `include!` it in your cxx_qt::bridge.
E.g.:
``` rust,ignore
#[cxx_qt::bridge]
mod ffi {
  extern "C++" {
    include!("my_custom_converter.h");
  }

  // ...
}
```
