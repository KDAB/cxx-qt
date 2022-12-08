// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

use indoc::formatdoc;

// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{
    generator::cpp::{fragment::CppFragment, qobject::GeneratedCppQObjectBlocks},
    parser::{cxxqtdata::ParsedCxxMappings, qobject::ParsedQObject},
};

use syn::{Error, Result, ReturnType};

use super::types::CppType;

pub fn generate(
    qobject: &ParsedQObject,
    cxx_mappings: &ParsedCxxMappings,
) -> Result<GeneratedCppQObjectBlocks> {
    let mut result = GeneratedCppQObjectBlocks::default();

    for method in &qobject.inherited_methods {
        let return_type = if let ReturnType::Type(_, ty) = &method.method.sig.output {
            CppType::from(ty, &None, cxx_mappings)?
                .as_cxx_ty()
                .to_owned()
        } else {
            "void".to_owned()
        };

        let missing_base_class_error = || {
            Error::new_spanned(&method.method, format!("C++ code generation failed!\nClass {} has  inherited methods, but no base class!", qobject.qobject_struct.ident))
        };
        let base_class = qobject
            .base_class
            .clone()
            .ok_or_else(missing_base_class_error)?;

        result.methods.push(CppFragment::Header(formatdoc! {
        r#"
              template <class... Args>
              {return_type} {wrapper_ident}(Args ...args) {mutability} {{
                  return {base_class}::{func_ident}(args...);
              }}"#,
        mutability = if method.mutable { "" } else { "const" },
        func_ident = method.ident.cpp,
        wrapper_ident = method.wrapper_ident,
        return_type = return_type,
        base_class = base_class
        }));
    }

    Ok(result)
}
