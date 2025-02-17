#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -ex

SOURCE_FOLDER=$1
BUILD_FOLDER=$2

# Ensure we are in the right directory
cd "$SOURCE_FOLDER"

# Ensure that we do see a "Compiling" in the output
# as if we do it means we have a cargo::rerun-if-changed incorrectly
function check_build_contains_compiling() {
    BUILD=$(cargo build --release --target-dir="$BUILD_FOLDER" -p qml-minimal-no-cmake 2>&1)

    if ! echo "$BUILD" | grep -q Compiling; then
        echo "cargo build is missing text 'Compiling', likely an incorrect cargo::rerun-if-changed in a build script."
        exit 1
    fi
}

# Ensure that we don't see any "Compiling" in the output
# as if we do it means we have a cargo::rerun-if-changed incorrectly
function check_build_no_compiling() {
    BUILD=$(cargo build --release --target-dir="$BUILD_FOLDER" -p qml-minimal-no-cmake 2>&1)

    if echo "$BUILD" | grep -q Compiling; then
        echo "cargo build contained text 'Compiling', likely an incorrect cargo::rerun-if-changed in a build script."
        exit 1
    fi
}

# Build once
cargo build --release --target-dir="$BUILD_FOLDER" -p qml-minimal-no-cmake

# Build a second time
check_build_no_compiling

# Modify a qml file
touch "$SOURCE_FOLDER/examples/cargo_without_cmake/qml/main.qml"

# Build a third and fourth time
check_build_contains_compiling
check_build_no_compiling

# Modify a Rust file
touch "$SOURCE_FOLDER/examples/cargo_without_cmake/src/cxxqt_object.rs"

# Build a fifth and sixth time
check_build_contains_compiling
check_build_no_compiling

exit 0
