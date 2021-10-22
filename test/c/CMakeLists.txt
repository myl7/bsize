cmake_minimum_required(VERSION 3.0)
project(bsize_test_c C)
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wl,--no-as-needed")
link_libraries(-lpthread -ldl -lm)

add_executable(test test.c)
target_include_directories(test PRIVATE ${PROJECT_SOURCE_DIR}/../../include)
target_link_libraries(test PRIVATE -L${PROJECT_SOURCE_DIR}/../../lib -lbsize)
