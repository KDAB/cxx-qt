#pragma once

#define assert_alignment_and_size(TYPE, ALIGNMENT, SIZE)                       \
  static_assert(alignof(TYPE) <= (ALIGNMENT),                                  \
                "unexpectedly large " #TYPE " alignment!");                    \
  static_assert(sizeof(TYPE) == (SIZE), "unexpected " #TYPE " size!");
