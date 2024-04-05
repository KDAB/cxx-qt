// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <cstddef>
#include <cstdint>

// Ensure that std::size_t is the size that Rust is expecting.
// This is the same as CXX does internally
// https://github.com/dtolnay/cxx/blob/5a7f93bd1361857c1bcd41f203d55f13ad0ccdf9/src/cxx.cc#L391
static_assert(sizeof(::std::size_t) == sizeof(::std::uintptr_t),
              "unsupported size_t size");
static_assert(alignof(::std::size_t) == alignof(::std::uintptr_t),
              "unsupported size_t alignment");
