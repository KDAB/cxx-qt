#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# Ensure we stop if there are any failures
set -e

# Ensure we are in the right directory
SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

# Helper to wait for a yes no response
function question_yesno() {
    while true; do
        read -r -p "$1 ? (y/n) " yn

        case $yn in
            [yY] )
                break;;
            [nN] )
                exit 1;;
            * ) echo invalid response;;
        esac
    done
}

function release_crate() {
    cd "$SCRIPTPATH/../crates/$1"

    # List the files in the package
    cargo package --list
    question_yesno "[crate:$1] Do the files look correct"

    # Attempt a dry run
    cargo publish --dry-run
    question_yesno "[crate:$1] Were there no errors"

    # Now actually publish
    question_yesno "[crate:$1] Upload"
    cargo publish
}

# Remind about cargo login
question_yesno "Have you run cargo login before and setup credentials"

# No other dependencies
release_crate "qt-build-utils"

# No other dependencies
release_crate "cxx-qt-gen"

# Requires cxx-qt-gen
release_crate "cxx-qt-macro"

# Requires cxx-qt-macro and qt-build-utils
release_crate "cxx-qt"

# Requires cxx-qt, cxx-qt-gen, and qt-build-utils
release_crate "cxx-qt-build"

# Requires cxx-qt, cxx-qt-build
release_crate "cxx-qt-lib"

# Requires cxx-qt-build
release_crate "cxx-qt-lib-extras-headers"

# Requires cxx-qt, cxx-qt-build, cxx-qt-lib, cxx-qt-lib-extras-headers
release_crate "cxx-qt-lib-extras"
