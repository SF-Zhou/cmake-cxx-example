#include "demo/src/cxx.rs.h"
#include "hello/src/cxx.rs.h"
#include <iostream>
#include <string_view>

int main() {
  auto result = concat_two_strings("hello", " world!");
  std::cout << std::string_view(result.data(), result.size()) << std::endl;

  auto a = create_a_rust_object();
  a->show();
  a->inc();
  a->show();

  std::cout << hello::add(1, 2) << std::endl;
}
