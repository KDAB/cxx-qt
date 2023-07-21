// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{GenericArgument, PathArguments, PathSegment, ReturnType, Type, TypeReference};

/// Return a qualified version of the type that can by used outside of a CXX bridge
///
/// Eg Pin -> core::pin::Pin or UniquePtr -> cxx::UniquePtr
pub(crate) fn syn_type_cxx_bridge_to_qualified(ty: &syn::Type) -> syn::Type {
    match ty {
        Type::Array(ty_array) => {
            let mut ty_array = ty_array.clone();
            *ty_array.elem = syn_type_cxx_bridge_to_qualified(&ty_array.elem);
            return Type::Array(ty_array);
        }
        Type::BareFn(ty_bare_fn) => {
            let mut ty_bare_fn = ty_bare_fn.clone();
            if let ReturnType::Type(_, ty) = &mut ty_bare_fn.output {
                **ty = syn_type_cxx_bridge_to_qualified(ty);
            }

            ty_bare_fn
                .inputs
                .iter_mut()
                .for_each(|arg| arg.ty = syn_type_cxx_bridge_to_qualified(&arg.ty));

            return Type::BareFn(ty_bare_fn);
        }
        Type::Path(ty_path) => {
            let mut ty_path = ty_path.clone();

            // Convert any generic arguments
            ty_path.path.segments.iter_mut().for_each(|segment| {
                if let PathArguments::AngleBracketed(angled) = &mut segment.arguments {
                    angled.args.iter_mut().for_each(|arg| {
                        if let GenericArgument::Type(ty) = arg {
                            *ty = syn_type_cxx_bridge_to_qualified(ty);
                        }
                    });
                }
            });

            // Convert the first element if it matches
            if let Some(segment) = ty_path.path.segments.first() {
                let qualified_prefix = match segment.ident.to_string().as_str() {
                    // Note we need to fully qualify any types that CXX supports that aren't
                    // - primitive types https://doc.rust-lang.org/stable/std/primitive/index.html
                    // - prelude types https://doc.rust-lang.org/stable/std/prelude/index.html
                    //
                    // We could also fully qualify types primitive and prelude types for full macro hygiene.
                    "CxxString" | "CxxVector" | "SharedPtr" | "UniquePtr" | "WeakPtr" => {
                        Some(vec!["cxx"])
                    }
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
            }

            return Type::Path(ty_path);
        }
        Type::Ptr(ty_ptr) => {
            let mut ty_ptr = ty_ptr.clone();
            *ty_ptr.elem = syn_type_cxx_bridge_to_qualified(&ty_ptr.elem);
            return Type::Ptr(ty_ptr);
        }
        Type::Reference(ty_ref) => {
            let mut ty_ref = ty_ref.clone();
            *ty_ref.elem = syn_type_cxx_bridge_to_qualified(&ty_ref.elem);
            return Type::Reference(ty_ref);
        }
        Type::Slice(ty_slice) => {
            let mut ty_slice = ty_slice.clone();
            *ty_slice.elem = syn_type_cxx_bridge_to_qualified(&ty_slice.elem);
            return Type::Slice(ty_slice);
        }
        Type::Tuple(ty_tuple) => {
            let mut ty_tuple = ty_tuple.clone();
            ty_tuple
                .elems
                .iter_mut()
                .for_each(|elem| *elem = syn_type_cxx_bridge_to_qualified(elem));
            return Type::Tuple(ty_tuple);
        }
        _others => {}
    }

    ty.clone()
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

    use syn::parse_quote;

    #[test]
    fn test_syn_type_cxx_bridge_to_qualified_cxx() {
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { CxxString }),
            parse_quote! { cxx::CxxString }
        );
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { CxxVector<T> }),
            parse_quote! { cxx::CxxVector<T> }
        );
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { SharedPtr<T> }),
            parse_quote! { cxx::SharedPtr<T> }
        );
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { UniquePtr<T> }),
            parse_quote! { cxx::UniquePtr<T> }
        );
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { WeakPtr<T> }),
            parse_quote! { cxx::WeakPtr<T> }
        );
    }

    #[test]
    fn test_syn_type_cxx_bridge_to_qualified_core() {
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { Pin<&mut T> }),
            parse_quote! { core::pin::Pin<&mut T> }
        );
    }

    #[test]
    fn test_syn_type_cxx_bridge_to_qualified_array() {
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { [UniquePtr<T>; 1] }),
            parse_quote! { [cxx::UniquePtr<T>; 1] }
        );
    }

    #[test]
    fn test_syn_type_cxx_bridge_to_qualified_fn() {
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { fn(UniquePtr<T>) -> SharedPtr<T> }),
            parse_quote! { fn(cxx::UniquePtr<T>) -> cxx::SharedPtr<T> }
        );
    }

    #[test]
    fn test_syn_type_cxx_bridge_to_qualified_nested() {
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { Pin<UniquePtr<T>> }),
            parse_quote! { core::pin::Pin<cxx::UniquePtr<T>> }
        );
    }

    #[test]
    fn test_syn_type_cxx_bridge_to_qualified_ptr() {
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { *mut UniquePtr<T> }),
            parse_quote! { *mut cxx::UniquePtr<T> }
        );
    }

    #[test]
    fn test_syn_type_cxx_bridge_to_qualified_reference() {
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { &UniquePtr<*mut T> }),
            parse_quote! { &cxx::UniquePtr<*mut T> }
        );
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { &mut UniquePtr<*mut T> }),
            parse_quote! { &mut cxx::UniquePtr<*mut T> }
        );
    }

    #[test]
    fn test_syn_type_cxx_bridge_to_qualified_slice() {
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { &[UniquePtr<T>] }),
            parse_quote! { &[cxx::UniquePtr<T>] }
        );
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { &mut [UniquePtr<T>] }),
            parse_quote! { &mut [cxx::UniquePtr<T>] }
        );
    }

    #[test]
    fn test_syn_type_cxx_bridge_to_qualified_tuple() {
        assert_eq!(
            syn_type_cxx_bridge_to_qualified(&parse_quote! { (UniquePtr<T>, ) }),
            parse_quote! { (cxx::UniquePtr<T>, ) }
        );
    }

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_path() {
        assert!(!syn_type_is_cxx_bridge_unsafe(&parse_quote! { i32 }));
    }

    #[test]
    fn test_syn_type_is_cxx_bridge_unsafe_path_template() {
        assert!(!syn_type_is_cxx_bridge_unsafe(
            &parse_quote! { Vector<i32> }
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
