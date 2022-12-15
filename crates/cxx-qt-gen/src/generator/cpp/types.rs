// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::cxxqtdata::ParsedCxxMappings;
use syn::{
    spanned::Spanned, Error, Expr, GenericArgument, Lit, PathArguments, PathSegment, Result,
    ReturnType, Type, TypeArray, TypeBareFn, TypePtr, TypeReference, TypeSlice,
};

/// A helper for describing a C++ type
///
/// If a cxx_type has been specified in the attribute then use it
/// otherwise parse the Rust type as the C++ type.
///
/// This is useful where you have UniquePtr<T> as the Rust type but
/// need T as the C++ type.
pub struct CppType {
    cxx_type: Option<String>,
    ty: String,
}

impl CppType {
    /// Retrieve either the cxx_type attribute value or the Rust type
    pub fn as_cxx_ty(&self) -> &str {
        if let Some(cxx_type) = &self.cxx_type {
            cxx_type.as_str()
        } else {
            self.as_rust_ty()
        }
    }

    /// Retrieve the Rust type in C++ form
    pub fn as_rust_ty(&self) -> &str {
        &self.ty
    }

    /// Construct a [CppType] from a given [syn::Type] and the contents of the cxx_type attribute
    pub fn from(
        ty: &Type,
        cxx_type: &Option<String>,
        cxx_mapping: &ParsedCxxMappings,
    ) -> Result<CppType> {
        Ok(CppType {
            cxx_type: cxx_type.clone(),
            ty: to_cpp_string(ty, cxx_mapping)?,
        })
    }
}

/// For a given Rust type attempt to generate a C++ string
///
/// This is similar to the parsing in CXX
/// https://github.com/dtolnay/cxx/blob/a6e1cd1e8d9d6df20e88e7443963dc4c5c8c4875/syntax/parse.rs#L1134
fn to_cpp_string(ty: &Type, cxx_mapping: &ParsedCxxMappings) -> Result<String> {
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
                ty = to_cpp_string(elem, cxx_mapping)?,
                len = len
            ))
        }
        Type::BareFn(TypeBareFn { inputs, output, .. }) => {
            let ret = if let ReturnType::Type(_, ty) = output {
                to_cpp_string(ty, cxx_mapping)?
            } else {
                "void".to_owned()
            };

            let args = inputs
                .iter()
                .map(|arg| to_cpp_string(&arg.ty, cxx_mapping))
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

                // Check if there is a cxx_name or namespace to handle
                let cxx_name = cxx_mapping.cxx_name.get(first).unwrap_or(first);
                if let Some(namespace) = cxx_mapping.namespace.get(first) {
                    Ok(format!("::{namespace}::{cxx_name}"))
                } else {
                    Ok(cxx_name.to_owned())
                }
            } else {
                Ok(ty_strings.join("::"))
            }
        }
        Type::Ptr(TypePtr {
            const_token, elem, ..
        }) => Ok(format!(
            "{is_const}{ty}*",
            is_const = if const_token.is_some() { "const " } else { "" },
            ty = to_cpp_string(elem, cxx_mapping)?
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
                    ty = to_cpp_string(elem, cxx_mapping)?
                )),
                // str is a special type only available as a reference
                // We need to map &str to rust::Str
                // Note that CXX does not support &mut str
                Type::Path(ty_path) if ty_path.path.is_ident("str") => Ok("::rust::Str".to_owned()),
                // Other types pass through as normal
                _others => Ok(format!(
                    "{ty}{is_const}&",
                    is_const = is_const,
                    ty = to_cpp_string(elem, cxx_mapping)?
                )),
            }
        }
        // TODO: consider Type::Tuple with an empty tuple mapping to void
        //
        // TODO: handling Result<T> is tricky, as we need Result<T> in the CXX bridge
        // but potentially Result<T, E> on the method. Then we need to detect that we have
        // Result<()> and notice that that is a void return, otherwise we try to convert
        // void which fails in C++
        _others => Err(Error::new(
            ty.span(),
            format!("Unsupported type: {:?}", _others),
        )),
    }
}

