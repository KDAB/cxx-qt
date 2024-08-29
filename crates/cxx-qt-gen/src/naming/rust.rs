// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::naming::TypeNames;
use syn::spanned::Spanned;
use syn::{
    Error, GenericArgument, PathArguments, PathSegment, Result, ReturnType, Type, TypePath,
    TypeReference,
};

macro_rules! convert_elem {
    ($id:ident, $variant:path, $type_names:ident) => {{
        let mut ty = $id.clone();
        *ty.elem = syn_type_cxx_bridge_to_qualified(&ty.elem, $type_names)?;
        Ok($variant(ty))
    }};
}

fn qualify_type_path(ty_path: &TypePath, type_names: &TypeNames) -> Result<Type> {
    let mut ty_path = ty_path.clone();

    // Convert any generic arguments
    for segment in ty_path.path.segments.iter_mut() {
        if let PathArguments::AngleBracketed(angled) = &mut segment.arguments {
            for arg in angled.args.iter_mut() {
                if let GenericArgument::Type(ty) = arg {
                    *ty = syn_type_cxx_bridge_to_qualified(ty, type_names)?;
                } else {
                    return Err(Error::new(arg.span(), "Unsupported GenericArgument type"));
                }
            }
        }
    }

    // Convert the first element if it matches
    if let Some(segment) = ty_path.path.segments.first() {
        let qualified_prefix = match segment.ident.to_string().as_str() {
            // Note we need to fully qualify any types that CXX supports that aren't
            // - primitive types https://doc.rust-lang.org/stable/std/primitive/index.html
            // - prelude types https://doc.rust-lang.org/stable/std/prelude/index.html
            //
            // We could also fully qualify types primitive and prelude types for full macro hygiene.
            "CxxString" | "CxxVector" | "SharedPtr" | "UniquePtr" | "WeakPtr" => Some(vec!["cxx"]),
            "Pin" => Some(vec!["core", "pin"]),
            _ => None,
        };

        // Inject the qualified prefix into the path if there is one
        if let Some(qualified_prefix) = qualified_prefix {
            for part in qualified_prefix.iter().rev() {
                let segment: PathSegment = syn::parse_str(part).unwrap();
                ty_path.path.segments.insert(0, segment);
            }
        }
    } else {
        // CODECOV_EXCLUDE_START
        unreachable!("Path cannot be empty!")
        // CODECOV_EXCLUDE_STOP
    }

    // If the path matches a known ident then used the qualified mapping
    if let Some(ident) = ty_path.path.get_ident() {
        ty_path.path = type_names.rust_qualified(ident)?;
    }

    Ok(Type::Path(ty_path))
}

/// Return a qualified version of the type that can by used outside of a CXX bridge
///
/// Eg Pin -> core::pin::Pin or UniquePtr -> cxx::UniquePtr
///
/// And also resolves any qualified mappings
///
/// Eg MyObject -> ffi::MyObject
pub(crate) fn syn_type_cxx_bridge_to_qualified(ty: &Type, type_names: &TypeNames) -> Result<Type> {
    match ty {
        Type::Array(ty_array) => {
            let mut ty_array = ty_array.clone();
            *ty_array.elem = syn_type_cxx_bridge_to_qualified(&ty_array.elem, type_names)?;
            Ok(Type::Array(ty_array))
        }
        Type::BareFn(ty_bare_fn) => {
            let mut ty_bare_fn = ty_bare_fn.clone();
            if let ReturnType::Type(_, ty) = &mut ty_bare_fn.output {
                **ty = syn_type_cxx_bridge_to_qualified(ty, type_names)?;
            }

            for arg in ty_bare_fn.inputs.iter_mut() {
                arg.ty = syn_type_cxx_bridge_to_qualified(&arg.ty, type_names)?;
            }

            Ok(Type::BareFn(ty_bare_fn))
        }
        Type::Path(ty_path) => qualify_type_path(ty_path, type_names),
        Type::Ptr(ty_ptr) => convert_elem!(ty_ptr, Type::Ptr, type_names),
        Type::Reference(ty_ref) => convert_elem!(ty_ref, Type::Reference, type_names),
        Type::Slice(ty_slice) => convert_elem!(ty_slice, Type::Slice, type_names),
        Type::Tuple(ty_tuple) => {
            let mut ty_tuple = ty_tuple.clone();
            for elem in ty_tuple.elems.iter_mut() {
                *elem = syn_type_cxx_bridge_to_qualified(elem, type_names)?;
            }
            Ok(Type::Tuple(ty_tuple))
        }
        _others => Err(syn::Error::new_spanned(ty, "Unsupported type")),
    }
}

