// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::BTreeMap;

use crate::{
    generator::{
        naming::{namespace::NamespaceName, qobject::QObjectName, CombinedIdent},
        rust::fragment::GeneratedRustFragment,
        utils::rust::{
            syn_ident_cxx_bridge_to_qualified_impl, syn_type_cxx_bridge_to_qualified,
            syn_type_is_cxx_bridge_unsafe,
        },
    },
    parser::constructor::Constructor,
    syntax::lifetimes,
};

use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_quote, parse_quote_spanned, spanned::Spanned, Error, Expr, FnArg, Ident, Item, Lifetime,
    Path, Result, Type,
};

const CONSTRUCTOR_ARGUMENTS: &str = "CxxQtConstructorArguments";
const BASE_ARGUMENTS: &str = "CxxQtConstructorBaseArguments";
const NEW_ARGUMENTS: &str = "CxxQtConstructorNewArguments";
const INITIALIZE_ARGUMENTS: &str = "CxxQtConstructorInitializeArguments";

fn map_types<F: FnMut((usize, &Type)) -> TokenStream>(args: &[Type], f: F) -> Vec<TokenStream> {
    args.iter().enumerate().map(f).collect()
}

fn extract_arguments_from_tuple(args: &[Type], tuple_name: Ident) -> Vec<TokenStream> {
    map_types(args, |(index, _ty)| {
        let arg_name = format_ident!("arg{index}");
        let index = syn::LitInt::new(index.to_string().as_str(), Span::call_site());
        quote! {
            #arg_name: #tuple_name.#index
        }
    })
}

fn extract_arguments_from_struct(args: &[Type], struct_name: Ident) -> Vec<TokenStream> {
    map_types(args, |(index, _ty)| {
        let arg_name = format_ident!("arg{index}");
        quote! {
            #struct_name.#arg_name
        }
    })
}

fn argument_members(args: &[Type]) -> Vec<TokenStream> {
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
) -> GeneratedRustFragment {
    let rust_struct_ident = &qobject_idents.rust_struct.rust;

    let create_rs_ident = format_ident!(
        "create_rs_{object_name}",
        object_name = rust_struct_ident.to_string().to_case(Case::Snake)
    );
    let namespace_internals = &namespace.internal;

    GeneratedRustFragment {
        cxx_mod_contents: vec![parse_quote! {
        extern "Rust" {
            #[cxx_name = "createRs"]
            #[namespace = #namespace_internals]
            fn #create_rs_ident() -> Box<#rust_struct_ident>;
        }
        }],
        cxx_qt_mod_contents: vec![parse_quote! {
            #[doc(hidden)]
            pub fn #create_rs_ident() -> std::boxed::Box<#rust_struct_ident> {
                // Wrapping the call to Default::default in a Box::new call leads
                // to a nicer error message, as it's not trying to infer trait bounds
                // on Box, but directly on the given type.
                std::boxed::Box::new(core::default::Default::default())
            }
        }],
    }
}

fn generate_arguments_struct(
    namespace_internals: &str,
    struct_name: &CombinedIdent,
    lifetime: &Option<TokenStream>,
    argument_list: &[Type],
) -> Item {
    let argument_members = argument_members(argument_list);
    let not_empty = if argument_list.is_empty() {
        Some(quote! { not_empty: i8 })
    } else {
        None
    };
    let rust_name = &struct_name.rust;
    // use to_string here, as the cxx_name needs to be in quotes for the attribute macro.
    let cxx_name = &struct_name.cpp.to_string();
    parse_quote! {
        #[namespace = #namespace_internals]
        #[cxx_name = #cxx_name]
        #[doc(hidden)]
        struct #rust_name #lifetime {
            #(#argument_members,)*
            #not_empty // Make sure there's always at least one struct member, as CXX
                       // doesn't support empty shared structs.
        }
    }
}

fn generate_arguments_initialization(
    struct_name: &Ident,
    instance_name: Ident,
    argument_list: &[Type],
) -> Expr {
    let init_arguments = extract_arguments_from_tuple(argument_list, instance_name);
    let not_empty = if argument_list.is_empty() {
        Some(quote! { not_empty: 0 })
    } else {
        None
    };

    parse_quote! {
        #struct_name {
            #(#init_arguments,)*
            #not_empty
        }
    }
}

