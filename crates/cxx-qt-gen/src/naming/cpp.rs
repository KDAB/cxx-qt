// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::naming::TypeNames;
use syn::{
    spanned::Spanned, Error, Expr, GenericArgument, Lit, PathArguments, PathSegment, Result,
    ReturnType, Type, TypeArray, TypeBareFn, TypePtr, TypeReference, TypeSlice,
};

/// For a given Rust return type determine if the C++ header should have noexcept
pub(crate) fn syn_return_type_to_cpp_except(return_ty: &ReturnType) -> &str {
    if let ReturnType::Type(_, ty) = return_ty {
        // If we are a Result<T> then we can have an exception
        if let Type::Path(ty_path) = &**ty {
            if let Some(segment) = ty_path.path.segments.first() {
                if segment.ident == "Result" {
                    return "";
                }
            } else {
                // CODECOV_EXCLUDE_START
                unreachable!("Path cannot be empty!")
                // CODECOV_EXCLUDE_STOP
            }
        }
    }

    "noexcept"
}

/// For a given Rust return type attempt to generate a C++ string
///
/// Note that return types are allowed to have a Result<T>
pub(crate) fn syn_type_to_cpp_return_type(
    return_ty: &ReturnType,
    type_names: &TypeNames,
) -> Result<Option<String>> {
    if let ReturnType::Type(_, ty) = return_ty {
        // If we are a Result<T> then we just become T for C++
        if let Type::Path(ty_path) = &**ty {
            if let Some(segment) = ty_path.path.segments.first() {
                if segment.ident == "Result" {
                    let mut args = path_argument_to_string(&segment.arguments, type_names)?
                        .unwrap_or_default();
                    if args.len() != 1 {
                        return Err(Error::new(
                            return_ty.span(),
                            "Result must have one argument",
                        ));
                    }

                    if let Some(arg) = args.pop() {
                        // Map void to None
                        if arg == "void" {
                            return Ok(None);
                        }

                        return Ok(Some(arg));
                    } else {
                        // CODECOV_EXCLUDE_START
                        unreachable!("Args should be of length 1");
                        // CODECOV_EXCLUDE_STOP
                    }
                }
            } else {
                // CODECOV_EXCLUDE_START
                unreachable!("Path cannot be empty!")
                // CODECOV_EXCLUDE_STOP
            }
        }

        syn_type_to_cpp_type(ty, type_names).map(|v| if v == "void" { None } else { Some(v) })
    } else {
        Ok(None)
    }
}

