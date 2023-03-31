<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/KDAB/cxx-qt/compare/v0.5.2...HEAD)

### Changed

- `QDateTime` API to use `current_date_time` rather than `current_date`
- Always call `qt_build_utils::setup_linker()` in `CxxQtBuilder` and remove the proxy method
- Moved to `syn` 2.0 internally and for any exported `syn` types

## [0.5.2](https://github.com/KDAB/cxx-qt/compare/v0.5.1...v0.5.2) - 2023-04-27

### Fixed

- Builds failing due to `link modifiers combination +bundle,+whole-archive is unstable when generating rlibs`

## [0.5.1](https://github.com/KDAB/cxx-qt/compare/v0.5.0...v0.5.1) - 2023-03-27

### Fixed

- `qrc` resources added to `CxxQtBuilder` or `QtBuild` now trigger `cargo:rerun-if-changed` for file entries
- Fix not being able to use `QVariant` as a `#[qproperty]`, because the `PartialEq` implementation was missing

## [0.5.0](https://github.com/KDAB/cxx-qt/compare/v0.4.1...v0.5.0) - 2023-03-08

## Added

- Support for inheriting methods from the superclass into Rust using `#[cxx_qt::inherit]`.
- Register QML types at build time: `#[cxxqt::qobject(qml_uri = "foo.bar", qml_version = "1.0")]`
- Register QRC resources at build time in Cargo builds (don't need to call initialization function from Rust `main` function)
- Support for container types: `QSet<T>`, `QHash<K, V>`, `QList<T>`, `QMap<K, V>`, `QVector<T>`
- Support for further types: `QByteArray`, `QCoreApplication`, `QGuiApplication`, `QMargins`, `QMarginsF`, `QModelIndex`, `QPersistentModelIndex`, `QQmlApplicationEngine`, `QQmlEngine`, `QStringList`, `QTimeZone`, `QVector2D`, `QVector3D`, `QVector4D`
- Support for nesting objects in properties, invokables, and signals with `*mut T`
- Allow for marking signals as existing in the base class
- Support for conversions to types in third-party crates: `bytes`, `chrono`, `http`, `rgb`, `time`, `url`
- Add several quality of life functions to builtin cxx-qt-lib types, including `Default` constructors, string formatting, `std::cmp` order and operators

### Changed

- `QVariant` now has a uses a `QVariantValue` trait for supported types, allowing custom types to be used with QVariant
- `QtGui` and `QtQml` types in cxx-qt-lib are now behind the features `qt_gui` and `qt_qml`

### Fixed

- Support for generating correct C++ code for `Pin<T>` Rust types
- Support namespace attribute on shared types, QObject struct, and extern blocks
- Asserts for 32bit platforms such as Wasm
- Errors from the generation not pointing to the span where they occurred

## [0.4.1](https://github.com/KDAB/cxx-qt/compare/v0.4.0...v0.4.1) - 2022-11-18

### Added

- Multiple QObjects can be defined in one bridge

### Fixed

- Fixed linking Qt with macOS frameworks. This allows using Qt from Homebrew.

## [0.4.0](https://github.com/KDAB/cxx-qt/compare/v0.3.0...v0.4.0) - 2022-10-28

### Added

- Addition of qt-build-utils crate
- Add generic system for performing type conversions in C++

### Changed

- Refactor of API so that CXX-Qt is a superset of CXX
- Rewrite of build system, removal of custom cmake file, corrosion is used for CMake and support for building with only Cargo
- Refactor of internal generation code so it's split into stages and removal of pattern matching for types, so arbritary CXX types can be supported
- Mark Qt relocatable types as trivial to CXX
- Use Rust closures to queue tasks onto the Qt thread

### Removed

- Support for nested objects

## [0.3.0](https://github.com/KDAB/cxx-qt/compare/v0.2.1...v0.3.0) - 2022-06-10

### Added

- Add a demo for showing complex threading and async
- Support for declaring and emitting signals

### Changed

- Use CXX itself for bridging Qt types

## [0.2.1](https://github.com/KDAB/cxx-qt/compare/v0.2.0...v0.2.1) - 2022-03-21

### Added

- Add vcpkg for Windows and macOS, then use this for CI
- Support mutable invokables

## [0.2.0](https://github.com/KDAB/cxx-qt/compare/v0.1.0...v0.2.0) - 2022-02-28

### Added

- Add more Qt types, QDate, QDateTime, Qpoint, QRect, QRectF, QSize, QTime, QUrl
- Support Qt 6 for Qt types
- Use a "CppObj" to represent the C++ context
- Add a book for documentation

### Changed

- Use postEvent for emitting events safely
## [0.1.0](https://github.com/KDAB/cxx-qt/releases/tag/v0.1.0) - 2021-12-03

### Added

- Initial release
- Support for bridging properties and invokables
- Support for QColor, QPointF, QSizeF, QString, QVariant
- Support for nested objects
