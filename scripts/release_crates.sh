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

# cxx-qt-cmake (no dependencies)
question_yesno "Have you bumped the project version (in CMakeLists.txt) and created a new tag in the cxx-qt-cmake repo"

# 5-cmake-integration.md has a GIT_TAG example pointing to the stable release
question_yesno "Have you updated the GIT_TAG for cxx-qt-cmake in the book"

# Cargo.toml version = "A.B.C"
question_yesno "Have you updated the Cargo versions for cxx-qt crates in Cargo.toml and the book examples"

# Unreleased to a tag with the date
question_yesno "Have you created a release in the CHANGELOG.md file"

# This also triggers a github pages build
question_yesno "Have you created a tag in the cxx-qt repo"

# No other dependencies
release_crate "qt-build-utils"

# No other dependencies
release_crate "cxx-qt-gen"

# Requires cxx-qt-gen
release_crate "cxx-qt-macro"

# Requires cxx-qt-gen and qt-build-utils
release_crate "cxx-qt-build"

# Requires cxx-qt-build, cxx-qt-macro, and qt-build-utils
release_crate "cxx-qt"

# Requires cxx-qt, cxx-qt-build
release_crate "cxx-qt-lib"

# Requires cxx-qt, cxx-qt-build, and cxx-qt-lib
release_crate "cxx-qt-lib-extras"
