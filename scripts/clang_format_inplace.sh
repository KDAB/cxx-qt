#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

CLANG_FORMAT_CMD=$1
DIR=$2
echo "Executing $CLANG_FORMAT_CMD on directory: $DIR"

function clang_format_files() {
    while IFS= read -r -d '' FILE
    do
        if git ls-files --error-unmatch "$FILE" &> /dev/null; then
            $CLANG_FORMAT_CMD -i -Werror "$FILE"
        fi
    done < <(find "$DIR" -type f -name "$1" -a -print0)
}

clang_format_files "*.cpp"
clang_format_files "*.h"
