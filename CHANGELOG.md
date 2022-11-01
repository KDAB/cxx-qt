<!--
SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/KDAB/cxx-qt/compare/v0.4.0...HEAD)

### Added

- Multiple QObjects can be defined in one bridge

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
