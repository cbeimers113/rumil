cmake_minimum_required(VERSION 3.31)
project(rumil)

set(CMAKE_CXX_STANDARD 17)

# --- Detect platform-specific library extension ---
if(WIN32)
    set(RUST_LIB "${CMAKE_SOURCE_DIR}/src/parser/target/release/rumil_parser.lib")
elseif(APPLE)
    set(RUST_LIB "${CMAKE_SOURCE_DIR}/src/parser/target/release/librumil_parser.a")
else()
    set(RUST_LIB "${CMAKE_SOURCE_DIR}/src/parser/target/release/librumil_parser.a")
endif()

# --- Build parser ---
add_custom_command(
    OUTPUT ${RUST_LIB}
    COMMAND cargo build --release
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/src/parser
    COMMENT "Building parser with Cargo"
)

add_custom_target(rumil_parser ALL
    DEPENDS ${RUST_LIB}
)

# --- Build binary ---
add_executable(rumil src/rumil.cpp)

# Make sure binary waits for parser
add_dependencies(rumil rumil_parser)

# Link against Rust static lib
target_link_libraries(rumil PRIVATE ${RUST_LIB})

# Add include dir for rumil.h
target_include_directories(rumil PRIVATE ${CMAKE_SOURCE_DIR}/src)

# Place final binary in project root
set_target_properties(rumil PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}
)