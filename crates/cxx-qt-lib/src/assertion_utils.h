// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#ifdef __EMSCRIPTEN__
// alignof completely explodes on emscripten builds
// so until fixed we leave this defined to nothing
#define assert_alignment_and_size(TYPE, ALIGNMENT, SIZE)
#else
#define assert_alignment_and_size(TYPE, ALIGNMENT, SIZE)                       \
  static_assert(alignof(TYPE) <= (ALIGNMENT),                                  \
                "unexpectedly large " #TYPE " alignment!");                    \
  static_assert(sizeof(TYPE) == (SIZE), "unexpected " #TYPE " size!");
#endif
