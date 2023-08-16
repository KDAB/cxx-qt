// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::mappings::ParsedCxxMappings;
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
    cxx_mapping: &ParsedCxxMappings,
) -> Result<Option<String>> {
    if let ReturnType::Type(_, ty) = return_ty {
        // If we are a Result<T> then we just become T for C++
        if let Type::Path(ty_path) = &**ty {
            if let Some(segment) = ty_path.path.segments.first() {
                if segment.ident == "Result" {
                    let mut args = path_argument_to_string(&segment.arguments, cxx_mapping)?
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
                        unreachable!("Args should be of length 1");
                    }
                }
            }
        }

        syn_type_to_cpp_type(ty, cxx_mapping).map(|v| if v == "void" { None } else { Some(v) })
    } else {
        Ok(None)
    }
}

/// For a given Rust type attempt to generate a C++ string
///
/// This is similar to the parsing in CXX
/// https://github.com/dtolnay/cxx/blob/a6e1cd1e8d9d6df20e88e7443963dc4c5c8c4875/syntax/parse.rs#L1134
pub(crate) fn syn_type_to_cpp_type(ty: &Type, cxx_mapping: &ParsedCxxMappings) -> Result<String> {
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
                ty = syn_type_to_cpp_type(elem, cxx_mapping)?,
                len = len
            ))
        }
        Type::BareFn(TypeBareFn { inputs, output, .. }) => {
            let ret = if let ReturnType::Type(_, ty) = output {
                syn_type_to_cpp_type(ty, cxx_mapping)?
            } else {
                "void".to_owned()
            };

            let args = inputs
                .iter()
                .map(|arg| syn_type_to_cpp_type(&arg.ty, cxx_mapping))
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
                .map(|generic| path_segment_to_string(generic, cxx_mapping))
                .collect::<Result<Vec<String>>>()?;
            if ty_strings.len() == 1 {
                let first = ty_strings.first().unwrap();

                // A built in type cannot have a cxx_name or a namespace
                if let Some(built_in) = possible_built_in(first) {
                    return Ok(built_in.to_owned());
                }

                // Use the CXX mapped name
                Ok(cxx_mapping.cxx(first))
            } else {
                Ok(ty_strings.join("::"))
            }
        }
        Type::Ptr(TypePtr {
            const_token, elem, ..
        }) => Ok(format!(
            "{is_const}{ty}*",
            is_const = if const_token.is_some() { "const " } else { "" },
            ty = syn_type_to_cpp_type(elem, cxx_mapping)?
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
                    ty = syn_type_to_cpp_type(elem, cxx_mapping)?
                )),
                // str is a special type only available as a reference
                // We need to map &str to rust::Str
                // Note that CXX does not support &mut str
                Type::Path(ty_path) if ty_path.path.is_ident("str") => Ok("::rust::Str".to_owned()),
                // Other types pass through as normal
                _others => Ok(format!(
                    "{ty}{is_const}&",
                    is_const = is_const,
                    ty = syn_type_to_cpp_type(elem, cxx_mapping)?
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
fn generic_argument_to_string(
    generic: &GenericArgument,
    cxx_mapping: &ParsedCxxMappings,
) -> Result<String> {
    match generic {
        GenericArgument::Type(ty) => syn_type_to_cpp_type(ty, cxx_mapping),
        _others => Err(Error::new(
            generic.span(),
            "Unsupported GenericArgument type",
        )),
    }
}

/// Convert the arguments for a path to C++, eg this is the whole <T> block
fn path_argument_to_string(
    args: &PathArguments,
    cxx_mapping: &ParsedCxxMappings,
) -> Result<Option<Vec<String>>> {
    match args {
        PathArguments::AngleBracketed(angled) => Ok(Some(
            angled
                .args
                .iter()
                .map(|generic| generic_argument_to_string(generic, cxx_mapping))
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
fn path_segment_to_string(
    segment: &PathSegment,
    cxx_mapping: &ParsedCxxMappings,
) -> Result<String> {
    let mut ident = segment.ident.to_string();

    // If we are a Pin<T> then for C++ it becomes just T
    let args = match ident.as_str() {
        "Pin" => {
            ident = path_argument_to_string(&segment.arguments, cxx_mapping)?
                .map_or_else(|| "".to_owned(), |values| values.join(", "));

            None
        }
        "Result" => {
            return Err(Error::new(segment.span(), "Result is not supported"));
        }
        "Option" => {
            return Err(Error::new(segment.span(), "Option is not supported"));
        }
        _others => path_argument_to_string(&segment.arguments, cxx_mapping)?
            .map(|values| values.join(", ")),
    };

    // If there are template args check that we aren't a recognised A of A<B>
    if args.is_some() {
        // A built in template base cannot have a cxx_name or a namespace
        ident = if let Some(built_in) = possible_built_in_template_base(&ident) {
            built_in.to_owned()
        } else {
            cxx_mapping.cxx(&ident)
        };
    }

    Ok(format!(
        "{ident}{args}",
        ident = ident,
        args = args.map_or_else(|| "".to_owned(), |arg| format!("<{arg}>"))
    ))
}

/// Convert any built in types to known C++ equivalents
///
/// This is similar to the methods in CXX
/// https://github.com/dtolnay/cxx/blob/9c1737feff7208cd4825984614beaf09a27aefcf/syntax/atom.rs#L30
/// https://github.com/dtolnay/cxx/blob/a6e1cd1e8d9d6df20e88e7443963dc4c5c8c4875/gen/src/write.rs#L1311
fn possible_built_in(ty: &str) -> Option<&str> {
    match ty {
        "bool" => Some("bool"),
        "c_char" => Some("char"),
        "u8" => Some("::std::uint8_t"),
        "u16" => Some("::std::uint16_t"),
        "u32" => Some("::std::uint32_t"),
        "u64" => Some("::std::uint64_t"),
        "usize" => Some("::std::size_t"),
        "i8" => Some("::std::int8_t"),
        "i16" => Some("::std::int16_t"),
        "i32" => Some("::std::int32_t"),
        "i64" => Some("::std::int64_t"),
        "isize" => Some("::rust::isize"),
        "f32" => Some("float"),
        "f64" => Some("double"),
        "CxxString" => Some("::std::string"),
        "String" => Some("::rust::String"),
        // TODO: handle pointer
        _others => None,
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

/// A trait to allow indenting multi-line string
/// This is specifically useful when using formatdoc! with a multi-line string argument.
/// As the formatdoc! formatting doesn't support indenting multi-line arguments, we can indent
/// those ourselves.
pub(crate) trait Indent {
    fn indented(&self, indent: usize) -> String;
}

impl Indent for str {
    fn indented(&self, indent: usize) -> String {
        self.lines()
            .map(|line| " ".repeat(indent) + line)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use indoc::{formatdoc, indoc};
    use pretty_assertions::assert_str_eq;
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
    fn test_syn_type_to_cpp_type_built_in_one_part() {
        let ty = parse_quote! { i32 };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::int32_t"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_unknown_one_part() {
        let ty = parse_quote! { QPoint };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "QPoint"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_ref_const_one_part() {
        let ty = parse_quote! { &QPoint };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "QPoint const&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_ref_mut_one_part() {
        let ty = parse_quote! { &mut QPoint };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "QPoint&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_ref_const_ptr_mut_one_part() {
        let ty = parse_quote! { &*mut T };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T* const&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_ref_const_ptr_const_one_part() {
        let ty = parse_quote! { &*const T };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "const T* const&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_ref_mut_ptr_mut_one_part() {
        let ty = parse_quote! { &mut *mut T };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T*&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_ref_mut_ptr_const_one_part() {
        let ty = parse_quote! { &mut *const T };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "const T*&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_ptr_mut_one_part() {
        let ty = parse_quote! { *mut T };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T*"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_ptr_const_one_part() {
        let ty = parse_quote! { *const T };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "const T*"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_templated_built_in() {
        let ty = parse_quote! { Vec<f64> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Vec<double>"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_templated_unknown() {
        let ty = parse_quote! { UniquePtr<QColor> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<QColor>"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_templated_built_in_ref_const() {
        let ty = parse_quote! { &Vec<f64> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Vec<double> const&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_templated_built_in_ptr_mut() {
        let ty = parse_quote! { &Vec<*mut T> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Vec<T*> const&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_templated_built_in_ptr_const() {
        let ty = parse_quote! { &Vec<*const T> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Vec<const T*> const&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_templated_unknown_ref_mut() {
        let ty = parse_quote! { &mut UniquePtr<QColor> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<QColor>&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_templated_unknown_ptr_mut() {
        let ty = parse_quote! { &mut UniquePtr<*mut T> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<T*>&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_templated_unknown_ptr_const() {
        let ty = parse_quote! { &mut UniquePtr<*const T> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<const T*>&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_mapped() {
        let ty = parse_quote! { A };
        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_names
            .insert("A".to_owned(), "A1".to_owned());
        assert_eq!(syn_type_to_cpp_type(&ty, &cxx_mappings).unwrap(), "A1");
    }

    #[test]
    fn test_syn_type_to_cpp_type_mapped_with_namespace() {
        let ty = parse_quote! { A };
        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_names
            .insert("A".to_owned(), "A1".to_owned());
        cxx_mappings
            .namespaces
            .insert("A".to_owned(), "N1".to_owned());
        assert_eq!(
            syn_type_to_cpp_type(&ty, &cxx_mappings).unwrap(),
            "::N1::A1"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_pin() {
        let ty = parse_quote! { Pin<T> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_pin_ref() {
        let ty = parse_quote! { Pin<&T> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T const&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_pin_ref_mut() {
        let ty = parse_quote! { Pin<&mut T> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_pin_template() {
        let ty = parse_quote! { Pin<UniquePtr<T>> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<T>"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_pin_template_ref() {
        let ty = parse_quote! { Pin<&UniquePtr<T>> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<T> const&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_pin_template_ref_mut() {
        let ty = parse_quote! { Pin<&mut UniquePtr<T>> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<T>&"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_slice() {
        let ty = parse_quote! { &[i32] };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Slice<::std::int32_t const>"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_slice_mut() {
        let ty = parse_quote! { &mut [i32] };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Slice<::std::int32_t>"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_str() {
        let ty = parse_quote! { &str };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Str"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_str_template() {
        let ty = parse_quote! { Vec<&str> };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Vec<::rust::Str>"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_array() {
        let ty = parse_quote! { [i32; 10] };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::array<::std::int32_t, 10>"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_array_length_zero() {
        let ty = parse_quote! { [i32; 0] };
        assert!(syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_type_array_length_invalid() {
        let ty = parse_quote! { [i32; String] };
        assert!(syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_type_fn() {
        let ty = parse_quote! { fn(i32, i32) -> bool };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Fn<bool, (::std::int32_t, ::std::int32_t)>"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_fn_void() {
        let ty = parse_quote! { fn() };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Fn<void, ()>"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_tuple_empty() {
        let ty = parse_quote! { () };
        assert_eq!(
            syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            "void"
        );
    }

    #[test]
    fn test_syn_type_to_cpp_type_tuple_multiple() {
        let ty = parse_quote! { (i32, ) };
        assert!(syn_type_to_cpp_type(&ty, &ParsedCxxMappings::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_none() {
        let ty = parse_quote! {};
        assert_eq!(
            syn_type_to_cpp_return_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            None
        );
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_normal() {
        let ty = parse_quote! { -> bool };
        assert_eq!(
            syn_type_to_cpp_return_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            Some("bool".to_string())
        );
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_result_bool() {
        let ty = parse_quote! { -> Result<bool> };
        assert_eq!(
            syn_type_to_cpp_return_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            Some("bool".to_string())
        );
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_result_empty() {
        let ty = parse_quote! { -> Result<> };
        assert!(syn_type_to_cpp_return_type(&ty, &ParsedCxxMappings::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_result_multiple() {
        let ty = parse_quote! { -> Result<A, B, C> };
        assert!(syn_type_to_cpp_return_type(&ty, &ParsedCxxMappings::default()).is_err());
    }

    #[test]
    fn test_syn_type_to_cpp_return_type_result_tuple() {
        let ty = parse_quote! { -> Result<()> };
        assert_eq!(
            syn_type_to_cpp_return_type(&ty, &ParsedCxxMappings::default()).unwrap(),
            None
        );
    }

    #[test]
    fn indent_string() {
        let multiline_string = indoc! { r#"
            A,
            B,
        "#};

        assert_str_eq!(
            formatdoc! { r#"
                enum Test {{
                {multiline_string}
                }}
            "#, multiline_string = multiline_string.indented(2) },
            indoc! { r#"
                enum Test {
                  A,
                  B,
                }
            "#}
        );
    }
}
