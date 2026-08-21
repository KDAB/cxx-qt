// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QObject>

// Mirrors the Rust
// `#[repr(transparent)] struct QObjectMutPtr(*mut QObject)`
// so it can be used as a shared type across the bridge.
using QObjectMutPtr = ::QObject*;
