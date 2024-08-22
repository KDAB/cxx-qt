// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <array>
#include <cassert>
#include <cstdint>

namespace rust::cxxqtlib1::assertion_utils {
template<typename iter>
constexpr static ::std::size_t
calc_align_size(const iter rbegin,
                const iter rend,
                const ::std::size_t actual_alignment)
{
  ::std::size_t rows = 0;
  ::std::size_t accum = 0;

  for (auto it = rbegin; it != rend; ++it) {
    assert(*it <= actual_alignment);

    if (it + 1 != rend) {
      if (accum + *it == actual_alignment) {
        accum += *it;
      } else {
        if (accum + *it + *(it + 1) <= actual_alignment)
          accum += *it;
        else
          accum = actual_alignment;
      }
      if (accum == actual_alignment) {
        accum = 0;
        ++rows;
      }
    } else {
      ++rows;
    }
  }

  return rows * actual_alignment;
}
} // namespace rust::cxxqtlib1::assertion_utils

#define assert_alignment_and_size(TYPE, EXP_ALIGN, ARR, ARR_SZ)                \
  static_assert(EXP_ALIGN == alignof(TYPE));                                   \
  static_assert(                                                               \
    rust::cxxqtlib1::assertion_utils::calc_align_size<                         \
      ::std::array<::std::size_t, ARR_SZ>::const_reverse_iterator>(            \
      ::std::rbegin(ARR), ::std::rend(ARR), alignof(TYPE)) == sizeof(TYPE));