/// For a given Rust type attempt to generate a C++ string
///
/// This is similar to the parsing in CXX
/// https://github.com/dtolnay/cxx/blob/a6e1cd1e8d9d6df20e88e7443963dc4c5c8c4875/syntax/parse.rs#L1134
pub(crate) fn syn_type_to_cpp_type(ty: &Type, type_names: &TypeNames) -> Result<String> {
    match ty {
        Type::Array(TypeArray { elem, len, .. }) => {
            let len = if let Expr::Lit(len) = &len {
                if let Lit::Int(len) = &len.lit {
                    len.base10_parse::<usize>()?
                } else {
                    return Err(Error::new(ty.span(), "Array length must be a integer"));
                }
            } else {
                return Err(Error::new(ty.span(), "Array length must be a integer"));
            };

            if len == 0 {
                return Err(Error::new(ty.span(), "Array length must be > 0"));
            }

            Ok(format!(
                "::std::array<{ty}, {len}>",
                ty = syn_type_to_cpp_type(elem, type_names)?,
                len = len
            ))
        }
        Type::BareFn(TypeBareFn { inputs, output, .. }) => {
            let ret = if let ReturnType::Type(_, ty) = output {
                syn_type_to_cpp_type(ty, type_names)?
            } else {
                "void".to_owned()
            };

            let args = inputs
                .iter()
                .map(|arg| syn_type_to_cpp_type(&arg.ty, type_names))
                .collect::<Result<Vec<String>>>()?;

            Ok(format!(
                "::rust::Fn<{ret}, ({args})>",
                ret = ret,
                args = args.join(", ")
            ))
        }
        Type::Path(ty_path) => {
            let ty_strings = ty_path
                .path
                .segments
                .iter()
                .map(|generic| path_segment_to_string(generic, type_names))
                .collect::<Result<Vec<String>>>()?;
            if ty_strings.len() == 1 {
                let first = ty_strings.first().unwrap();
                Ok(first.to_owned())
            } else {
                Err(Error::new(
                    ty.span(),
                    "Paths with multiple segments are not supported in types",
                ))
            }
        }
        Type::Ptr(TypePtr {
            const_token, elem, ..
        }) => Ok(format!(
            "{is_const}{ty}*",
            is_const = if const_token.is_some() { "const " } else { "" },
            ty = syn_type_to_cpp_type(elem, type_names)?
        )),
        Type::Reference(TypeReference {
            mutability, elem, ..
        }) => {
            let is_const = if mutability.is_some() { "" } else { " const" };
            match &**elem {
                // Slice is a special type only available as a reference
                // We need to map &[T] to rust::Slice<const T> and &mut [T] without the const
                Type::Slice(TypeSlice { elem, .. }) => Ok(format!(
                    "::rust::Slice<{ty}{is_const}>",
                    is_const = is_const,
                    ty = syn_type_to_cpp_type(elem, type_names)?
                )),
                // str is a special type only available as a reference
                // We need to map &str to rust::Str
                // Note that CXX does not support &mut str
                Type::Path(ty_path) if ty_path.path.is_ident("str") => Ok("::rust::Str".to_owned()),
                // Other types pass through as normal
                _others => Ok(format!(
                    "{ty}{is_const}&",
                    is_const = is_const,
                    ty = syn_type_to_cpp_type(elem, type_names)?
                )),
            }
        }
        Type::Tuple(tuple) if tuple.elems.is_empty() => Ok("void".to_string()),
        _others => Err(Error::new(
            ty.span(),
            format!("Unsupported type: {_others:?}"),
        )),
    }
}

/// Convert any generic arguments to C++, eg A and B in Ty<A, B>
fn generic_argument_to_string(generic: &GenericArgument, type_names: &TypeNames) -> Result<String> {
    match generic {
        GenericArgument::Type(ty) => syn_type_to_cpp_type(ty, type_names),
        _others => Err(Error::new(
            generic.span(),
            "Unsupported GenericArgument type",
        )),
    }
}

/// Convert the arguments for a path to C++, eg this is the whole <T> block
fn path_argument_to_string(
    args: &PathArguments,
    type_names: &TypeNames,
) -> Result<Option<Vec<String>>> {
    match args {
        PathArguments::AngleBracketed(angled) => Ok(Some(
            angled
                .args
                .iter()
                .map(|generic| generic_argument_to_string(generic, type_names))
                .collect::<Result<Vec<String>>>()?,
        )),
        PathArguments::Parenthesized(_) => Err(Error::new(
            args.span(),
            "Parenthesized arguments are unsupported",
        )),
        PathArguments::None => Ok(None),
    }
}

/// Convert a segment of a path to C++
fn path_segment_to_string(segment: &PathSegment, type_names: &TypeNames) -> Result<String> {
    let ident = &segment.ident;
    let ident_string = ident.to_string();

    // If we are a Pin<T> then for C++ it becomes just T
    let arg = match &*ident_string {
        "Pin" => {
            let mut args =
                path_argument_to_string(&segment.arguments, type_names)?.unwrap_or_else(Vec::new);

            if args.len() != 1 {
                return Err(Error::new(segment.span(), "Pin must have one argument"));
            }
            return Ok(args.pop().unwrap());
        }
        "Result" => {
            return Err(Error::new(segment.span(), "Result is not supported"));
        }
        "Option" => {
            return Err(Error::new(segment.span(), "Option is not supported"));
        }
        _others => {
            path_argument_to_string(&segment.arguments, type_names)?.map(|values| values.join(", "))
        }
    };

    // If there are template args check that its a supported template type.
    if let Some(arg) = arg {
        // A built in template base cannot have a cxx_name or a namespace
        if let Some(ident) = possible_built_in_template_base(&ident_string) {
            Ok(format!("{ident}<{arg}>"))
        } else {
            Err(Error::new_spanned(
                ident,
                format!("Unsupported template base: {ident}"),
            ))
        }
    } else {
        type_names.cxx_qualified(&segment.ident)
    }
}

