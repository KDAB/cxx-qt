#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0
set -e

DIR=$1
echo "Executing clang-format on directory: $DIR"

function clang_format_files() {
    while IFS= read -r -d '' file
    do
        if ! git check-ignore -q "$file"; then
            clang-format --dry-run -Werror "$file"
        fi
    done < <(find "$DIR" -type f -name "$1" -a -not -path "$DIR/.git/*" -not -path "$DIR/vcpkg/*" -not -path "$DIR/*/vcpkg_installed/*" -print0)
}

clang_format_files "*.cpp"
clang_format_files "*.h"
