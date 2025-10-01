// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::qenum::ParsedQEnum;
use quote::{format_ident, quote_spanned};
use syn::{parse_quote_spanned, spanned::Spanned};

use super::fragment::GeneratedRustFragment;

pub fn generate(qenums: &[ParsedQEnum]) -> Vec<GeneratedRustFragment> {
    qenums
        .iter()
        .map(|qenum| {
            if qenum.qobject.is_none() {
                generate_standalone_qenum(qenum)
            } else {
                generate_member_qenum(qenum)
            }
        })
        .collect()
}

// Previously we used CXX to generate ourselves a C++ enum that we then imported into the QObject
// class.
//
// However, this broke in Qt 6.9.2, see https://github.com/KDAB/cxx-qt/issues/1328
//
// We now have to emulate what CXX generates on the Rust side ourselves.
// By then using just an import inside the CXX bridge, the include order works out on the C++ side.
// The general problem is that the order in C++ usually is:
//
// 1. forward-declare everything for CXX
// 2. Include the CXX generated header, which checks the types exist
// 3. Define the QObject class
//
// The problem is that we cannot forward-declare a member enum inside the QObject class.
// So then we have to basically let CXX think this is a custom trivial type, which it only checks inside
// the generated cpp file, not the header file.
fn generate_member_qenum(qenum: &ParsedQEnum) -> GeneratedRustFragment {
    let item = &qenum.item;
    let docs = &qenum.docs;
    let cfgs = &qenum.cfgs;
    let (qenum_ident, qenum_attrs, _qualified) = qenum.name.clone().into_cxx_parts();
    let qenum_ident_str = qenum_ident.to_string();

    let variants = qenum
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            // Note: The types here must match with the `repr` inside the struct, as quote will
            // emit the type that exactly matches the input type.
            let index = index as i32;
            quote_spanned! {
                variant.span() =>
                pub const #variant: #qenum_ident = #qenum_ident { repr: #index };
            }
        })
        .collect::<Vec<_>>();

    let module_name = format_ident!("cxx_qt_private_qenum_{qenum_ident}");

    // TODO: Support different repr types
    GeneratedRustFragment {
        cxx_mod_contents: vec![parse_quote_spanned! {
            item.span() =>
            #(#cfgs)*
            extern "C++" {
                #[allow(private_interfaces)]
                #(#docs)*
                #(#qenum_attrs)*
                type #qenum_ident = super::#module_name::#qenum_ident;
            }
        }],
        cxx_qt_mod_contents: vec![parse_quote_spanned! {
            item.span() =>

            #(#cfgs)*
            mod #module_name {
                #(#docs)*
                #[derive(PartialEq, Eq, Clone, Copy)]
                #[repr(transparent)]
                pub(super) struct #qenum_ident {
                    #[allow(missing_docs)]
                    pub repr: i32
                }

                #[allow(non_upper_case_globals)]
                impl #qenum_ident {
                    #(#variants)*
                }

                #[automatically_derived]
                unsafe impl ::cxx::ExternType for #qenum_ident {
                    type Id = ::cxx::type_id!(#qenum_ident_str);
                    type Kind = ::cxx::kind::Trivial;
                }
            }
        }],
    }
}

fn generate_standalone_qenum(qenum: &ParsedQEnum) -> GeneratedRustFragment {
    let (qenum_ident, qenum_attrs, _qualified) = &qenum.name.clone().into_cxx_parts();
    let item = &qenum.item;
    let vis = &item.vis;
    let variants = &item.variants;
    let docs = &qenum.docs;
    let cfgs = &qenum.cfgs;

    GeneratedRustFragment {
        cxx_mod_contents: vec![
            parse_quote_spanned! {
                item.span() =>
                    #[repr(i32)]
                    #(#cfgs)*
                    #(#docs)*
                    #(#qenum_attrs)*
                    #vis enum #qenum_ident {
                        #variants
                    }
            },
            parse_quote_spanned! {
                item.span() =>
                    extern "C++" {
                        #(#cfgs)*
                        #(#qenum_attrs)*
                        type #qenum_ident;
                    }
            },
        ],
        cxx_qt_mod_contents: vec![],
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::assert_tokens_eq;
    use quote::{format_ident, quote};
    use syn::parse_quote;

    use super::*;

    #[test]
    fn generates_namespaced() {
        let qenums = vec![ParsedQEnum::parse(
            parse_quote! {
                /// Doc comment
                #[namespace="my_namespace"]
                enum MyEnum {
                    /// Document Variant1
                    Variant1,
                    /// Document Variant2
                    Variant2,
                }
            },
            None,
            None,
            &format_ident!("qobject"),
        )
        .unwrap()];

        let generated = generate(&qenums);
        assert_eq!(generated.len(), 1);
        let generated = &generated[0];
        assert!(generated.cxx_qt_mod_contents.is_empty());
        assert_eq!(generated.cxx_mod_contents.len(), 2);
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                #[repr(i32)]
                #[doc = r" Doc comment"]
                #[namespace = "my_namespace"]
                enum MyEnum {
                    #[doc = r" Document Variant1"]
                    Variant1,
                    #[doc = r" Document Variant2"]
                    Variant2,
                }
            },
        );
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                extern "C++" {
                    #[namespace = "my_namespace"]
                    type MyEnum;
                }
            },
        )
    }

    #[test]
    fn generates_member() {
        let qenums = vec![ParsedQEnum::parse(
            parse_quote! {
                /// Doc comment
                #[namespace="my_namespace"]
                enum MyEnum {
                    /// Document Variant1
                    Variant1,
                    /// Document Variant2
                    Variant2,
                }
            },
            // The Ident of the QObject shouldn't matter
            Some(format_ident!("MyObject")),
            None,
            &format_ident!("qobject"),
        )
        .unwrap()];
        let generated = generate(&qenums);
        assert_eq!(generated.len(), 1);
        let generated = &generated[0];
        assert_eq!(generated.cxx_mod_contents.len(), 1);
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                extern "C++" {
                    #[allow(private_interfaces)]
                    #[doc = r" Doc comment"]
                    #[namespace = "my_namespace"]
                    type MyEnum = super::cxx_qt_private_qenum_MyEnum::MyEnum;
                }
            },
        );
        assert_eq!(generated.cxx_qt_mod_contents.len(), 1);
        assert_tokens_eq(
            &generated.cxx_qt_mod_contents[0],
            quote! {
                mod cxx_qt_private_qenum_MyEnum {
                    #[doc = r" Doc comment"]
                    #[derive(PartialEq, Eq, Clone, Copy)]
                    #[repr(transparent)]
                    pub(super) struct MyEnum {
                        #[allow(missing_docs)]
                        pub repr: i32
                    }
                    #[allow(non_upper_case_globals)]
                    impl MyEnum {
                        pub const Variant1: MyEnum = MyEnum { repr: 0i32 };
                        pub const Variant2: MyEnum = MyEnum { repr: 1i32 };
                    }
                    #[automatically_derived]
                    unsafe impl ::cxx::ExternType for MyEnum {
                        type Id = ::cxx::type_id!("MyEnum");
                        type Kind = ::cxx::kind::Trivial;
                    }
                }
            },
        );
    }
}
