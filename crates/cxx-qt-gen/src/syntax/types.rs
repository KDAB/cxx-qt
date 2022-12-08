// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::path::*;
use syn::{GenericArgument, PathArguments, Type, TypePath, TypeReference};

pub fn is_pin_of_self(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if path_compare_str(&type_path.path, &["Pin"]) {
            if let PathArguments::AngleBracketed(angles) =
                &type_path.path.segments.first().unwrap().arguments
            {
                if let [GenericArgument::Type(Type::Reference(TypeReference {
                    elem: type_elem,
                    ..
                }))] = *angles.args.iter().collect::<Vec<_>>()
                {
                    if let Type::Path(TypePath {
                        path: self_path, ..
                    }) = &**type_elem
                    {
                        if path_compare_str(self_path, &["Self"]) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}
