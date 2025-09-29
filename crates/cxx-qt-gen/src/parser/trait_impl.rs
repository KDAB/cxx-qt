use indoc::formatdoc;
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use syn::{parse_quote, Error, Ident, ItemImpl, Path, Result, Token, Type, TypePath};

use crate::{parser::constructor::Constructor, syntax::path::path_compare_str};

/// The kind of marker trait implementation.
#[derive(Debug, PartialEq, Eq)]
pub enum TraitKind {
    Threading,
    Constructor(Box<Constructor>),
}

impl TraitKind {
    fn parse_threading(not: &Option<Token![!]>, path: &Path, imp: &ItemImpl) -> Result<Self> {
        if let Some(unsafety) = imp.unsafety.as_ref() {
            return Err(Error::new_spanned(
                unsafety,
                "Unnecessary unsafe, cxx_qt::Threading is safe to implement!",
            ));
        }
        if not.is_some() {
            return Err(Error::new_spanned(
                path,
                "Negative impls for cxx_qt::Threading are not allowed!",
            ));
        }
        Ok(Self::Threading)
    }

    fn parse_constructor(imp: &ItemImpl) -> Result<Self> {
        let constructor = Constructor::parse(imp.clone())?;
        Ok(Self::Constructor(Box::new(constructor)))
    }

    fn parse(imp: &ItemImpl) -> Result<Self> {
        let (not, path, _) = &imp
            .trait_
            .as_ref()
            .ok_or_else(|| Error::new_spanned(imp.clone(), "Expected trait impl!"))?;

        if path_compare_str(path, &["cxx_qt", "Threading"]) {
            Self::parse_threading(not, path, imp)
        } else if path_compare_str(path, &["cxx_qt", "Constructor"]) {
            Self::parse_constructor(imp)
        } else if path_compare_str(path, &["cxx_qt", "Initialize"]) {
            let struct_name = &*imp.self_ty;
            // Inside the bridge, Initialize is shorthand for the line below
            let default_constructor: ItemImpl = parse_quote! {
                impl cxx_qt::Constructor<()> for #struct_name {}
            };
            Self::parse_constructor(&default_constructor)
        } else {
            // TODO: Give suggestions on which trait might have been meant
            Err(Error::new_spanned(
                path,
                formatdoc! {"
                    Unsupported trait!
                    CXX-Qt currently only supports:
                      - cxx_qt::Threading
                      - cxx_qt::Constructor
                      - cxx_qt::Initialize (as shorthand for Constructor<()>)
                      - (cxx_qt::Locking has been removed as of CXX-Qt 0.7)
                    Note that the trait must always be fully-qualified.
                    "},
            ))
        }
    }
}

/// A marker trait implementation that has been picked up by the CXX-Qt parser.
#[derive(Debug)]
pub struct TraitImpl {
    pub qobject: Ident,
    pub declaration: ItemImpl,
    pub kind: TraitKind,
}

impl TraitImpl {
    pub fn parse(imp: ItemImpl) -> Result<Self> {
        if let Some(attr) = imp.attrs.first() {
            return Err(Error::new_spanned(
                attr,
                "Attributes are not allowed on trait impls in cxx_qt::bridge",
            ));
        }

        if !imp.items.is_empty() {
            return Err(Error::new_spanned(
                imp.items.first(),
                "Only trait declarations, not implementations are allowed in the bridge!",
            ));
        }

        let invalid_path =
            || Error::new_spanned(&imp.self_ty, "Invalid type! Expected a single identifier!");
        let qobject = if let Type::Path(TypePath { path, .. }) = imp.self_ty.as_ref() {
            path.get_ident().cloned().ok_or_else(invalid_path)
        } else {
            Err(invalid_path())
        }?;
        let kind = TraitKind::parse(&imp)?;
        Ok(Self {
            qobject,
            kind,
            declaration: imp,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::format_ident;
    use syn::parse_quote;

    #[test]
    fn parse_threading() {
        let imp = parse_quote! {
            impl cxx_qt::Threading for QObject {}
        };
        let marker = TraitImpl::parse(imp).unwrap();
        assert_eq!(marker.qobject, format_ident!("QObject"));
        assert_eq!(marker.kind, TraitKind::Threading);
    }

    #[test]
    fn parse_constructor() {
        let imp = parse_quote! {
            impl cxx_qt::Constructor<(i32, i32)> for MyObject {}
        };
        let marker = TraitImpl::parse(imp).unwrap();
        assert_eq!(marker.qobject, format_ident!("MyObject"));
        assert!(matches!(marker.kind, TraitKind::Constructor(_)));
    }

    #[test]
    fn parse_constructor_shorthand() {
        let imp = parse_quote! {
            impl cxx_qt::Constructor<()> for MyObject {}
        };
        let marker = TraitImpl::parse(imp).unwrap();
        assert_eq!(marker.qobject, format_ident!("MyObject"));
        assert!(matches!(marker.kind, TraitKind::Constructor(_)));

        let imp = parse_quote! {
            impl cxx_qt::Initialize for MyObject {}
        };
        let shorthand = TraitImpl::parse(imp).unwrap();
        assert_eq!(marker.kind, shorthand.kind);
        assert_eq!(marker.qobject, shorthand.qobject);
    }

    use crate::tests::assert_parse_errors;

    #[test]
    fn test_parse_errors() {
        assert_parse_errors! {
            TraitImpl::parse =>

            // Threading is safe to implement
            { unsafe impl cxx_qt::Threading for QObject {} }
            // Threading cannot be negated
            { impl !cxx_qt::Threading for QObject {} }
            // Invalid QObject name
            { impl cxx_qt::Locking for my::path {} }
            // Invalid trait name
            { impl cxx_qt::AnotherTrait for QObject {} }
            // Invalid Path to self type
            { impl cxx_qt::Threading for *mut QObject{} }
            {
                // Attributes are not allowed
                #[my_attribute]
                impl cxx_qt::Threading for QObject {}
            }
            {
                // Item in the impl block
                impl cxx_qt::Threading for X {
                    fn some_impl() {}
                }
            }
        }
    }
}
