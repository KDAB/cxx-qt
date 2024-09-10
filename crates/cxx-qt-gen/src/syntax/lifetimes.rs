// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{Error, GenericArgument, Lifetime, PathArguments, PathSegment, Result, Type};

pub(crate) fn err_unsupported_type<T: quote::ToTokens>(ty: &T) -> Error {
    Error::new_spanned(ty, "Type not supported by CXX-Qt!")
}

fn from_generic_argument(argument: &GenericArgument) -> Result<Vec<Lifetime>> {
    match argument {
        GenericArgument::Lifetime(lifetime) => Ok(vec![lifetime.clone()]),
        GenericArgument::Type(ty) => from_type(ty),
        _ => Err(err_unsupported_type(argument)),
    }
}

fn from_pathsegment(segment: &PathSegment) -> Result<Vec<Lifetime>> {
    match segment.arguments {
        PathArguments::None => Ok(vec![]),
        PathArguments::AngleBracketed(ref angles) => Ok(angles
            .args
            .iter()
            .map(from_generic_argument)
            .collect::<Result<Vec<Vec<Lifetime>>>>()?
            .into_iter()
            .flatten()
            .collect()),
        PathArguments::Parenthesized(_) => {
            // CODECOV_EXCLUDE_START
            unreachable!(
                "Parenthesized path args should not be in function args, only trait bounds!"
            )
            // CODECOV_EXCLUDE_STOP
        }
    }
}

pub fn from_type(ty: &Type) -> Result<Vec<Lifetime>> {
    match ty {
        Type::Array(array) => from_type(&array.elem),
        Type::Paren(paren) => from_type(&paren.elem),
        Type::Ptr(pointer) => from_type(&pointer.elem),
        Type::Slice(slice) => from_type(&slice.elem),
        Type::Path(path) => {
            if path.qself.is_some() {
                Err(err_unsupported_type(ty))
            } else {
                Ok(path
                    .path
                    .segments
                    .iter()
                    .map(from_pathsegment)
                    .collect::<Result<Vec<Vec<Lifetime>>>>()?
                    .into_iter()
                    .flatten()
                    .collect())
            }
        }
        Type::Reference(reference) => Ok(from_type(&reference.elem)?
            .into_iter()
            .chain(reference.lifetime.clone())
            .collect()),
        Type::Tuple(tuple) => Ok(tuple
            .elems
            .iter()
            .map(from_type)
            .collect::<Result<Vec<Vec<Lifetime>>>>()?
            .into_iter()
            .flatten()
            .collect()),
        _ => Err(err_unsupported_type(ty)),
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    macro_rules! assert_no_lifetimes {
        ($($tt:tt)*) => {
            assert!(super::from_type(&parse_quote! { $($tt)* })
                .unwrap()
                .is_empty());
        };
    }

    #[test]
    fn extract_no_lifetimes() {
        assert_no_lifetimes! { () }
        assert_no_lifetimes! { T }
        assert_no_lifetimes! { T<A> }
        assert_no_lifetimes! { *mut X }
        assert_no_lifetimes! { *const X }
        assert_no_lifetimes! { Pin<*mut T> }
        assert_no_lifetimes! { &T }
        assert_no_lifetimes! { &mut T }
        assert_no_lifetimes! { [T] }
        assert_no_lifetimes! { [T;4] }
        assert_no_lifetimes! { (X, Y) }
        assert_no_lifetimes! { (Y) }
        assert_no_lifetimes! { std::collections::Vec<i32> }
    }

    macro_rules! assert_lifetimes {
        ([$($lifetime:lifetime),*] $($tt:tt)*) => {
            assert_eq!(
                super::from_type(&parse_quote! { $($tt)* }).unwrap(),
                vec![$(parse_quote! { $lifetime }),*]
            );
        }
    }

    #[test]
    fn assert_lifetimes() {
        assert_lifetimes! { ['a] &'a T }
        assert_lifetimes! { ['a] [&'a T] }
        assert_lifetimes! { ['a] [&'a T;5] }

        assert_lifetimes! { ['a, 'a] (&'a A, &'a mut B) }
        assert_lifetimes! { ['a, 'a] &'a A<'a> }

        assert_lifetimes! { ['a, 'b] &'b &'a mut T }
        assert_lifetimes! { ['a, 'b] Pin<&'a X, &'b mut Y> }
        assert_lifetimes! { ['a, 'b] (&'a A, &'b mut B) }

        assert_lifetimes! { ['lifetime] A<'lifetime> }
    }

    macro_rules! assert_unsupported_type {
        ($( $tt:tt )*) => {
            assert!(super::from_type(&parse_quote! { $($tt)* }).is_err());
        };
    }

    #[test]
    fn extract_lifetimes_unsupported_types() {
        assert_unsupported_type! { dyn Foo }
        assert_unsupported_type! { &dyn Foo }
        assert_unsupported_type! { fn(A) }
        assert_unsupported_type! { fn(i64) -> i32 }
        assert_unsupported_type! { Vec<T = A> }
        assert_unsupported_type! { fn(A, B) -> C }
        assert_unsupported_type! { <T as Send>::Associated }
        assert_unsupported_type! { impl Fn(A,B) }
    }
}
