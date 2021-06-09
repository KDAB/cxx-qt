#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "main.h"
#include "cxx-qt-gen/include/lib.rs.h"

int hidden_num = 100;

int get_cpp_number() {
    return hidden_num;
}

TEST_CASE("Clean cxx allows basic interaction between C++ and Rust") {
    CHECK(get_numbers_sum() == 102);
    hidden_num = 200;
    CHECK(get_numbers_sum() == 202);
}
