// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod main_object;

// Ensure the symbols from the rlib dependencies end up
// in the staticlib (if you use Rust symbols from these
// crates in this crate, you can skip these `pub use` statements).
pub use sub1;
pub use sub2;
