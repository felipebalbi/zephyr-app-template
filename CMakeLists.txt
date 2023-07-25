# Zephyr Template Application
#
# Copyright (c) 2023 Felipe Balbi
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.13.1)
find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

project(app LANGUAGES C VERSION 1.0.0)

configure_file(app.h.in ${CMAKE_BINARY_DIR}/app/include/app.h)
target_include_directories(app PRIVATE ${CMAKE_BINARY_DIR}/app/include src rust-lib)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.4.2
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH rust-lib/Cargo.toml)

add_subdirectory(src)
target_link_libraries(app PUBLIC rust-lib)
