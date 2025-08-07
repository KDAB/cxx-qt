// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::path::path_compare_str;
use quote::ToTokens;
use syn::{
    token::Mut, Error, GenericArgument, Ident, Path, PathArguments, Result, Type, TypePath,
    TypeReference,
};

fn pin_path(ty: &Type) -> Option<Path> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if path_compare_str(path, &["Pin"]) {
            return Some(path.clone());
        }
    }

    None
}

fn err_pin_misuse(tokens: impl ToTokens) -> Error {
    Error::new_spanned(
        tokens,
        "Expected a non mutable T reference! Use either `&T` or `Pin<&mut T>`!",
    )
}

/// Checks if the given type is a `Pin<&Self>` or `Pin<&mut Self>`.
/// `Pin<Box<Self>>` is currently not supported.
pub fn is_pin_of_self(ty: &Type) -> bool {
    if let Some(path) = pin_path(ty) {
        if let PathArguments::AngleBracketed(angles) = &path.segments.first().unwrap().arguments {
            if let [GenericArgument::Type(Type::Reference(TypeReference {
                elem: type_elem, ..
            }))] = *angles.args.iter().collect::<Vec<_>>()
            {
                if let Type::Path(TypePath {
                    path: self_path, ..
                }) = &**type_elem
                {
                    if path_compare_str(self_path, &["Self"]) {
                        return true;
                    }
                } else {
                    // CODECOV_EXCLUDE_START
                    unreachable!(
                        "Path is already checked in pin_path so this should be unreachable!"
                    )
                    // CODECOV_EXCLUDE_STOP
                }
            }
        }
    }

    false
}

fn extract_qobject_ident_from_path(path: &Path) -> Result<Ident> {
    if path.segments.len() == 1 {
        Ok(path.segments[0].ident.clone())
    } else {
        Err(Error::new_spanned(
            path,
            "Expected a path with one segment!",
        ))
    }
}

fn extract_qobject_ident_from_ref(ty: &TypeReference) -> Result<(Ident, Option<Mut>)> {
    if let Type::Path(type_path) = &*ty.elem {
        Ok((
            extract_qobject_ident_from_path(&type_path.path)?,
            ty.mutability,
        ))
    } else {
        // CODECOV_EXCLUDE_START
        // If using parse_quote and not using a &T reference, parse quote will fail to parse as TypeReference
        unreachable!("Expected type to be a `&T` reference!")
        // CODECOV_EXCLUDE_STOP
    }
}

fn extract_qobject_from_mut_pin(ty: &TypePath) -> Result<(Ident, Mut)> {
    if path_compare_str(&ty.path, &["Pin"]) {
        return if let PathArguments::AngleBracketed(angles) =
            &ty.path.segments.first().unwrap().arguments
        {
            if let [GenericArgument::Type(Type::Reference(reference))] =
                *angles.args.iter().collect::<Vec<_>>()
            {
                let (ident, mutability) = extract_qobject_ident_from_ref(reference)?;
                if mutability.is_none() {
                    return Err(err_pin_misuse(reference));
                }
                Ok((ident, mutability.unwrap()))
            } else {
                Err(err_pin_misuse(ty))
            }
        } else {
            Err(Error::new_spanned(
                ty,
                "Pin must use angle brackets not parentheses for generic args! Use Pin<&mut T>",
            ))
        };
    }

    Err(err_pin_misuse(ty))
}

/// Extract the qobject ident from any of the following patterns:
/// - &T
/// - Pin<&mut T>
pub fn extract_qobject_ident(ty: &Type) -> Result<(Ident, Option<Mut>)> {
    match ty {
        Type::Reference(type_ref) => {
            let (ident, mutability) = extract_qobject_ident_from_ref(type_ref)?;
            if mutability.is_some() {
                return Err(err_pin_misuse(type_ref));
            }
            Ok((ident, mutability))
        }
        Type::Path(type_path) => {
            let (ident, mutability) = extract_qobject_from_mut_pin(type_path)?;
            Ok((ident, Some(mutability)))
        }
        _ => Err(err_pin_misuse(ty)),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::method::ParsedMethod;
    use crate::parser::CaseConversion;
    use crate::tests::assert_parse_errors;
    use syn::{parse_quote, ForeignItemFn, Type};

    #[test]
    fn test_is_pin_of_self() {
        assert!(super::is_pin_of_self(&parse_quote! { Pin<&Self> }));
        assert!(super::is_pin_of_self(&parse_quote! { Pin<&mut Self> }));

        // `Pin<Box<Self>>` is currently not supported because it can't be used with
        // Opaque C++ types. Use UniquePtr<Self> instead.
        assert!(!super::is_pin_of_self(&parse_quote! { Pin<Box<Self>> }));
        assert!(!super::is_pin_of_self(&parse_quote! { Pin<&Self, Foo> }));
        assert!(!super::is_pin_of_self(&parse_quote! { Pin }));
        assert!(!super::is_pin_of_self(&parse_quote! { Pin<Self> }));
        assert!(!super::is_pin_of_self(&parse_quote! { Pin<&Foo> }));
        assert!(!super::is_pin_of_self(&parse_quote! { Pin<&mut Foo> }));
        assert!(!super::is_pin_of_self(&parse_quote! { *mut T }));
    }

    fn assert_qobject_ident(ty: Type, expected_ident: &str, expected_mutability: bool) {
        let (ident, mutability) = super::extract_qobject_ident(&ty).unwrap();
        assert_eq!(ident.to_string(), expected_ident);
        assert_eq!(mutability.is_some(), expected_mutability);
    }

    #[test]
    fn test_extract_qobject_ident() {
        assert_qobject_ident(parse_quote! { &Foo }, "Foo", false);
        assert_qobject_ident(parse_quote! { Pin<&mut Foo> }, "Foo", true);

        assert_parse_errors! {
            |ty| super::extract_qobject_ident(&ty) =>

            { Foo }
            { &mut Foo }
            { Pin<&Foo> }
            { X::Foo }
            { Self }
            { Pin<T = A> }
        }
    }

    #[test]
    fn test_incorrect_pin_syntax() {
        let method: ForeignItemFn = parse_quote! {
            #[qinvokable]
            unsafe fn test(self: Pin);
        };
        let parsed = ParsedMethod::parse(method, CaseConversion::none(), false);
        assert!(parsed.is_err());
    }
}
