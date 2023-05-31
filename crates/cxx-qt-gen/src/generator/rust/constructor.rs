// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{namespace::NamespaceName, qobject::QObjectName, CombinedIdent},
        rust::{qobject::GeneratedRustQObjectBlocks, types},
    },
    parser::constructor::Constructor,
};

use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_quote, parse_quote_spanned, spanned::Spanned, Expr, Ident, Item, Result, Type};

const CONSTRUCTOR_ARGUMENTS: &str = "CxxQtConstructorArguments";
const BASE_ARGUMENTS: &str = "CxxQtConstructorBaseArguments";
const NEW_ARGUMENTS: &str = "CxxQtConstructorNewArguments";
const INITIALIZE_ARGUMENTS: &str = "CxxQtConstructorInitializeArguments";

fn map_types<F: FnMut((usize, &Type)) -> TokenStream>(
    args: &Option<Vec<Type>>,
    f: F,
) -> Vec<TokenStream> {
    args.as_ref()
        .map(|args| args.iter().enumerate().map(f).collect())
        .unwrap_or_default()
}

fn extract_arguments_from_tuple(args: &Option<Vec<Type>>, tuple_name: Ident) -> Vec<TokenStream> {
    map_types(args, |(index, _ty)| {
        let arg_name = format_ident!("arg{index}");
        let index = syn::LitInt::new(index.to_string().as_str(), Span::call_site());
        quote! {
            #arg_name: #tuple_name.#index
        }
    })
}

fn extract_arguments_from_struct(args: &Option<Vec<Type>>, struct_name: Ident) -> Vec<TokenStream> {
    map_types(args, |(index, _ty)| {
        let arg_name = format_ident!("arg{index}");
        quote! {
            #struct_name.#arg_name
        }
    })
}

fn argument_members(args: &Option<Vec<Type>>) -> Vec<TokenStream> {
    map_types(args, |(index, ty)| {
        let arg_name = format_ident!("arg{index}");
        quote! {
            #arg_name: #ty
        }
    })
}

fn generate_default_constructor(
    qobject_idents: &QObjectName,
    namespace: &NamespaceName,
) -> GeneratedRustQObjectBlocks {
    let rust_struct_ident = &qobject_idents.rust_struct.rust;
    let create_rs_ident = format_ident!(
        "create_rs_{object_name}",
        object_name = rust_struct_ident.to_string().to_case(Case::Snake)
    );
    let namespace_internals = &namespace.internal;

    GeneratedRustQObjectBlocks {
        cxx_mod_contents: vec![parse_quote! {
        extern "Rust" {
            #[cxx_name = "createRs"]
            #[namespace = #namespace_internals]
            fn #create_rs_ident() -> Box<#rust_struct_ident>;
        }
        }],
        cxx_qt_mod_contents: vec![parse_quote! {
            /// Generated CXX-Qt method which creates a boxed rust struct of a QObject
            pub fn #create_rs_ident() -> std::boxed::Box<#rust_struct_ident> {
                core::default::Default::default()
            }
        }],
    }
}

fn generate_arguments_struct(
    namespace_internals: &str,
    struct_name: &CombinedIdent,
    argument_list: &Option<Vec<Type>>,
) -> Item {
    let argument_members = argument_members(argument_list);
    let not_empty = if argument_list.as_ref().is_some_and(|list| !list.is_empty()) {
        None
    } else {
        Some(quote! { not_empty: i8 })
    };
    let rust_name = &struct_name.rust;
    // use to_string here, as the cxx_name needs to be in quotes for the attribute macro.
    let cxx_name = &struct_name.cpp.to_string();
    parse_quote! {
        #[namespace = #namespace_internals]
        #[cxx_name = #cxx_name]
        #[doc(hidden)]
        struct #rust_name {
            #(#argument_members,)*
            #not_empty // Make sure there's always at least one struct member, as CXX
                       // doesn't support empty shared structs.
        }
    }
}

