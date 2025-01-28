#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

# Assumes you have grcov and llvm in a system path
# Install:
# cargo install grcov
set -ex

# Ensure we are in the right directory
SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH/../"

# Ensure coverage folder is cleared
rm -f "$SCRIPTPATH"/coverage/*.profraw

# Check that the llvm path exists
#
# We can use rustup component add llvm-tools but this can be out of sync
# See versions from the table in this link
# https://github.com/taiki-e/cargo-llvm-cov?tab=readme-ov-file#get-coverage-of-cc-code-linked-to-rust-librarybinary
if [ ! -d /usr/lib/llvm-17/bin/ ]; then
  echo "LLVM 17 not found"
fi

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="$SCRIPTPATH/coverage/coverage_data-%p-%m.profraw"
cargo build --package cxx-qt-gen
cargo test --package cxx-qt-gen
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing --llvm --llvm-path=/usr/lib/llvm-17/bin/ -o ./target/debug/ --excl-start CODECOV_EXCLUDE_START --excl-stop CODECOV_EXCLUDE_STOP
echo "Coverage html report generated in $(realpath "$SCRIPTPATH"/../target/debug/html)"
