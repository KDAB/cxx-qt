// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>

// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::path::path_compare_str;
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

/// Determine if a given [syn::Type] has a mutable T in Pin<T>
pub fn is_pin_mut(ty: &Type) -> bool {
    if let Some(path) = pin_path(ty) {
        // Read the contents of the T
        if let Some(last) = path.segments.last() {
            if let PathArguments::AngleBracketed(args) = &last.arguments {
                if let Some(GenericArgument::Type(Type::Reference(TypeReference {
                    mutability: Some(_),
                    ..
                }))) = args.args.first()
                {
                    return true;
                }
            }
        }
    }

    false
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
                }
            }
        }
    }

    false
}

fn extract_qobject_ident_from_path(path: &Path) -> Result<Ident> {
    if path.segments.len() == 2 && path.segments.first().unwrap().ident == "qobject" {
        Ok(path.segments.last().unwrap().ident.clone())
    } else {
        Err(Error::new_spanned(path, "Expected a qobject::T type!"))
    }
}

fn extract_qobject_ident_from_ref(ty: &TypeReference) -> Result<(Ident, Option<Mut>)> {
    if let Type::Path(type_path) = &*ty.elem {
        Ok((
            extract_qobject_ident_from_path(&type_path.path)?,
            ty.mutability,
        ))
    } else {
        Err(Error::new_spanned(
            ty,
            "Expected type to be a `&qobject::T` reference!",
        ))
    }
}

fn extract_qobject_from_mut_pin(ty: &TypePath) -> Result<(Ident, Mut)> {
    if path_compare_str(&ty.path, &["Pin"]) {
        if let PathArguments::AngleBracketed(angles) = &ty.path.segments.first().unwrap().arguments
        {
            if let [GenericArgument::Type(Type::Reference(reference))] =
                *angles.args.iter().collect::<Vec<_>>()
            {
                let (ident, mutability) = extract_qobject_ident_from_ref(reference)?;
                if mutability.is_none() {
                    return Err(Error::new_spanned(
                        reference,
                        "Expected a mutable reference when using Pin<>!",
                    ));
                }
                return Ok((ident, mutability.unwrap()));
            }
        }
    }
    if ty
        .path
        .segments
        .first()
        .map(|segment| segment.ident == "qobject")
        == Some(true)
    {
        Err(Error::new_spanned(ty, "Cannot take qobject::T by value, use either `self: &qobject::T`, or `Pin<&mut qobject::T>`"))
    } else {
        Err(Error::new_spanned(
            ty,
            "Expected a qobject::T refernce! Use either `&qobject::T` or `Pin<&mut qobject::T>`",
        ))
    }
}

/// Extract the qobject ident from any of the following patterns:
/// - &qobject::T
/// - Pin<&mut qobject::T>
pub fn extract_qobject_ident(ty: &Type) -> Result<(Ident, Option<Mut>)> {
    match ty {
        Type::Reference(type_ref) => {
            let (ident, mutability) = extract_qobject_ident_from_ref(type_ref)?;
            if mutability.is_some() {
                return Err(Error::new_spanned(
                    type_ref,
                    "Cannot take qobject::T by mutable reference, use either `self: &qobject::T`, or `Pin<&mut qobject::T>`",
                ));
            }
            Ok((ident, mutability))
        }
        Type::Path(type_path) => {
            let (ident, mutability) = extract_qobject_from_mut_pin(type_path)?;
            Ok((ident, Some(mutability)))
        }
        _ => Err(Error::new_spanned(
            ty,
            "Expected type to be a &qobject::T or Pin<&mut qobject::T> reference!",
        )),
    }
}

#[cfg(test)]
mod tests {

    use crate::tests::tokens_to_syn;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{parse_quote, Type};

    fn assert_pin_of_self(tokens: TokenStream) {
        let pin: Type = tokens_to_syn(tokens);
        assert!(super::is_pin_of_self(&pin));
    }

    fn assert_not_pin_of_self(tokens: TokenStream) {
        let pin: Type = tokens_to_syn(tokens);
        assert!(!super::is_pin_of_self(&pin));
    }

    #[test]
    fn test_is_pin_of_self() {
        assert_pin_of_self(quote! { Pin<&Self> });
        assert_pin_of_self(quote! { Pin<&mut Self> });

        // `Pin<Box<Self>>` is currently not supported because it can't be used with
        // Opaque C++ types. Use UniquePtr<Self> instead.
        assert_not_pin_of_self(quote! { Pin<Box<Self>> });
        assert_not_pin_of_self(quote! { Pin<&Self, Foo> });
        assert_not_pin_of_self(quote! { Pin });
        assert_not_pin_of_self(quote! { Pin<Self> });
        assert_not_pin_of_self(quote! { Pin<&Foo> });
        assert_not_pin_of_self(quote! { Pin<&mut Foo> });
    }

    #[test]
    fn test_is_pin_mut() {
        assert!(!super::is_pin_mut(&parse_quote! { Pin<&Self> }));
        assert!(super::is_pin_mut(&parse_quote! { Pin<&mut Self> }));
        assert!(!super::is_pin_mut(&parse_quote! { Pin<Box<Self>> }));
        assert!(!super::is_pin_mut(&parse_quote! { Pin<&Self, Foo> }));
        assert!(!super::is_pin_mut(&parse_quote! { Pin }));
        assert!(!super::is_pin_mut(&parse_quote! { Pin<Self> }));
        assert!(!super::is_pin_mut(&parse_quote! { Pin<&Foo> }));
        assert!(super::is_pin_mut(&parse_quote! { Pin<&mut Foo> }));
    }

    fn assert_qobject_ident(tokens: TokenStream, expected_ident: &str, expected_mutability: bool) {
        let ty: Type = tokens_to_syn(tokens);
        let (ident, mutability) = super::extract_qobject_ident(&ty).unwrap();
        assert_eq!(ident.to_string(), expected_ident);
        assert_eq!(mutability.is_some(), expected_mutability);
    }

    fn assert_no_qobject_ident(tokens: TokenStream) {
        let ty: Type = tokens_to_syn(tokens);
        assert!(super::extract_qobject_ident(&ty).is_err());
    }

    #[test]
    fn test_extract_qobject_ident() {
        assert_qobject_ident(quote! { &qobject::Foo }, "Foo", false);
        assert_qobject_ident(quote! { Pin<&mut qobject::Foo> }, "Foo", true);

        assert_no_qobject_ident(quote! { qobject::Foo });
        assert_no_qobject_ident(quote! { &mut qobject::Foo });
        assert_no_qobject_ident(quote! { Pin<&qobject::Foo> });
        assert_no_qobject_ident(quote! { Foo });
        assert_no_qobject_ident(quote! { qobject::X::Foo });
        assert_no_qobject_ident(quote! { Self });
    }
}
