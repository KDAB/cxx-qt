// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "basic_cxx_only/cxx_test.h"

namespace {
int hidden_num = 100;
}

int
get_cpp_number()
{
  return hidden_num;
}

void
set_cpp_number(int num)
{
  hidden_num = num;
}
