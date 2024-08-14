# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# No longer add cargo-without-cmake here.
# This causes duplicate builds, as this crate is already entirely built
# When using `cargo test`
add_subdirectory(qml_features)
add_subdirectory(qml_minimal)

# TODO: get demo_threading working for wasm builds
if(NOT BUILD_WASM)
    add_subdirectory(demo_threading)
endif()