fn lifetime_of_arguments(
    lifetime: &Option<Lifetime>,
    argument_list: &[Type],
) -> Result<Option<TokenStream>> {
    if let Some(lifetime) = lifetime.as_ref() {
        let argument_lifetimes: Vec<Lifetime> = argument_list
            .iter()
            .map(lifetimes::from_type)
            .collect::<Result<Vec<Vec<Lifetime>>>>()?
            .into_iter()
            .flatten()
            .collect();

        if argument_lifetimes.is_empty() {
            Ok(None)
        } else {
            // Any of the argument lifetimes may be different from the lifetime passed to the constructor.
            // However, then the compiler will just generate an appropriate error, as the lifetime has the wrong name.
            Ok(Some(quote! {
                < #lifetime >
            }))
        }
    } else {
        // If there's a lifetime in the arguments, we can just let the compiler error on us.
        // The error message will be reasonably nice.
        Ok(None)
    }
}

fn unsafe_if(condition: bool) -> Option<TokenStream> {
    if condition {
        Some(quote! { unsafe })
    } else {
        None
    }
}

pub fn generate(
    constructors: &[Constructor],
    qobject_idents: &QObjectName,
    namespace: &NamespaceName,
    qualified_mappings: &BTreeMap<Ident, Path>,
    module_ident: &Ident,
) -> Result<GeneratedRustFragment> {
    if constructors.is_empty() {
        return Ok(generate_default_constructor(qobject_idents, namespace));
    }

    let mut result = GeneratedRustFragment::default();
    let namespace_internals = &namespace.internal;

    let qobject_name = &qobject_idents.cpp_class.cpp;
    let qobject_name_rust = &qobject_idents.cpp_class.rust;
    let qobject_name_rust_qualified =
        syn_ident_cxx_bridge_to_qualified_impl(qobject_name_rust, qualified_mappings);
    let qobject_name_snake = qobject_name.to_string().to_case(Case::Snake);

    let rust_struct_name_rust = &qobject_idents.rust_struct.rust;

    for (index, constructor) in constructors.iter().enumerate() {
        let lifetime = constructor.lifetime.as_ref().map(|lifetime| {
            quote! {
                < #lifetime >
            }
        });
        let arguments_lifetime =
            lifetime_of_arguments(&constructor.lifetime, &constructor.arguments)?;
        let base_lifetime =
            lifetime_of_arguments(&constructor.lifetime, &constructor.base_arguments)?;
        let new_lifetime =
            lifetime_of_arguments(&constructor.lifetime, &constructor.new_arguments)?;
        let initialize_lifetime =
            lifetime_of_arguments(&constructor.lifetime, &constructor.initialize_arguments)?;
        let args_tuple_lifetime = base_lifetime
            .clone()
            .or(new_lifetime.clone())
            .or(initialize_lifetime.clone());

        // If there is a lifetime declared, but it's not used, The compiler will catch this,
        // but the error message is so obscure, that we should catch it here with a better error
        // message.
        if lifetime.is_some()
            && [
                &arguments_lifetime,
                &base_lifetime,
                &new_lifetime,
                &initialize_lifetime,
            ]
            .into_iter()
            .all(Option::is_none)
        {
            return Err(Error::new_spanned(
                &constructor.lifetime,
                "this lifetime isn't used in the Constructor declaration!",
            ));
        }

        let args_tuple_rust = format_ident!("{CONSTRUCTOR_ARGUMENTS}{qobject_name}{index}");
        let base_arguments_rust = format_ident!("{BASE_ARGUMENTS}{qobject_name}{index}");
        let new_arguments_rust = format_ident!("{NEW_ARGUMENTS}{qobject_name}{index}");
        let initialize_arguments_rust =
            format_ident!("{INITIALIZE_ARGUMENTS}{qobject_name}{index}");

        let args_tuple_cxx = format!("{CONSTRUCTOR_ARGUMENTS}{index}");
        let base_arguments_cxx = format_ident!("{BASE_ARGUMENTS}{index}");
        let new_arguments_cxx = format_ident!("{NEW_ARGUMENTS}{index}");
        let initialize_arguments_cxx = format_ident!("{INITIALIZE_ARGUMENTS}{index}");

        let new_rust = format_ident!("new_rs_{qobject_name_snake}_{index}");
        let new_cxx = format!("newRs{index}");

        let initialize_rust = format_ident!("initialize_{qobject_name_snake}_{index}");
        let initialize_cxx = format!("initialize{index}");

        let route_arguments_rust = format_ident!("route_arguments_{qobject_name_snake}_{index}");
        let route_arguments_cxx = format!("routeArguments{index}");

        let argument_types_qualified: Vec<Type> = constructor
            .arguments
            .iter()
            .map(|arg| syn_type_cxx_bridge_to_qualified(arg, qualified_mappings))
            .collect();

        let route_arguments_parameters: Vec<FnArg> = constructor
            .arguments
            .iter()
            .enumerate()
            .map(|(index, ty)| {
                let name = format_ident!("arg{index}");
                parse_quote! { #name: #ty }
            })
            .collect();
        let route_arguments_parameter_qualified: Vec<FnArg> = route_arguments_parameters
            .iter()
            .cloned()
            .map(|mut parameter| {
                if let FnArg::Typed(pat_type) = &mut parameter {
                    *pat_type.ty =
                        syn_type_cxx_bridge_to_qualified(&pat_type.ty, qualified_mappings);
                }
                parameter
            })
            .collect();

        // As the function implementations cast to `Constructor<(Args)>`, these `Args` may
        // include the lifetime.
        let new_function_lifetime = new_lifetime.clone().or(arguments_lifetime.clone());
        let initialize_function_lifetime =
            initialize_lifetime.clone().or(arguments_lifetime.clone());
        let route_function_decl_lifetime =
            arguments_lifetime.clone().or(args_tuple_lifetime.clone());

        let route_arguments_safety = unsafe_if(
            lifetime.is_some()
                || constructor
                    .arguments
                    .iter()
                    .any(syn_type_is_cxx_bridge_unsafe),
        );
        let new_safety = unsafe_if(new_function_lifetime.is_some());
        let initialize_safety = unsafe_if(initialize_function_lifetime.is_some());

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
            parse_quote_spanned! {
                constructor.imp.span() =>
                #[namespace = #namespace_internals]
                #[cxx_name = #args_tuple_cxx]
                #[doc(hidden)]
                struct #args_tuple_rust #args_tuple_lifetime {
                    base: #base_arguments_rust #base_lifetime,
                    // new a keyword in C++, so we use `new_` here
                    #[cxx_name = "new_"]
                    new: #new_arguments_rust #new_lifetime,
                    initialize: #initialize_arguments_rust #initialize_lifetime,
                }
            },
            generate_arguments_struct(&namespace.internal, &CombinedIdent {
                cpp: base_arguments_cxx.clone(),
                rust: base_arguments_rust.clone(),
            }, &base_lifetime, &constructor.base_arguments),
            generate_arguments_struct(&namespace.internal, &CombinedIdent {
                cpp: new_arguments_cxx.clone(),
                rust: new_arguments_rust.clone(),
            }, &new_lifetime, &constructor.new_arguments),
            generate_arguments_struct(&namespace.internal, &CombinedIdent {
                cpp: initialize_arguments_cxx.clone(),
                rust: initialize_arguments_rust.clone(),
            }, &initialize_lifetime, &constructor.initialize_arguments),
            parse_quote_spanned! {
                constructor.imp.span() =>
                extern "Rust" {
                    #[namespace = #namespace_internals]
                    #[cxx_name = #route_arguments_cxx]
                    // This function may need to be marked unsafe, as some arguments may be pointers.
                    #route_arguments_safety fn #route_arguments_rust #route_function_decl_lifetime (#(#route_arguments_parameters),*) -> #args_tuple_rust #args_tuple_lifetime;

                    #[namespace = #namespace_internals]
                    #[cxx_name = #new_cxx]
                    #new_safety fn #new_rust #new_lifetime (args: #new_arguments_rust #new_lifetime) -> Box<#rust_struct_name_rust>;

                    #[namespace = #namespace_internals]
                    #[cxx_name = #initialize_cxx]
                    #initialize_safety fn #initialize_rust #initialize_lifetime (qobject: Pin<&mut #qobject_name_rust>, args: #initialize_arguments_rust #initialize_lifetime);
                }
            },
        ]);
        result.cxx_qt_mod_contents.append(&mut vec![parse_quote_spanned! {
            constructor.imp.span() =>
            #[doc(hidden)]
            // Use the catch-all lifetime here, as if a lifetime argument is specified, it should
            // be used in either the argument list itself, or the returned, routed arguments.
            // So it must be used by this function somewhere.
            pub fn #route_arguments_rust #lifetime(#(#route_arguments_parameter_qualified),*) -> #module_ident::#args_tuple_rust #args_tuple_lifetime {
                // These variables won't be used if the corresponding argument list is empty.
                #[allow(unused_variables)]
                #[allow(clippy::let_unit_value)]
                let (
                    new_arguments,
                    base_arguments,
                    initialize_arguments
                    ) = <#qobject_name_rust_qualified as cxx_qt::Constructor<(#(#argument_types_qualified,)*)>>
                                    ::route_arguments((#(#assign_arguments,)*));
                #module_ident::#args_tuple_rust {
                    base: #module_ident::#init_base_arguments,
                    initialize: #module_ident::#init_initalize_arguments,
                    new: #module_ident::#init_new_arguments,
                }
            }
        },
        parse_quote_spanned! {
            constructor.imp.span() =>
            #[doc(hidden)]
            #[allow(unused_variables)]
            #[allow(clippy::extra_unused_lifetimes)]
            // If we use the lifetime here for casting to the specific Constructor type, then
            // clippy for some reason thinks that the lifetime is unused even though it is used
            // by the `as` expression.
            // So let's just allow unused extra lifetimes for this function.
            pub fn #new_rust  #new_function_lifetime(new_arguments: #module_ident::#new_arguments_rust #new_lifetime) -> std::boxed::Box<#rust_struct_name_rust> {
                std::boxed::Box::new(<#qobject_name_rust_qualified as cxx_qt::Constructor<(#(#argument_types_qualified,)*)>>::new((#(#extract_new_arguments,)*)))
            }
        },
        parse_quote_spanned! {
            constructor.imp.span() =>
            #[doc(hidden)]
            #[allow(unused_variables)]
            #[allow(clippy::extra_unused_lifetimes)]
            // If we use the lifetime here for casting to the specific Constructor type, then
            // clippy for some reason thinks that the lifetime is unused even though it is used
            // by the `as` expression.
            // So let's just allow unused extra lifetimes for this function.
            pub fn #initialize_rust #initialize_function_lifetime(
                qobject: core::pin::Pin<&mut #qobject_name_rust_qualified>,
                initialize_arguments: #module_ident::#initialize_arguments_rust  #initialize_lifetime
            ) {
                <#qobject_name_rust_qualified as cxx_qt::Constructor<(#(#argument_types_qualified,)*)>>::initialize(
                    qobject,
                    (#(#extract_initialize_arguments,)*));
            }
        }])
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_tokens_eq;

    use quote::format_ident;

    fn mock_constructor() -> Constructor {
        Constructor {
            new_arguments: vec![],
            base_arguments: vec![],
            initialize_arguments: vec![],
            arguments: vec![],
            lifetime: None,
            // dummy impl for testing
            imp: parse_quote! {impl X {}},
        }
    }

    fn mock_name() -> QObjectName {
        QObjectName::from_idents(format_ident!("MyObject"), format_ident!("MyObjectRust"))
    }

    fn mock_namespace() -> NamespaceName {
        NamespaceName::from_pair_str("ffi", &format_ident!("MyObject"))
    }

    fn generate_mocked(constructors: &[Constructor]) -> GeneratedRustFragment {
        generate(
            constructors,
            &mock_name(),
            &mock_namespace(),
            &BTreeMap::<Ident, Path>::default(),
            &format_ident!("ffi"),
        )
        .unwrap()
    }

    #[test]
    fn default_constructor() {
        let blocks = generate_mocked(&[]);

        assert_eq!(blocks.cxx_mod_contents.len(), 1);
        assert_eq!(blocks.cxx_qt_mod_contents.len(), 1);

        assert_tokens_eq(
            &blocks.cxx_mod_contents[0],
            quote! {
                extern "Rust" {
                    #[cxx_name="createRs"]
                    #[namespace="ffi::cxx_qt_my_object"]
                    fn create_rs_my_object_rust() -> Box<MyObjectRust>;
                }
            },
        );
        assert_tokens_eq(
            &blocks.cxx_qt_mod_contents[0],
            quote! {
                #[doc(hidden)]
                pub fn create_rs_my_object_rust() -> std::boxed::Box<MyObjectRust>
                {
                    std::boxed::Box::new(core::default::Default::default())
                }
            },
        );
    }

    fn assert_empty_argument_struct<T: quote::ToTokens>(
        tokens: &T,
        rust_name: &str,
        cxx_name: &str,
    ) {
        let rust_name = format_ident!("{rust_name}");
        assert_tokens_eq(
            tokens,
            quote! {
                #[namespace = "ffi::cxx_qt_my_object"]
                #[cxx_name = #cxx_name]
                #[doc(hidden)]
                struct #rust_name {
                    not_empty: i8
                }
            },
        );
    }

    // This is called by the `multiple_constructors` test so we don't have to
    // assert this in two separate tests.
    fn assert_empty_constructor_blocks(
        blocks: &GeneratedRustFragment,
        namespace_attr: &TokenStream,
    ) {
        assert_tokens_eq(
            &blocks.cxx_mod_contents[0],
            quote! {
                #namespace_attr
                #[cxx_name = "CxxQtConstructorArguments0"]
                #[doc(hidden)]
                struct CxxQtConstructorArgumentsMyObject0 {
                    base: CxxQtConstructorBaseArgumentsMyObject0,
                    #[cxx_name="new_"]
                    new: CxxQtConstructorNewArgumentsMyObject0,
                    initialize : CxxQtConstructorInitializeArgumentsMyObject0,
                }
            },
        );

        assert_empty_argument_struct(
            &blocks.cxx_mod_contents[1],
            "CxxQtConstructorBaseArgumentsMyObject0",
            "CxxQtConstructorBaseArguments0",
        );
        assert_empty_argument_struct(
            &blocks.cxx_mod_contents[2],
            "CxxQtConstructorNewArgumentsMyObject0",
            "CxxQtConstructorNewArguments0",
        );
        assert_empty_argument_struct(
            &blocks.cxx_mod_contents[3],
            "CxxQtConstructorInitializeArgumentsMyObject0",
            "CxxQtConstructorInitializeArguments0",
        );
        assert_tokens_eq(
            &blocks.cxx_mod_contents[4],
            quote! {
                extern "Rust" {
                    #namespace_attr
                    #[cxx_name = "routeArguments0"]
                    fn route_arguments_my_object_0() -> CxxQtConstructorArgumentsMyObject0;

                    #namespace_attr
                    #[cxx_name = "newRs0"]
                    fn new_rs_my_object_0(args: CxxQtConstructorNewArgumentsMyObject0) -> Box<MyObjectRust>;

                    #namespace_attr
                    #[cxx_name = "initialize0"]
                    fn initialize_my_object_0(qobject: Pin<&mut MyObject>, args: CxxQtConstructorInitializeArgumentsMyObject0);
                }
            },
        );

        assert_tokens_eq(
            &blocks.cxx_qt_mod_contents[0],
            quote! {
                #[doc(hidden)]
                pub fn route_arguments_my_object_0() -> ffi::CxxQtConstructorArgumentsMyObject0
                {
                    #[allow(unused_variables)]
                    #[allow(clippy::let_unit_value)]
                    let (new_arguments, base_arguments, initialize_arguments) =
                        <MyObject as cxx_qt::Constructor<()> >::route_arguments(());

                    ffi::CxxQtConstructorArgumentsMyObject0 {
                        base: ffi::CxxQtConstructorBaseArgumentsMyObject0 { not_empty: 0 },
                        initialize: ffi::CxxQtConstructorInitializeArgumentsMyObject0 { not_empty: 0 },
                        new: ffi::CxxQtConstructorNewArgumentsMyObject0 { not_empty : 0 },
                    }
                }
            },
        );
        assert_tokens_eq(
            &blocks.cxx_qt_mod_contents[1],
            quote! {
                #[doc(hidden)]
                #[allow(unused_variables)]
                #[allow(clippy::extra_unused_lifetimes)]
                pub fn new_rs_my_object_0(new_arguments: ffi::CxxQtConstructorNewArgumentsMyObject0) -> std::boxed::Box<MyObjectRust> {
                    std::boxed::Box::new(
                        <MyObject as cxx_qt::Constructor<()> >::new(())
                    )
                }
            },
        );
        assert_tokens_eq(
            &blocks.cxx_qt_mod_contents[2],
            quote! {
                #[doc(hidden)]
                #[allow(unused_variables)]
                #[allow(clippy::extra_unused_lifetimes)]
                pub fn initialize_my_object_0(
                    qobject: core::pin::Pin<&mut MyObject>,
                    initialize_arguments: ffi::CxxQtConstructorInitializeArgumentsMyObject0)
                {
                    <MyObject as cxx_qt::Constructor<()> >::initialize(qobject, ());
                }
            },
        );
    }

    fn assert_full_constructor_blocks(
        blocks: &GeneratedRustFragment,
        namespace_attr: &TokenStream,
    ) {
        // the index here starts with 5, as this is part of the larger multiple_constructors test.
        assert_tokens_eq(
            &blocks.cxx_mod_contents[5],
            quote! {
                #namespace_attr
                #[cxx_name = "CxxQtConstructorArguments1"]
                #[doc(hidden)]
                struct CxxQtConstructorArgumentsMyObject1<'lifetime> {
                    base: CxxQtConstructorBaseArgumentsMyObject1,
                    #[cxx_name="new_"]
                    new: CxxQtConstructorNewArgumentsMyObject1,
                    initialize : CxxQtConstructorInitializeArgumentsMyObject1<'lifetime>,
                }
            },
        );

        assert_tokens_eq(
            &blocks.cxx_mod_contents[6],
            quote! {
                #namespace_attr
                #[cxx_name="CxxQtConstructorBaseArguments1"]
                #[doc(hidden)]
                struct CxxQtConstructorBaseArgumentsMyObject1 {
                    arg0: i64,
                    arg1: *mut QObject,
                    arg2: f32,
                }
            },
        );
        assert_tokens_eq(
            &blocks.cxx_mod_contents[7],
            quote! {
                #namespace_attr
                #[cxx_name="CxxQtConstructorNewArguments1"]
                #[doc(hidden)]
                struct CxxQtConstructorNewArgumentsMyObject1 {
                    arg0: i16,
                }
            },
        );
        assert_tokens_eq(
            &blocks.cxx_mod_contents[8],
            quote! {
                #namespace_attr
                #[cxx_name="CxxQtConstructorInitializeArguments1"]
                #[doc(hidden)]
                struct CxxQtConstructorInitializeArgumentsMyObject1<'lifetime> {
                    arg0: i32,
                    arg1: &'lifetime QString,
                }
            },
        );
        assert_tokens_eq(
            &blocks.cxx_mod_contents[9],
            quote! {
                extern "Rust" {
                    #namespace_attr
                    #[cxx_name = "routeArguments1"]
                    unsafe fn route_arguments_my_object_1<'lifetime>(arg0: *const QObject) -> CxxQtConstructorArgumentsMyObject1<'lifetime>;

                    #namespace_attr
                    #[cxx_name = "newRs1"]
                    fn new_rs_my_object_1(args: CxxQtConstructorNewArgumentsMyObject1) -> Box<MyObjectRust>;

                    #namespace_attr
                    #[cxx_name = "initialize1"]
                    unsafe fn initialize_my_object_1<'lifetime>(qobject: Pin<&mut MyObject>, args: CxxQtConstructorInitializeArgumentsMyObject1<'lifetime>);
                }
            },
        );

        assert_tokens_eq(
            &blocks.cxx_qt_mod_contents[3],
            quote! {
                #[doc(hidden)]
                pub fn route_arguments_my_object_1<'lifetime>(arg0: *const QObject) -> ffi::CxxQtConstructorArgumentsMyObject1<'lifetime>
                {
                    #[allow(unused_variables)]
                    #[allow(clippy::let_unit_value)]
                    let (new_arguments, base_arguments, initialize_arguments) =
                        <MyObject as cxx_qt::Constructor<(*const QObject,)> >::route_arguments((arg0,));

                    ffi::CxxQtConstructorArgumentsMyObject1 {
                        base: ffi::CxxQtConstructorBaseArgumentsMyObject1 {
                            arg0: base_arguments.0,
                            arg1: base_arguments.1,
                            arg2: base_arguments.2,
                        },
                        initialize: ffi::CxxQtConstructorInitializeArgumentsMyObject1 {
                            arg0: initialize_arguments.0,
                            arg1: initialize_arguments.1,
                        },
                        new: ffi::CxxQtConstructorNewArgumentsMyObject1 {
                            arg0: new_arguments.0,
                        },
                    }
                }
            },
        );
        assert_tokens_eq(
            &blocks.cxx_qt_mod_contents[4],
            quote! {
                #[doc(hidden)]
                #[allow(unused_variables)]
                #[allow(clippy::extra_unused_lifetimes)]
                pub fn new_rs_my_object_1(new_arguments: ffi::CxxQtConstructorNewArgumentsMyObject1) -> std::boxed::Box<MyObjectRust> {
                    std::boxed::Box::new(
                        <MyObject as cxx_qt::Constructor<(*const QObject,)> >::new(
                            (new_arguments.arg0,)))
                }
            },
        );
        assert_tokens_eq(
            &blocks.cxx_qt_mod_contents[5],
            quote! {
                #[doc(hidden)]
                #[allow(unused_variables)]
                #[allow(clippy::extra_unused_lifetimes)]
                pub fn initialize_my_object_1<'lifetime>(
                    qobject: core::pin::Pin<&mut MyObject>,
                    initialize_arguments: ffi::CxxQtConstructorInitializeArgumentsMyObject1<'lifetime>)
                {
                    <MyObject as cxx_qt::Constructor<(*const QObject,)> >::initialize(
                        qobject,
                        (initialize_arguments.arg0, initialize_arguments.arg1,));
                }
            },
        );
    }

    #[test]
    fn multiple_constructors() {
        let blocks = generate_mocked(&[
            mock_constructor(),
            Constructor {
                arguments: vec![parse_quote! { *const QObject }],
                new_arguments: vec![parse_quote! { i16 }],
                initialize_arguments: vec![
                    parse_quote! { i32 },
                    parse_quote! { &'lifetime QString },
                ],
                base_arguments: vec![
                    parse_quote! { i64 },
                    parse_quote! { *mut QObject },
                    parse_quote! { f32 },
                ],
                lifetime: Some(parse_quote! { 'lifetime }),
                ..mock_constructor()
            },
        ]);

        assert_eq!(blocks.cxx_mod_contents.len(), 10);
        assert_eq!(blocks.cxx_qt_mod_contents.len(), 6);

        let namespace_attr = quote! {
                #[namespace = "ffi::cxx_qt_my_object"]
        };

        assert_empty_constructor_blocks(&blocks, &namespace_attr);

        assert_full_constructor_blocks(&blocks, &namespace_attr);
    }

    #[test]
    fn constructor_impl_with_unused_lifetime() {
        let result = super::generate(
            &[Constructor {
                lifetime: Some(parse_quote! { 'a }),
                ..mock_constructor()
            }],
            &mock_name(),
            &mock_namespace(),
            &BTreeMap::<Ident, Path>::default(),
            &format_ident!("ffi"),
        );

        assert!(result.is_err());
    }
}
