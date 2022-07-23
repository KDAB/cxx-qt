// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "ffi.cxx.h"
#include "main.h"

int hidden_num = 100;

int
get_cpp_number()
{
  return hidden_num;
}

TEST_CASE("Clean cxx allows basic interaction between C++ and Rust")
{
  CHECK(get_numbers_sum() == 102);
  hidden_num = 200;
  CHECK(get_numbers_sum() == 202);
}
