// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::extract::QObject;
use crate::utils::is_type_ident_ptr;

/// Generate Rust code that used CXX to interact with the C++ code generated for a QObject
pub fn generate_qobject_cxx(obj: &QObject) -> Result<TokenStream, TokenStream> {
    let class_name = &obj.ident;
    let rust_class_name = &obj.rust_struct_ident;

    // TODO: Abstract this calculation to make it common to gen_rs and gen_cpp
    let ident_snake = class_name.to_string().to_case(Case::Snake);
    let import_path = format!("cxx-qt-gen/include/{}.h", ident_snake);

    let mut cpp_functions = Vec::new();
    let mut rs_functions = Vec::new();

    // Invokables are only added to extern rust side
    //
    // TODO: later support a cxx_qt_name attribute on invokables to allow for renaming
    // an invokable from snake to camel case for C++
    for i in &obj.invokables {
        let ident = &i.ident;
        let parameters = &i.parameters;

        if parameters.is_empty() {
            // Determine if there is a return type and if it's a reference
            if let Some(return_type) = &i.return_type {
                let type_idents = &return_type.idents;
                if return_type.is_ref {
                    rs_functions.push(quote! {
                        fn #ident(self: &#rust_class_name) -> &#(#type_idents)::*;
                    });
                } else {
                    rs_functions.push(quote! {
                        fn #ident(self: &#rust_class_name) -> #(#type_idents)::*;
                    });
                }
            } else {
                rs_functions.push(quote! {
                    fn #ident(self: &#rust_class_name);
                });
            }
        } else {
            let mut parameters_quotes = Vec::new();
            for p in parameters {
                let ident = &p.ident;
                let type_idents = &p.type_ident.idents;
                if p.type_ident.is_ref {
                    parameters_quotes.push(quote! {
                        #ident: &#(#type_idents)::*
                    });
                } else {
                    parameters_quotes.push(quote! {
                        #ident: #(#type_idents)::*
                    });
                }
            }

            // TODO: add cpp functions for the invokable so that it can be called
            // consider how the different types for strings work here

            // Determine if there is a return type and if it's a reference
            if let Some(return_type) = &i.return_type {
                let type_idents = &return_type.idents;
                if return_type.is_ref {
                    rs_functions.push(quote! {
                        fn #ident(self: &#rust_class_name, #(#parameters_quotes),*) -> &#(#type_idents)::*;
                    });
                } else {
                    rs_functions.push(quote! {
                        fn #ident(self: &#rust_class_name, #(#parameters_quotes),*) -> #(#type_idents)::*;
                    });
                }
            } else {
                rs_functions.push(quote! {
                    fn #ident(self: &#rust_class_name, #(#parameters_quotes),*);
                });
            }
        }
    }

    // Add getters/setters/notify from properties
    for property in &obj.properties {
        let type_idents = &property.type_ident.idents;

        // This type is a pointer, so special case the C++ functions and no Rust functions
        if is_type_ident_ptr(type_idents) {
            // Check that type_idents is not empty
            if type_idents.is_empty() {
                return Err(syn::Error::new(
                    property.ident.span(),
                    "Property type needs at least one type ident.",
                )
                .to_compile_error());
            }
            // We can assume that unwrap will work here as we have checked that type_idents is not empty
            //
            // Build the class name of the pointer, eg Object in crate::module::Object
            let ptr_class_name = type_idents.last().unwrap();

            // Swap the last type segment to be CppObj
            // so that crate::module::Object becomes crate::module::CppObj
            //
            // As we will generate a public type which points to the ffi type at the module level
            let mut type_idents_ffi = type_idents.clone();
            type_idents_ffi.pop();
            type_idents_ffi.push(format_ident!("CppObj"));

            cpp_functions.push(quote! {
                type #ptr_class_name = #(#type_idents_ffi)::*;
            });

            // Build the C++ method declarations
            let property_ident_snake = property.ident.to_string().to_case(Case::Snake);
            let property_ident_pascal = property.ident.to_string().to_case(Case::Pascal);
            let getter = format!("take_{}", property_ident_snake);
            let getter_cpp = format_ident!("take{}", property_ident_pascal);
            let setter = format!("give_{}", property_ident_snake);
            let setter_cpp = format_ident!("give{}", property_ident_pascal);
            cpp_functions.push(quote! {
                #[rust_name = #getter]
                fn #getter_cpp(self: Pin<&mut #class_name>) -> UniquePtr<#ptr_class_name>;
                #[rust_name = #setter]
                fn #setter_cpp(self: Pin<&mut #class_name>, value: UniquePtr<#ptr_class_name>);
            });
        // This is a normal primitive type so add Rust getters and setters
        } else {
            // Build the getter
            //
            // TODO: do we need the setter on the cpp side?
            //
            // TODO: what should happen with refs in the type here?
            // do we always return a ref of the same type as the property?
            if let Some(getter) = &property.getter {
                let getter_ident = &getter.rust_ident;
                let getter_cpp_ident = getter.cpp_ident.to_string();
                rs_functions.push(quote! {
                    #[cxx_name = #getter_cpp_ident]
                    fn #getter_ident(self: &#rust_class_name) -> &#(#type_idents)::*;
                });
            }

            // Build the setter
            //
            // TODO: do we need the setter on the cpp side?
            //
            // TODO: what should happen with refs in the type here?
            // do we always take by value of the same type as the property?
            if let Some(setter) = &property.setter {
                let setter_ident = &setter.rust_ident;
                let setter_cpp_ident = setter.cpp_ident.to_string();
                rs_functions.push(quote! {
                    #[cxx_name = #setter_cpp_ident]
                    fn #setter_ident(self: &mut #rust_class_name, value: #(#type_idents)::*);
                });
            }
        }
    }

    let new_object_ident_cpp = format_ident!("new{}", class_name);
    let new_object_rust = format!("new_{}", class_name);
    let create_object_ident = format_ident!("create_{}_rs", ident_snake);
    let create_object_cpp = create_object_ident.to_string().to_case(Case::Camel);

    let output = quote! {
        #[cxx::bridge]
        mod ffi {
            unsafe extern "C++" {
                include!(#import_path);

                type #class_name;

                #(#cpp_functions)*

                #[rust_name = #new_object_rust]
                fn #new_object_ident_cpp() -> UniquePtr<#class_name>;
            }

            extern "Rust" {
                type #rust_class_name;

                #(#rs_functions)*

                #[cxx_name = #create_object_cpp]
                fn #create_object_ident() -> Box<#rust_class_name>;
            }
        }

        pub type CppObj = ffi::#class_name;
    };
    Ok(output.into_token_stream())
}

/// Generate a Rust function that heap constructs the Rust object corresponding to the QObject
fn generate_rust_object_creator(obj: &QObject) -> Result<TokenStream, TokenStream> {
    let class_name = &obj.ident;
    let rust_class_name = &obj.rust_struct_ident;

    let ident_snake = class_name.to_string().to_case(Case::Snake);
    let fn_ident = format_ident!("create_{}_rs", ident_snake);

    // TODO: check if the original object had an explicit constructor and if so ensure that the create
    // function also takes the same parameters so that it can call this constructor. The C++ object will
    // also need to take the same parameters in its constructor. If the object is not default constructable
    // and does not provide a constructor then we need to throw an error.
    //
    // TODO: for now we assume that any object with properties implements Default. This likely means
    // for now it needs to derive from Default. As we don't (?) currently rename multiple impl
    // blocks - eg if there was a impl Default for Struct.

    // If an object has properties, we assume that it implements Default.
    let output = if obj.properties.is_empty() {
        quote! {
            fn #fn_ident() -> Box<#rust_class_name> {
                Box::new(#rust_class_name {})
            }
        }
    } else {
        quote! {
            fn #fn_ident() -> Box<#rust_class_name> {
                Box::new(#rust_class_name::default())
            }
        }
    };
    Ok(output.into_token_stream())
}

fn generate_property_methods_rs(obj: &QObject) -> Result<Vec<TokenStream>, TokenStream> {
    let mut property_methods = Vec::new();
    let rust_class_name = &obj.rust_struct_ident;

    for property in &obj.properties {
        let property_ident = &property.ident;
        let type_idents = &property.type_ident.idents;

        // Only add Rust getters and setters if we are not a special case of a pointer
        // If the type is a pointer then the getters are setters are on the C++ side
        if !is_type_ident_ptr(type_idents) {
            // TODO: later we might need consider if the struct has already implemented custom getters
            if let Some(getter) = &property.getter {
                let getter_ident = &getter.rust_ident;
                property_methods.push(quote! {
                    fn #getter_ident(self: &#rust_class_name) -> &#(#type_idents)::* {
                        &self.#property_ident
                    }
                });
            }

            // TODO: later we might need consider if the struct has already implemented custom setters
            if let Some(setter) = &property.setter {
                let setter_ident = &setter.rust_ident;
                property_methods.push(quote! {
                    fn #setter_ident(self: &mut #rust_class_name, value: #(#type_idents)::*) {
                        self.#property_ident = value;
                    }
                });
            }
        }
    }

    Ok(property_methods)
}

