#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0
# Assumes you have grcov and llvm-tools
# Install:
# cargo install grcov
# rustup component add llvm-tools
set -ex
# Ensure we are in the right directory
SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH/../"

export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="$SCRIPTDIR/coverage/coverage_data-%p-%m.profraw"
cargo build --package cxx-qt-gen
cargo test --package cxx-qt-gen
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/
echo "Coverage html report generated in $SCRIPTDIR/target/debug/html"