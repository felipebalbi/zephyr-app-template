# Zephyr Template Application
#
# Copyright (c) 2023 Felipe Balbi
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.13.1)
find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

project(app LANGUAGES C VERSION 1.0.0)

configure_file(app.h.in ${CMAKE_BINARY_DIR}/app/include/app.h)
target_include_directories(app PRIVATE ${CMAKE_BINARY_DIR}/app/include src)

add_subdirectory(src)
