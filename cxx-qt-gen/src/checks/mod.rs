// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod collisions;
mod reserved;

use crate::parser::Parser;
use collisions::check_for_colliding_names;
use reserved::check_for_reserved_names;
use syn::Result;

pub fn validate(parser: &Parser) -> Result<()> {
    check_for_reserved_names(parser).and(check_for_colliding_names(parser))
}

#[cfg(test)]
mod tests {
    // TODO
}
