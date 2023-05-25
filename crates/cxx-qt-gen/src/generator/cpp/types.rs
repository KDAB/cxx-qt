// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{generator::utils::cpp::syn_type_to_cpp_type, parser::cxxqtdata::ParsedCxxMappings};
use syn::{Result, Type};

/// A helper for describing a C++ type
pub struct CppType {
    ty: String,
}

impl CppType {
    /// Retrieve the Rust type in C++ form
    pub fn as_cxx_ty(&self) -> &str {
        &self.ty
    }

    /// Construct a [CppType] from a given [syn::Type] and [ParsedCxxMappings].
    pub fn from(ty: &Type, cxx_mapping: &ParsedCxxMappings) -> Result<CppType> {
        Ok(CppType {
            ty: syn_type_to_cpp_type(ty, cxx_mapping)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_cxx_type() {
        let ty = parse_quote! { UniquePtr<QColor> };
        let cxx_ty = CppType::from(&ty, &ParsedCxxMappings::default()).unwrap();
        assert_eq!(cxx_ty.as_cxx_ty(), "::std::unique_ptr<QColor>");
    }

    #[test]
    fn test_cxx_type_mapped() {
        let ty = parse_quote! { A };
        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_names
            .insert("A".to_owned(), "A1".to_owned());
        let cxx_ty = CppType::from(&ty, &cxx_mappings).unwrap();
        assert_eq!(cxx_ty.as_cxx_ty(), "A1");
    }
}