/// Convert any templated bases to known C++ equivalents
///
/// This is similar to the method in CXX
/// https://github.com/dtolnay/cxx/blob/a6e1cd1e8d9d6df20e88e7443963dc4c5c8c4875/gen/src/write.rs#L1213
fn possible_built_in_template_base(ty: &str) -> Option<&str> {
    match ty {
        "Box" => Some("::rust::Box"),
        "Vec" => Some("::rust::Vec"),
        "UniquePtr" => Some("::std::unique_ptr"),
        "SharedPtr" => Some("::std::shared_ptr"),
        "WeakPtr" => Some("::std::weak_ptr"),
        "CxxVector" => Some("::std::vector"),
        _others => None,
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_syn_return_type_to_cpp_except_default() {
        let ty = parse_quote! {};
        assert_eq!(syn_return_type_to_cpp_except(&ty), "noexcept");
    }

    #[test]
    fn test_syn_return_type_to_cpp_except_result() {
        let ty = parse_quote! { -> Result<T> };
        assert_eq!(syn_return_type_to_cpp_except(&ty), "");
    }

    #[test]
    fn test_syn_return_type_to_cpp_except_type() {
        let ty = parse_quote! { -> T };
        assert_eq!(syn_return_type_to_cpp_except(&ty), "noexcept");
    }

    #[test]
    fn test_syn_return_type_to_cpp_except_type_ptr() {
        let ty = parse_quote! { -> *mut T };
        assert_eq!(syn_return_type_to_cpp_except(&ty), "noexcept");
    }

    macro_rules! test_syn_types_to_cpp_types {
        [$($input_type:tt => $output_type:literal),*] => {
            let mut type_names = TypeNames::default();
            // Add some types to the list of available types so we can use them in tests.
            type_names.mock_insert("T", None, None, None);
            type_names.mock_insert("QColor", None, None, None);
            type_names.mock_insert("QPoint", None, None, None);
            $(
            assert_eq!(
                syn_type_to_cpp_type(&parse_quote! $input_type, &type_names).unwrap(),
                $output_type);
            )*
        }
    }

    #[test]
    fn test_syn_type_to_cpp_type() {
        test_syn_types_to_cpp_types! [
            { i32 } => "::std::int32_t",
            { () } => "void",
            { fn() } => "::rust::Fn<void, ()>",
            { fn(i32, i32) -> bool } => "::rust::Fn<bool, (::std::int32_t, ::std::int32_t)>",
            { [i32; 10] } => "::std::array<::std::int32_t, 10>",
            { Vec<&str> } => "::rust::Vec<::rust::Str>",
            { &str } => "::rust::Str",
            { &mut [i32] } => "::rust::Slice<::std::int32_t>",
            { &[i32] } => "::rust::Slice<::std::int32_t const>",
            { Pin<&mut UniquePtr<T>> } => "::std::unique_ptr<T>&",
            { Pin<&UniquePtr<T>> } => "::std::unique_ptr<T> const&",
            { Pin<UniquePtr<T>> } => "::std::unique_ptr<T>",
            { Pin<&mut T> } => "T&",
            { Pin<&T> } => "T const&",
            { Pin<T> } => "T",
            { &mut UniquePtr<*const T> } => "::std::unique_ptr<const T*>&",
            { &mut UniquePtr<*mut T> } => "::std::unique_ptr<T*>&",
            { &mut UniquePtr<QColor> } => "::std::unique_ptr<QColor>&",
            { &Vec<*const T> } => "::rust::Vec<const T*> const&",
            { &Vec<*mut T> } => "::rust::Vec<T*> const&",
            { &Vec<f64> } => "::rust::Vec<double> const&",
            { UniquePtr<QColor> } => "::std::unique_ptr<QColor>",
            { Vec<f64> } => "::rust::Vec<double>",
            { *const T } => "const T*",
            { *mut T } => "T*",
            { &mut *const T } => "const T*&",
            { &mut *mut T } => "T*&",
            { &*const T } => "const T* const&",
            { &*mut T } => "T* const&",
            { &mut QPoint } => "QPoint&",
            { &QPoint } => "QPoint const&",
            { QPoint } => "QPoint",
            { SharedPtr<T> } => "::std::shared_ptr<T>",
            { WeakPtr<T> } => "::std::weak_ptr<T>",
            { CxxVector<T> } => "::std::vector<T>"
        ];
    }

    #[test]
    fn test_syn_type_invalid() {
        let ty = parse_quote! { (A) };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), None);
        assert!(syn_type_to_cpp_type(&ty, &type_names).is_err());

        let ty = parse_quote! { Option<A> };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), None);
        assert!(syn_type_to_cpp_type(&ty, &type_names).is_err());

        let ty = parse_quote! { Result<A> };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), None);
        assert!(syn_type_to_cpp_type(&ty, &type_names).is_err());

        let ty = parse_quote! { Pin<> };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), None);
        assert!(syn_type_to_cpp_type(&ty, &type_names).is_err());

        let ty = parse_quote! { Vec<'a,T> };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), None);
        assert!(syn_type_to_cpp_type(&ty, &type_names).is_err());

        let ty = parse_quote! { f32::f32::f32 };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), None);
        assert!(syn_type_to_cpp_type(&ty, &type_names).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_type_no_template() {
        let ty = parse_quote! { NotATemplate<A> };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), None);
        assert!(syn_type_to_cpp_type(&ty, &type_names).is_err(),);
    }

    #[test]
    fn test_syn_type_to_cpp_type_mapped() {
        let ty = parse_quote! { A };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), None);
        assert_eq!(syn_type_to_cpp_type(&ty, &type_names).unwrap(), "A1");
    }

    #[test]
    fn test_syn_type_to_cpp_type_mapped_with_namespace() {
        let ty = parse_quote! { A };
        let mut type_names = TypeNames::default();
        type_names.mock_insert("A", None, Some("A1"), Some("N1"));
        assert_eq!(syn_type_to_cpp_type(&ty, &type_names).unwrap(), "N1::A1");
    }

    #[test]
    fn test_syn_type_to_cpp_type_array_length_zero() {
        let ty = parse_quote! { [i32; 0] };
        assert!(syn_type_to_cpp_type(&ty, &TypeNames::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_type_array_length_invalid() {
        let ty = parse_quote! { [i32; String] };
        assert!(syn_type_to_cpp_type(&ty, &TypeNames::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_type_array_length_non_integer() {
        let ty = parse_quote! { [i32; 1.5f32] };
        assert!(syn_type_to_cpp_type(&ty, &TypeNames::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_type_tuple_multiple() {
        let ty = parse_quote! { (i32, ) };
        assert!(syn_type_to_cpp_type(&ty, &TypeNames::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_none() {
        let ty = parse_quote! {};
        assert_eq!(
            syn_type_to_cpp_return_type(&ty, &TypeNames::default()).unwrap(),
            None
        );
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_normal() {
        let ty = parse_quote! { -> bool };
        assert_eq!(
            syn_type_to_cpp_return_type(&ty, &TypeNames::default()).unwrap(),
            Some("bool".to_string())
        );
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_result_bool() {
        let ty = parse_quote! { -> Result<bool> };
        assert_eq!(
            syn_type_to_cpp_return_type(&ty, &TypeNames::default()).unwrap(),
            Some("bool".to_string())
        );
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_result_empty() {
        let ty = parse_quote! { -> Result<> };
        assert!(syn_type_to_cpp_return_type(&ty, &TypeNames::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_result_multiple() {
        let ty = parse_quote! { -> Result<A, B, C> };
        assert!(syn_type_to_cpp_return_type(&ty, &TypeNames::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_result_tuple() {
        let ty = parse_quote! { -> Result<()> };
        assert_eq!(
            syn_type_to_cpp_return_type(&ty, &TypeNames::default()).unwrap(),
            None
        );
    }
}
