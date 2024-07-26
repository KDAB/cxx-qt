// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    generator::{
        naming::{namespace::NamespaceName, qobject::QObjectNames},
        rust::{
            constructor, cxxqttype,
            fragment::{GeneratedRustFragment, RustFragmentPair},
            inherit,
            method::generate_rust_methods,
            property::generate_rust_properties,
            signals::generate_rust_signals,
            threading,
        },
    },
    naming::TypeNames,
    parser::qobject::ParsedQObject,
};
use quote::quote;
use syn::{Ident, Result};

impl GeneratedRustFragment {
    // Might need to be refactored to use a StructuredQObject instead (confirm with Leon)
    pub fn from_qobject(
        qobject: &ParsedQObject,
        type_names: &TypeNames,
        module_ident: &Ident,
    ) -> Result<Self> {
        // Create the base object
        let qobject_idents = QObjectNames::from_qobject(qobject, type_names)?;
        let namespace_idents = NamespaceName::from(qobject);
        let mut generated = Self::default();

        generated.append(&mut generate_qobject_definitions(
            &qobject_idents,
            &namespace_idents.namespace,
        )?);

        // Generate methods for the properties, invokables, signals
        // TODO: BEN refactor using methods and signals inside structured (might need to pass structured QObject instead of ParsedQObject
        generated.append(&mut generate_rust_properties(
            &qobject.properties,
            &qobject_idents,
            type_names,
            module_ident,
        )?);
        generated.append(&mut generate_rust_methods(
            &qobject.methods,
            &qobject_idents,
        )?);
        generated.append(&mut inherit::generate(
            &qobject_idents,
            &qobject.inherited_methods,
        )?);
        generated.append(&mut generate_rust_signals(
            &qobject.signals,
            &qobject_idents,
            type_names,
            module_ident,
        )?);

        // If this type is a singleton then we need to add an include
        if let Some(qml_metadata) = &qobject.qml_metadata {
            if qml_metadata.singleton {
                let fragment = RustFragmentPair {
                    cxx_bridge: vec![quote! {
                        unsafe extern "C++" {
                            include!(<QtQml/QQmlEngine>);
                        }
                    }],
                    implementation: vec![],
                };
                generated
                    .cxx_mod_contents
                    .append(&mut fragment.cxx_bridge_as_items()?);
            }
        }

        // If this type has threading enabled then add generation
        if qobject.threading {
            generated.append(&mut threading::generate(
                &qobject_idents,
                &namespace_idents,
                type_names,
                module_ident,
            )?);
        }

        // If this type has locking enabling then implement the trait
        //
        // This could be implemented using an auto trait in the future once stable
        // https://doc.rust-lang.org/beta/unstable-book/language-features/auto-traits.html
        if qobject.locking {
            let qualified_impl =
                type_names.rust_qualified(qobject_idents.name.rust_unqualified())?;
            generated.cxx_qt_mod_contents.push(syn::parse_quote! {
                impl cxx_qt::Locking for #qualified_impl {}
            });
        }

        generated.append(&mut constructor::generate(
            &qobject.constructors,
            &qobject_idents,
            &namespace_idents,
            type_names,
            module_ident,
        )?);

        generated.append(&mut cxxqttype::generate(&qobject_idents, type_names)?);

        Ok(generated)
    }
}

/// Generate the C++ and Rust CXX definitions for the QObject
fn generate_qobject_definitions(
    qobject_idents: &QObjectNames,
    namespace: &str,
) -> Result<GeneratedRustFragment> {
    let mut generated = GeneratedRustFragment::default();
    let cpp_class_name_rust = &qobject_idents.name.rust_unqualified();
    let cpp_class_name_cpp = &qobject_idents.name.cxx_unqualified();

    let rust_struct_name_rust = &qobject_idents.rust_struct.rust_unqualified();
    let rust_struct_name_rust_str = rust_struct_name_rust.to_string();
    let namespace = if !namespace.is_empty() {
        Some(quote! { #[namespace=#namespace] })
    } else {
        None
    };
    let cxx_name = if cpp_class_name_rust.to_string() == *cpp_class_name_cpp {
        quote! {}
    } else {
        let cpp_class_name_cpp = cpp_class_name_cpp.to_string();
        quote! {
            #[doc = "\n\nNote: The C++ name of this QObject is: "]
            #[doc = #cpp_class_name_cpp]
            #[cxx_name = #cpp_class_name_cpp]
        }
    };

    let fragment = RustFragmentPair {
        cxx_bridge: vec![
            quote! {
                unsafe extern "C++" {
                    #[doc = "The C++ type for the QObject "]
                    #[doc = #rust_struct_name_rust_str]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
                    #namespace
                    #cxx_name
                    type #cpp_class_name_rust;
                }
            },
            quote! {
                extern "Rust" {
                    type #rust_struct_name_rust;
                }
            },
        ],
        implementation: vec![],
    };

    generated
        .cxx_mod_contents
        .append(&mut fragment.cxx_bridge_as_items()?);

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::Parser;
    use crate::tests::assert_tokens_eq;
    use quote::format_ident;
    use syn::{parse_quote, ItemMod};

    #[test]
    fn test_generated_rust_qobject_blocks_singleton() {
        let module: ItemMod = parse_quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
            mod ffi {
                extern "RustQt" {
                    #[qobject]
                    #[qml_element]
                    #[qml_singleton]
                    type MyObject = super::MyObjectRust;
                }
            }
        };
        let parser = Parser::from(module).unwrap();

        let rust = GeneratedRustFragment::from_qobject(
            parser.cxx_qt_data.qobjects.values().next().unwrap(),
            &parser.type_names,
            &format_ident!("ffi"),
        )
        .unwrap();
        assert_eq!(rust.cxx_mod_contents.len(), 6);
        assert_tokens_eq(
            &rust.cxx_mod_contents[0],
            quote! {
                unsafe extern "C++" {
                    #[doc = "The C++ type for the QObject "]
                    #[doc = "MyObjectRust"]
                    #[doc = "\n"]
                    #[doc = "Use this type when referring to the QObject as a pointer"]
                    #[doc = "\n"]
                    #[doc = "See the book for more information: <https://kdab.github.io/cxx-qt/book/qobject/generated-qobject.html>"]
                    #[namespace = "cxx_qt"]
                    type MyObject;
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[1],
            quote! {
                extern "Rust" {
                    type MyObjectRust;
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[2],
            quote! {
                unsafe extern "C++" {
                    include!(<QtQml/QQmlEngine>);
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[3],
            quote! {
                extern "Rust" {
                    #[cxx_name = "createRs"]
                    #[namespace = "cxx_qt::cxx_qt_my_object"]
                    fn create_rs_my_object_rust() -> Box<MyObjectRust>;
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[4],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "unsafeRust"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_rust(self: &MyObject) -> &MyObjectRust;
                }
            },
        );
        assert_tokens_eq(
            &rust.cxx_mod_contents[5],
            quote! {
                unsafe extern "C++" {
                    #[cxx_name = "unsafeRustMut"]
                    #[doc(hidden)]
                    fn cxx_qt_ffi_rust_mut(self: Pin<&mut MyObject>) -> Pin<&mut MyObjectRust>;
                }
            },
        );
    }
}
