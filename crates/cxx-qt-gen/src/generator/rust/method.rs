// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::generator::rust::get_params_tokens;
use crate::{
    generator::{
        naming::{method::QMethodName, qobject::QObjectNames},
        rust::fragment::{GeneratedRustFragment, RustFragmentPair},
    },
    parser::method::ParsedMethod,
};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Result};

pub fn generate_rust_methods(
    invokables: &Vec<&ParsedMethod>,
    qobject_idents: &QObjectNames,
) -> Result<GeneratedRustFragment> {
    let mut generated = GeneratedRustFragment::default();
    let cpp_class_name_rust = &qobject_idents.name.rust_unqualified();

    for &invokable in invokables {
        let idents = QMethodName::try_from(invokable)?;
        let wrapper_ident_cpp = idents.wrapper.cxx_unqualified();
        let invokable_ident_rust = &idents.name.rust_unqualified();

        // TODO: once we aren't using qobject::T in the extern "RustQt"
        // we can just pass through the original ExternFn block and add the attribute?

        let parameter_signatures = get_params_tokens(
            invokable.mutable,
            &invokable.parameters,
            cpp_class_name_rust,
        );

        let return_type = &invokable.method.sig.output;

        let mut unsafe_call = Some(quote! { unsafe });
        if invokable.safe {
            std::mem::swap(&mut unsafe_call, &mut None);
        }

        let doc_comments = &invokable.docs;

        let fragment = RustFragmentPair {
            cxx_bridge: vec![quote_spanned! {
                invokable.method.span() =>
                // Note: extern "Rust" block does not need to be unsafe
                extern "Rust" {
                    // Note that we are exposing a Rust method on the C++ type to C++
                    //
                    // CXX ends up generating the source, then we generate the matching header.
                    #(#doc_comments)*
                    #[cxx_name = #wrapper_ident_cpp]
                    // TODO: Add #[namespace] of the QObject
                    #unsafe_call fn #invokable_ident_rust(#parameter_signatures) #return_type;
                }
            }],
            implementation: vec![],
        };

        generated
            .cxx_mod_contents
            .append(&mut fragment.cxx_bridge_as_items()?);
        generated
            .cxx_qt_mod_contents
            .append(&mut fragment.implementation_as_items()?);
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::generator::naming::qobject::tests::create_qobjectname;
    use crate::naming::Name;
    use crate::parser::parameter::ParsedFunctionParameter;
    use crate::tests::assert_tokens_eq;
    use quote::format_ident;
    use std::collections::HashSet;
    use syn::{parse_quote, ForeignItemFn};

    #[test]
    fn test_generate_rust_invokables() {
        let method1: ForeignItemFn = parse_quote! { fn void_invokable(self: &MyObject); };
        let method2: ForeignItemFn =
            parse_quote! { fn trivial_invokable(self: &MyObject, param: i32) -> i32; };
        let method3: ForeignItemFn = parse_quote! { fn opaque_invokable(self: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor>; };
        let method4: ForeignItemFn =
            parse_quote! { unsafe fn unsafe_invokable(self: &MyObject, param: *mut T) -> *mut T; };
        let invokables = vec![
            ParsedMethod {
                method: method1.clone(),
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: true,
                parameters: vec![],
                specifiers: HashSet::new(),
                is_qinvokable: true,
                name: Name::from_rust_ident_and_attrs(
                    &method1.sig.ident,
                    &method1.attrs,
                    None,
                    None,
                )
                .unwrap(),
                docs: vec![],
            },
            ParsedMethod {
                method: method2.clone(),
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: true,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { i32 },
                }],
                specifiers: HashSet::new(),
                is_qinvokable: true,
                name: Name::from_rust_ident_and_attrs(
                    &method2.sig.ident,
                    &method2.attrs,
                    None,
                    None,
                )
                .unwrap(),
                docs: vec![],
            },
            ParsedMethod {
                method: method3.clone(),
                qobject_ident: format_ident!("MyObject"),
                mutable: true,
                safe: true,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { &QColor },
                }],
                specifiers: HashSet::new(),
                is_qinvokable: true,
                name: Name::from_rust_ident_and_attrs(
                    &method3.sig.ident,
                    &method3.attrs,
                    None,
                    None,
                )
                .unwrap(),
                docs: vec![],
            },
            ParsedMethod {
                method: method4.clone(),
                qobject_ident: format_ident!("MyObject"),
                mutable: false,
                safe: false,
                parameters: vec![ParsedFunctionParameter {
                    ident: format_ident!("param"),
                    ty: parse_quote! { *mut T },
                }],
                specifiers: HashSet::new(),
                is_qinvokable: true,
                name: Name::from_rust_ident_and_attrs(
                    &method4.sig.ident,
                    &method4.attrs,
                    None,
                    None,
                )
                .unwrap(),
                docs: vec![],
            },
        ];
        let qobject_idents = create_qobjectname();

        let generated =
            generate_rust_methods(&invokables.iter().collect(), &qobject_idents).unwrap();

        assert_eq!(generated.cxx_mod_contents.len(), 4);
        assert_eq!(generated.cxx_qt_mod_contents.len(), 0);

        // void_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[0],
            quote! {
                extern "Rust" {
                    #[cxx_name = "voidInvokableWrapper"]
                    fn void_invokable(self: &MyObject);
                }
            },
        );

        // trivial_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[1],
            quote! {
                extern "Rust" {
                    #[cxx_name = "trivialInvokableWrapper"]
                    fn trivial_invokable(self: &MyObject, param: i32) -> i32;
                }
            },
        );

        // opaque_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[2],
            quote! {
                extern "Rust" {
                    #[cxx_name = "opaqueInvokableWrapper"]
                    fn opaque_invokable(self: Pin<&mut MyObject>, param: &QColor) -> UniquePtr<QColor>;
                }
            },
        );

        // unsafe_invokable
        assert_tokens_eq(
            &generated.cxx_mod_contents[3],
            quote! {
                extern "Rust" {
                    #[cxx_name = "unsafeInvokableWrapper"]
                    unsafe fn unsafe_invokable(self:&MyObject, param: *mut T) -> *mut T;
                }
            },
        );
    }
}
