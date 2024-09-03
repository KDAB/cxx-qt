// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
// SPDX-FileContributor: Matt Aber <matt.aber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#define assert_alignment_and_size(TYPE, MEMBERS)                               \
  namespace {                                                                  \
  struct s##TYPE MEMBERS;                                                      \
  }                                                                            \
  static_assert(alignof(s##TYPE) == alignof(TYPE));                            \
  static_assert(sizeof(s##TYPE) == sizeof(TYPE))