/// Convert any generic arguments to C++, eg A and B in Ty<A, B>
fn generic_argument_to_string(
    generic: &GenericArgument,
    cxx_mapping: &ParsedCxxMappings,
) -> Result<String> {
    match generic {
        GenericArgument::Type(ty) => to_cpp_string(ty, cxx_mapping),
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
    let args = if ident == "Pin" {
        ident = path_argument_to_string(&segment.arguments, cxx_mapping)?
            .map_or_else(|| "".to_owned(), |values| values.join(", "));

        None
    } else {
        path_argument_to_string(&segment.arguments, cxx_mapping)?.map(|values| values.join(", "))
    };

    // If there are template args check that we aren't a recognised A of A<B>
    if args.is_some() {
        // A built in template base cannot have a cxx_name or a namespace
        ident = if let Some(built_in) = possible_built_in_template_base(&ident) {
            built_in.to_owned()
        } else {
            // Check if there is a cxx_name or namespace to handle
            let cxx_name = cxx_mapping.cxx_name.get(&ident).unwrap_or(&ident);
            if let Some(namespace) = cxx_mapping.namespace.get(&ident) {
                format!("::{namespace}::{cxx_name}")
            } else {
                cxx_name.to_owned()
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;

    #[test]
    fn test_cxx_type_with_attribute() {
        let ty = tokens_to_syn(quote! { UniquePtr<QColor> });
        let cxx_ty = CppType::from(
            &ty,
            &Some("QColor".to_owned()),
            &ParsedCxxMappings::default(),
        )
        .unwrap();
        assert_eq!(cxx_ty.as_cxx_ty(), "QColor");
        assert_eq!(cxx_ty.as_rust_ty(), "::std::unique_ptr<QColor>");
    }

    #[test]
    fn test_cxx_type_without_attribute() {
        let ty = tokens_to_syn(quote! { UniquePtr<QColor> });
        let cxx_ty = CppType::from(&ty, &None, &ParsedCxxMappings::default()).unwrap();
        assert_eq!(cxx_ty.as_cxx_ty(), "::std::unique_ptr<QColor>");
        assert_eq!(cxx_ty.as_rust_ty(), "::std::unique_ptr<QColor>");
    }

    #[test]
    fn test_cxx_type_mapped() {
        let ty = tokens_to_syn(quote! { A });
        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_name
            .insert("A".to_owned(), "A1".to_owned());
        let cxx_ty = CppType::from(&ty, &None, &cxx_mappings).unwrap();
        assert_eq!(cxx_ty.as_cxx_ty(), "A1");
        assert_eq!(cxx_ty.as_rust_ty(), "A1");
    }

    #[test]
    fn test_cxx_type_mapped_with_attribute() {
        let ty = tokens_to_syn(quote! { A });
        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_name
            .insert("A".to_owned(), "A1".to_owned());
        let cxx_ty = CppType::from(&ty, &Some("B1".to_owned()), &cxx_mappings).unwrap();
        assert_eq!(cxx_ty.as_cxx_ty(), "B1");
        assert_eq!(cxx_ty.as_rust_ty(), "A1");
    }

    #[test]
    fn test_to_cpp_string_built_in_one_part() {
        let ty = tokens_to_syn(quote! { i32 });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::int32_t"
        );
    }

    #[test]
    fn test_to_cpp_string_unknown_one_part() {
        let ty = tokens_to_syn(quote! { QPoint });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "QPoint"
        );
    }

    #[test]
    fn test_to_cpp_string_ref_const_one_part() {
        let ty = tokens_to_syn(quote! { &QPoint });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "QPoint const&"
        );
    }

    #[test]
    fn test_to_cpp_string_ref_mut_one_part() {
        let ty = tokens_to_syn(quote! { &mut QPoint });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "QPoint&"
        );
    }

    #[test]
    fn test_to_cpp_string_ref_const_ptr_mut_one_part() {
        let ty = tokens_to_syn(quote! { &*mut T });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T* const&"
        );
    }

    #[test]
    fn test_to_cpp_string_ref_const_ptr_const_one_part() {
        let ty = tokens_to_syn(quote! { &*const T });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "const T* const&"
        );
    }

    #[test]
    fn test_to_cpp_string_ref_mut_ptr_mut_one_part() {
        let ty = tokens_to_syn(quote! { &mut *mut T });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T*&"
        );
    }

    #[test]
    fn test_to_cpp_string_ref_mut_ptr_const_one_part() {
        let ty = tokens_to_syn(quote! { &mut *const T });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "const T*&"
        );
    }

    #[test]
    fn test_to_cpp_string_ptr_mut_one_part() {
        let ty = tokens_to_syn(quote! { *mut T });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T*"
        );
    }

    #[test]
    fn test_to_cpp_string_ptr_const_one_part() {
        let ty = tokens_to_syn(quote! { *const T });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "const T*"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_built_in() {
        let ty = tokens_to_syn(quote! { Vec<f64> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Vec<double>"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_unknown() {
        let ty = tokens_to_syn(quote! { UniquePtr<QColor> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<QColor>"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_built_in_ref_const() {
        let ty = tokens_to_syn(quote! { &Vec<f64> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Vec<double> const&"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_built_in_ptr_mut() {
        let ty = tokens_to_syn(quote! { &Vec<*mut T> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Vec<T*> const&"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_built_in_ptr_const() {
        let ty = tokens_to_syn(quote! { &Vec<*const T> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Vec<const T*> const&"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_unknown_ref_mut() {
        let ty = tokens_to_syn(quote! { &mut UniquePtr<QColor> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<QColor>&"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_unknown_ptr_mut() {
        let ty = tokens_to_syn(quote! { &mut UniquePtr<*mut T> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<T*>&"
        );
    }

    #[test]
    fn test_to_cpp_string_templated_unknown_ptr_const() {
        let ty = tokens_to_syn(quote! { &mut UniquePtr<*const T> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<const T*>&"
        );
    }

    #[test]
    fn test_to_cpp_string_mapped() {
        let ty = tokens_to_syn(quote! { A });
        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_name
            .insert("A".to_owned(), "A1".to_owned());
        assert_eq!(to_cpp_string(&ty, &cxx_mappings).unwrap(), "A1");
    }

    #[test]
    fn test_to_cpp_string_mapped_with_namespace() {
        let ty = tokens_to_syn(quote! { A });
        let mut cxx_mappings = ParsedCxxMappings::default();
        cxx_mappings
            .cxx_name
            .insert("A".to_owned(), "A1".to_owned());
        cxx_mappings
            .namespace
            .insert("A".to_owned(), "N1".to_owned());
        assert_eq!(to_cpp_string(&ty, &cxx_mappings).unwrap(), "::N1::A1");
    }

    #[test]
    fn test_to_cpp_string_pin() {
        let ty = tokens_to_syn(quote! { Pin<T> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T"
        );
    }

    #[test]
    fn test_to_cpp_string_pin_ref() {
        let ty = tokens_to_syn(quote! { Pin<&T> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T const&"
        );
    }

    #[test]
    fn test_to_cpp_string_pin_ref_mut() {
        let ty = tokens_to_syn(quote! { Pin<&mut T> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "T&"
        );
    }

    #[test]
    fn test_to_cpp_string_pin_template() {
        let ty = tokens_to_syn(quote! { Pin<UniquePtr<T>> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<T>"
        );
    }

    #[test]
    fn test_to_cpp_string_pin_template_ref() {
        let ty = tokens_to_syn(quote! { Pin<&UniquePtr<T>> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<T> const&"
        );
    }

    #[test]
    fn test_to_cpp_string_pin_template_ref_mut() {
        let ty = tokens_to_syn(quote! { Pin<&mut UniquePtr<T>> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::unique_ptr<T>&"
        );
    }

    #[test]
    fn test_to_cpp_string_slice() {
        let ty = tokens_to_syn(quote! { &[i32] });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Slice<::std::int32_t const>"
        );
    }

    #[test]
    fn test_to_cpp_string_slice_mut() {
        let ty = tokens_to_syn(quote! { &mut [i32] });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Slice<::std::int32_t>"
        );
    }

    #[test]
    fn test_to_cpp_string_str() {
        let ty = tokens_to_syn(quote! { &str });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Str"
        );
    }

    #[test]
    fn test_to_cpp_string_str_template() {
        let ty = tokens_to_syn(quote! { Vec<&str> });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Vec<::rust::Str>"
        );
    }

    #[test]
    fn test_to_cpp_string_array() {
        let ty = tokens_to_syn(quote! { [i32; 10] });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::std::array<::std::int32_t, 10>"
        );
    }

    #[test]
    fn test_to_cpp_string_array_length_zero() {
        let ty = tokens_to_syn(quote! { [i32; 0] });
        assert!(to_cpp_string(&ty, &ParsedCxxMappings::default()).is_err());
    }

    #[test]
    fn test_to_cpp_string_array_length_invalid() {
        let ty = tokens_to_syn(quote! { [i32; String] });
        assert!(to_cpp_string(&ty, &ParsedCxxMappings::default()).is_err());
    }

    #[test]
    fn test_to_cpp_string_fn() {
        let ty = tokens_to_syn(quote! { fn(i32, i32) -> bool });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Fn<bool, (::std::int32_t, ::std::int32_t)>"
        );
    }

    #[test]
    fn test_to_cpp_string_fn_void() {
        let ty = tokens_to_syn(quote! { fn() });
        assert_eq!(
            to_cpp_string(&ty, &ParsedCxxMappings::default()).unwrap(),
            "::rust::Fn<void, ()>"
        );
    }
}
