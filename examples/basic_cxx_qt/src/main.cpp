#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "cxx-qt-gen/include/my_object.h"

TEST_CASE("CXX-Qt allows basic interaction between C++ (with Qt) and Rust") {
    // TODO: once invokables can accept parameters and/or return values,
    // this test should be updated to do something useful

    MyObject obj;
    obj.say_hi();

    CHECK(true == true);
}
