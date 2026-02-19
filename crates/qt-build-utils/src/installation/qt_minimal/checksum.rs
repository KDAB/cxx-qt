// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use sha2::{Digest, Sha256};
use std::io::{BufReader, Read};

const BUFFER_SIZE: usize = 1024;

/// Hash a file at specified path, using sha256, and return it as a vec of bytes
pub(crate) fn hash_file(path: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let file = std::fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = [0; BUFFER_SIZE];

    while let Ok(size) = reader.read(&mut buffer) {
        if size == 0 {
            break;
        }
        hasher.update(&buffer[..size]);
    }

    hasher.finalize().to_vec()
}
