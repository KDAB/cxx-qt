#include "test.h"
#include "lib.rs.h"

#include <iostream>

void hello_from_cpp() {
    std::cout << "Hello from C++!" << std::endl;
}

int main() {
   hello_from_rust();
   return 0;
}