fn generate_arguments_initialization(
    struct_name: &Ident,
    instance_name: Ident,
    argument_list: &Option<Vec<Type>>,
) -> Expr {
    let init_arguments = extract_arguments_from_tuple(argument_list, instance_name);
    println!("init_arguments: {:?}", init_arguments);
    let not_empty = if argument_list.as_ref().is_some_and(|list| !list.is_empty()) {
        None
    } else {
        Some(quote! { not_empty: 0 })
    };

    parse_quote! {
        #struct_name {
            #(#init_arguments,)*
            #not_empty
        }
    }
}

pub fn generate(
    constructors: &[Constructor],
    qobject_idents: &QObjectName,
    namespace: &NamespaceName,
) -> Result<GeneratedRustQObjectBlocks> {
    if constructors.is_empty() {
        return Ok(generate_default_constructor(qobject_idents, namespace));
    }

    let mut result = GeneratedRustQObjectBlocks::default();
    let namespace_internals = &namespace.internal;
    let rust_struct_name_rust = &qobject_idents.rust_struct.rust;
    let rust_struct_name_snake = &qobject_idents
        .rust_struct
        .rust
        .to_string()
        .to_case(Case::Snake);
    let qobject_name_rust = &qobject_idents.cpp_class.rust;

    for (index, constructor) in constructors.iter().enumerate() {
        let arguments_rust = format_ident!("{CONSTRUCTOR_ARGUMENTS}{rust_struct_name_rust}{index}");
        let base_arguments_rust = format_ident!("{BASE_ARGUMENTS}{rust_struct_name_rust}{index}");
        let new_arguments_rust = format_ident!("{NEW_ARGUMENTS}{rust_struct_name_rust}{index}");
        let initialize_arguments_rust =
            format_ident!("{INITIALIZE_ARGUMENTS}{rust_struct_name_rust}{index}");

        let arguments_cxx = format!("{CONSTRUCTOR_ARGUMENTS}{index}");
        let base_arguments_cxx = format_ident!("{BASE_ARGUMENTS}{index}");
        let new_arguments_cxx = format_ident!("{NEW_ARGUMENTS}{index}");
        let initialize_arguments_cxx = format_ident!("{INITIALIZE_ARGUMENTS}{index}");

        let new_rust = format_ident!("new_rs_{rust_struct_name_snake}_{index}");
        let new_cxx = format!("newRs{index}");

        let initialize_rust = format_ident!("initialize_{rust_struct_name_snake}_{index}");
        let initialize_cxx = format!("initialize{index}");

        let route_arguments_rust =
            format_ident!("route_arguments_{rust_struct_name_snake}_{index}");
        let route_arguemnts_cxx = format!("routeArguments{index}");

        let argument_types = &constructor.arguments;

        let route_arguments_parameters: Vec<TokenStream> = constructor
            .arguments
            .iter()
            .enumerate()
            .map(|(index, ty)| {
                let name = format_ident!("arg{index}");
                quote! { #name: #ty }
            })
            .collect();
        let route_arguments_safety = if constructor.arguments.iter().any(types::is_unsafe_cxx_type)
        {
            quote! { unsafe }
        } else {
            quote! {}
        };

        let assign_arguments = constructor
            .arguments
            .iter()
            .enumerate()
            .map(|(index, _ty)| {
                let name = format_ident!("arg{index}");
                quote! { #name }
            })
            .collect::<Vec<_>>();

        let init_new_arguments = generate_arguments_initialization(
            &new_arguments_rust,
            format_ident!("new_arguments"),
            &constructor.new_arguments,
        );
        let init_initalize_arguments = generate_arguments_initialization(
            &initialize_arguments_rust,
            format_ident!("initialize_arguments"),
            &constructor.initialize_arguments,
        );
        let init_base_arguments = generate_arguments_initialization(
            &base_arguments_rust,
            format_ident!("base_arguments"),
            &constructor.base_arguments,
        );

        let extract_new_arguments = extract_arguments_from_struct(
            &constructor.new_arguments,
            format_ident!("new_arguments"),
        );

        let extract_initialize_arguments = extract_arguments_from_struct(
            &constructor.initialize_arguments,
            format_ident!("initialize_arguments"),
        );

        result.cxx_mod_contents.append(&mut vec![
            parse_quote! {
                #[namespace = #namespace_internals]
                #[cxx_name = #arguments_cxx]
                #[doc(hidden)]
                struct #arguments_rust {
                    baseArguments: #base_arguments_rust,
                    newArguments: #new_arguments_rust,
                    initializeArguments: #initialize_arguments_rust,
                }
            },
            generate_arguments_struct(&namespace.internal, &CombinedIdent {
                cpp: base_arguments_cxx.clone(),
                rust: base_arguments_rust.clone(),
            }, &constructor.base_arguments),
            generate_arguments_struct(&namespace.internal, &CombinedIdent {
                cpp: new_arguments_cxx.clone(),
                rust: new_arguments_rust.clone(),
            }, &constructor.new_arguments),
            generate_arguments_struct(&namespace.internal, &CombinedIdent {
                cpp: initialize_arguments_cxx.clone(),
                rust: initialize_arguments_rust.clone(),
            }, &constructor.initialize_arguments),
            parse_quote! {
                extern "Rust" {
                    #[namespace = #namespace_internals]
                    #[cxx_name = #route_arguemnts_cxx]
                    // This function needs to marked unsafe, as some arguments may be pointers.
                    #route_arguments_safety fn #route_arguments_rust(#(#route_arguments_parameters),*) -> #arguments_rust;

                    #[namespace = #namespace_internals]
                    #[cxx_name = #new_cxx]
                    fn #new_rust(args: #new_arguments_rust) -> Box<#rust_struct_name_rust>;

                    #[namespace = #namespace_internals]
                    #[cxx_name = #initialize_cxx]
                    fn #initialize_rust(qobject: Pin<&mut #qobject_name_rust>, args: #initialize_arguments_rust);
                }
            },
        ]);
        result.cxx_qt_mod_contents.append(&mut vec![parse_quote_spanned! {
            constructor.imp.span() =>
            #[doc(hidden)]
            pub fn #route_arguments_rust(#(#route_arguments_parameters),*) -> #arguments_rust {
                // These variables won't be used if the corresponding argument list is empty.
                #[allow(unused_variables)]
                #[allow(clippy::let_unit_value)]
                let (
                    new_arguments,
                    base_arguments,
                    initialize_arguments
                    ) = <#qobject_name_rust as cxx_qt::Constructor<(#(#argument_types,)*)>>
                                    ::route_arguments((#(#assign_arguments,)*));
                #arguments_rust {
                    baseArguments: #init_base_arguments,
                    initializeArguments: #init_initalize_arguments,
                    newArguments: #init_new_arguments,
                }
            }
        },
        parse_quote_spanned! {
            constructor.imp.span() =>
            #[doc(hidden)]
            #[allow(unused_variables)]
            pub fn #new_rust(new_arguments: #new_arguments_rust) -> std::boxed::Box<#rust_struct_name_rust> {
                #[allow(clippy::let_unit_value)]
                let new_arguments = (#(#extract_new_arguments,)*);
                std::boxed::Box::new(<#qobject_name_rust as cxx_qt::Constructor<(#(#argument_types,)*)>>::new(new_arguments))
            }
        },
        parse_quote_spanned! {
            constructor.imp.span() =>
            #[doc(hidden)]
            #[allow(unused_variables)]
            pub fn #initialize_rust(
                qobject: core::pin::Pin<&mut #qobject_name_rust>,
                initialize_arguments: #initialize_arguments_rust
            ) {
                #[allow(clippy::let_unit_value)]
                let initialize_arguments = (#(#extract_initialize_arguments,)*);
                <#qobject_name_rust as cxx_qt::Constructor<(#(#argument_types,)*)>>::initialize(qobject, initialize_arguments)
            }
        }])
    }
    Ok(result)
}