/// Generate all the Rust code required to communicate with a QObject backed by generated C++ code
pub fn generate_qobject_rs(obj: &QObject) -> Result<TokenStream, TokenStream> {
    // Load module information
    let mod_attrs = obj
        .original_mod
        .attrs
        .iter()
        .filter_map(|attr| {
            // Filter out any attributes that are #[make_qobject] as that is ourselves
            //
            // TODO: what happens if there are multiple macros to start from?
            // Will generate_qobject_rs only ever come from make_qobject?
            // Otherwise we might need to pass the originating macro from the
            // calling proc_macro_attribute method.
            if let Some(first_segment) = attr.path.segments.first() {
                if first_segment.ident.to_string().as_str() != "make_qobject" {
                    Some(attr.to_owned())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<syn::Attribute>>();
    let mod_ident = &obj.original_mod.ident;
    let mod_vis = &obj.original_mod.vis;

    // Generate cxx block
    let cxx_block = generate_qobject_cxx(obj)?;

    let rust_class_name = &obj.rust_struct_ident;
    // Create our renamed struct eg MyObject -> MyObjectRs with any filtering required
    let renamed_struct = {
        // Filter the fields of the struct to remove any pointer fields
        // as they are instead stored in the C++ object and not owned by the rust side
        let filtered_fields = obj
            .original_struct
            .fields
            .iter()
            .filter(|field| {
                let ty_path;
                match &field.ty {
                    syn::Type::Path(path) => {
                        ty_path = path;
                    }
                    syn::Type::Reference(syn::TypeReference { elem, .. }) => {
                        if let syn::Type::Path(path) = &**elem {
                            ty_path = path;
                        } else {
                            // Unknown type, just ignore so we pass through
                            return true;
                        }
                    }
                    _others => {
                        // Unknown type, just ignore so we pass through
                        return true;
                    }
                }

                // Filter any fields that have a type which is a pointer
                !is_type_ident_ptr(
                    &ty_path
                        .path
                        .segments
                        .iter()
                        .map(|segment| segment.ident.to_owned())
                        .collect::<Vec<syn::Ident>>(),
                )
            })
            .collect::<Vec<&syn::Field>>();
        let original_attributes = &obj.original_struct.attrs;
        let original_generics = &obj.original_struct.generics;
        let original_visibility = &obj.original_struct.vis;

        // Finally build the renamed struct
        //
        // If there are no fields then use semi-colon instead of brackets
        if filtered_fields.is_empty() {
            quote! {
                #(#original_attributes)*
                #original_visibility struct #rust_class_name #original_generics;
            }
        } else {
            quote! {
                #(#original_attributes)*
                #original_visibility struct #rust_class_name #original_generics {
                    #(#filtered_fields),*
                }
            }
        }
    };

    let property_methods = generate_property_methods_rs(obj)?;
    let original_methods = obj.invokables.iter().map(|m| &m.original_method);
    let original_trait_impls = &obj.original_trait_impls;
    let original_use_decls = &obj.original_use_decls;

    let creator_fn = generate_rust_object_creator(obj)?;

    // Determine if we need an impl block on the renamed struct
    let renamed_struct_impl = if original_methods.len() != 0 || !property_methods.is_empty() {
        quote! {
            impl #rust_class_name {
                #(#original_methods)*
                #(#property_methods)*
            }
        }
    } else {
        quote! {}
    };

    let output = quote! {
        #(#mod_attrs)*
        #mod_vis mod #mod_ident {
            #(#original_use_decls)*

            #cxx_block

            #renamed_struct

            #renamed_struct_impl

            #(#original_trait_impls)*

            #creator_fn
        }
    };
    Ok(output.into_token_stream())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract_qobject;

    use pretty_assertions::assert_eq;
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    use syn::ItemMod;

    fn format_rs_source(rs_code: &str) -> String {
        // NOTE: this error handling is pretty rough so should only used for tests
        let mut command = Command::new("rustfmt");
        let mut child = command
            .args(&["--emit", "stdout"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        // Scope stdin to force an automatic flush
        {
            let mut stdin = child.stdin.take().unwrap();
            write!(stdin, "{}", rs_code).unwrap();
        }

        let output = child.wait_with_output().unwrap();
        let output = String::from_utf8(output.stdout).unwrap();

        // Quote does not retain empty lines so we throw them away in the case of the
        // reference string as to not cause clashes
        output.replace("\n\n", "\n")
    }

    #[test]
    fn generates_basic_custom_default() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_custom_default.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_custom_default.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_invokable_and_properties() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_invokable_and_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_invokable_and_properties.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_only_invokables() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_only_invokable.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_only_invokables_with_return() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_invokable_return.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_only_invokable_return.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_only_properties() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_only_properties.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_mod_attrs_vis() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_mod_attrs_vis.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_mod_attrs_vis.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_basic_mod_use() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_mod_use.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/basic_mod_use.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    #[test]
    fn generates_subobject_property() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/subobject_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_output = include_str!("../test_outputs/subobject_property.rs");
        let expected_output = format_rs_source(expected_output);

        let generated_rs = generate_qobject_rs(&qobject).unwrap().to_string();
        let generated_rs = format_rs_source(&generated_rs);

        assert_eq!(generated_rs, expected_output);
    }

    // TODO: add tests for more complex cases such as invokables with parameters
    // and for objects with properties
}
