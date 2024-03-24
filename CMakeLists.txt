cmake_minimum_required(VERSION 3.12)

project(cmake-cxx-example DESCRIPTION "Setup cxx with CMake")

add_link_options(-fuse-ld=lld)
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)

include(cmake/add_crate.cmake)

add_subdirectory(src)
