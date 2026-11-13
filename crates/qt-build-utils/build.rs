// SPDX-FileCopyrightText: 2026 The Qt Company
// SPDX-FileContributor: Be Wilson <be.wilson@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//! This no-op build.rs is a hack to get Cargo to set the OUT_DIR environment variable when building this crate,
//! which is compiled into the crate with the env! macro.
fn main() {}
