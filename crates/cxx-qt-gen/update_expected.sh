#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0


set -ex

SCRIPT=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")

cd "${SCRIPT_DIR}"
CXX_QT_UPDATE_EXPECTED="${SCRIPT_DIR}" cargo test tests::generates