/// Return if the type is unsafe for CXX bridges
pub(crate) fn syn_type_is_cxx_bridge_unsafe(ty: &syn::Type) -> bool {
    match ty {
        Type::Path(ty_path) => {
            ty_path
                .path
                .segments
                .iter()
                .any(|segment| match &segment.arguments {
                    PathArguments::AngleBracketed(angled) => {
                        angled.args.iter().any(|generic| match generic {
                            GenericArgument::Type(ty) => syn_type_is_cxx_bridge_unsafe(ty),
                            _others => false,
                        })
                    }
                    _others => false,
                })
        }
        Type::Ptr(..) => true,
        Type::Reference(TypeReference { elem, .. }) => syn_type_is_cxx_bridge_unsafe(elem),
        _others => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;
    use syn::parse_quote;

    macro_rules! test_bridge_types_qualified {
        [$($input_type:tt => $output_type:tt),*] => {
            let mut type_names = TypeNames::mock();
            type_names.mock_insert("T", None, None, None);

            $(
            let expected_type : syn::Type = parse_quote! $output_type ;
            crate::tests::assert_tokens_eq(
                &syn_type_cxx_bridge_to_qualified(&parse_quote! $input_type, &type_names).unwrap(),
                expected_type.to_token_stream()
            );
            )*
        }
    }

    #[test]
    fn test_syn_type_cxx_bridge_to_qualified() {
        test_bridge_types_qualified! [
        // Primitive types
        { i32 } => { i32 },
        { f32 } => { f32 },
        { bool } => { bool },
        { String } => { String },
        // CXX implementation of C++ types
        { CxxString } => { cxx::CxxString },
        { CxxVector<T> } => { cxx::CxxVector<T> },
        { SharedPtr<T> } => { cxx::SharedPtr<T> },
        { UniquePtr<T> } => { cxx::UniquePtr<T> },
        { WeakPtr<T> } => { cxx::WeakPtr<T> },
        // Pin
        { Pin<&mut T> } => { core::pin::Pin<&mut T> },
        // Different nestings
        { [UniquePtr<T>; 1] } => { [cxx::UniquePtr<T>; 1] },
        { fn(UniquePtr<T>) -> SharedPtr<T> } => { fn(cxx::UniquePtr<T>) -> cxx::SharedPtr<T> },
        { Pin<UniquePtr<T>> } =>  { core::pin::Pin<cxx::UniquePtr<T>> },
        { *mut UniquePtr<T> } => { *mut cxx::UniquePtr<T> },
        { &UniquePtr<*mut T> } => { &cxx::UniquePtr<*mut T> },
        { &[UniquePtr<T>] } => { &[cxx::UniquePtr<T>] },
        { &mut [UniquePtr<T>] } => { &mut[cxx::UniquePtr<T>] },
        { (UniquePtr<T>, ) } => { (cxx::UniquePtr<T>, ) },
        // Mapped type
        { MyObject } => { qobject::MyObject },
        { fn(i32) } => { fn(i32) }
        ];
    }

    #[test]
    fn test_syn_type_cxx_bridge_invalid() {
        let ty = parse_quote! { Vec<'a,T> };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, None, None);
        assert!(syn_type_cxx_bridge_to_qualified(&ty, &type_names).is_err());

        let ty = parse_quote! { (T) };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, None, None);
        assert!(syn_type_cxx_bridge_to_qualified(&ty, &type_names).is_err());
    }

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_path() {
        assert!(!syn_type_is_cxx_bridge_unsafe(&parse_quote! { i32 }));
    }

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_path_other() {
        assert!(!syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { impl MyTrait }
        ));
    }

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_path_template() {
        assert!(!syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { Vector<i32> }
        ));
        assert!(!syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { Vector<'a,i32> }
        ));
        assert!(syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { Vector<*mut T> }
        ));
    }

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_ptr() {
        assert!(syn_type_is_cxx_bridge_unsafe(&parse_quote! { *mut T }));
    }

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_reference() {
        assert!(!syn_type_is_cxx_bridge_unsafe(&parse_quote! { &i32 }));
        assert!(syn_type_is_cxx_bridge_unsafe(&parse_quote! { &*mut T }));
        assert!(!syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { &Vector<i32> }
        ));
        assert!(syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { &Vector<*mut T> }
        ));
    }
}
