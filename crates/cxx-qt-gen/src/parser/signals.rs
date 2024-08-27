// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    naming::Name,
    parser::parameter::ParsedFunctionParameter,
    syntax::{attribute::attribute_take_path, path::path_compare_str, safety::Safety},
};
use syn::{spanned::Spanned, Attribute, Error, ForeignItemFn, Ident, Result, Visibility};

use crate::parser::method::MethodFields;
use crate::parser::{check_safety, separate_docs};
#[cfg(test)]
use quote::format_ident;

#[derive(Clone)]
/// Describes an individual Signal
pub struct ParsedSignal {
    /// The original [syn::ForeignItemFn] of the signal declaration
    pub method: ForeignItemFn,
    /// The type of the self argument
    pub qobject_ident: Ident,
    /// whether the signal is marked as mutable
    pub mutable: bool,
    /// Whether the method is safe to call.
    pub safe: bool,
    /// The parameters of the signal
    pub parameters: Vec<ParsedFunctionParameter>,
    /// The name of the signal
    pub name: Name,
    /// If the signal is defined in the base class
    pub inherit: bool,
    /// Whether the signal is private
    pub private: bool,
    /// All the doc attributes (each line) of the signal
    pub docs: Vec<Attribute>,
}

impl ParsedSignal {
    /// Builds a signal from a given property method
    pub fn from_property_method(
        method: ForeignItemFn,
        name: Name,
        qobject_ident: Ident,
        docs: Vec<Attribute>,
    ) -> Self {
        Self {
            method,
            qobject_ident,
            mutable: true,
            safe: true,
            parameters: vec![],
            name,
            inherit: false,
            private: false,
            docs,
        }
    }

    #[cfg(test)]
    /// Test fn for creating a mocked signal from a method body
    pub fn mock_with_method(method: &ForeignItemFn) -> Self {
        Self {
            method: method.clone(),
            qobject_ident: format_ident!("MyObject"),
            mutable: true,
            parameters: vec![],
            name: Name::from_rust_ident_and_attrs(&method.sig.ident, &method.attrs, None, None)
                .unwrap(),
            safe: true,
            inherit: false,
            private: false,
            docs: vec![],
        }
    }

    pub fn parse(mut method: ForeignItemFn, safety: Safety) -> Result<Self> {
        check_safety(&method, &safety)?;

        let docs = separate_docs(&mut method);
        let invokable_fields = MethodFields::parse(&method, docs)?;

        if invokable_fields.name.namespace().is_some() {
            return Err(Error::new_spanned(
                method.sig.ident,
                "Signals cannot have a namespace attribute!",
            ));
        }

        if !invokable_fields.mutable {
            return Err(Error::new(
                method.span(),
                "signals must be mutable, use Pin<&mut T> instead of T for the self type",
            ));
        }

        let inherit = attribute_take_path(&mut method.attrs, &["inherit"]).is_some();
        let private = if let Visibility::Restricted(vis_restricted) = &method.vis {
            path_compare_str(&vis_restricted.path, &["self"])
        } else {
            false
        };

        Ok(Self::from_invokable_fields(
            invokable_fields,
            method,
            inherit,
            private,
        ))
    }

    fn from_invokable_fields(
        fields: MethodFields,
        method: ForeignItemFn,
        inherit: bool,
        private: bool,
    ) -> Self {
        Self {
            method,
            qobject_ident: fields.qobject_ident,
            mutable: fields.mutable,
            safe: fields.safe,
            parameters: fields.parameters,
            name: fields.name,
            inherit,
            private,
            docs: fields.docs,
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    use crate::parser::tests::f64_type;
    use quote::format_ident;

    #[test]
    fn test_parse_signal() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut MyObject>);
        };
        let signal = ParsedSignal::parse(method.clone(), Safety::Safe).unwrap();
        assert_eq!(signal.method, method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters, vec![]);
        assert_eq!(signal.name, Name::new(format_ident!("ready")));
        assert!(signal.safe);
        assert!(!signal.inherit);
        assert!(!signal.private);
    }

    #[test]
    fn test_parse_signal_cxx_name() {
        let method: ForeignItemFn = parse_quote! {
            #[cxx_name = "cppReady"]
            fn ready(self: Pin<&mut MyObject>);
        };
        let signal = ParsedSignal::parse(method, Safety::Safe).unwrap();

        let expected_method: ForeignItemFn = parse_quote! {
            #[cxx_name = "cppReady"]
            fn ready(self: Pin<&mut MyObject>);
        };
        assert_eq!(signal.method, expected_method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters, vec![]);
        assert_eq!(
            signal.name,
            Name::new(format_ident!("ready")).with_cxx_name("cppReady".to_owned())
        );
        assert!(signal.safe);
        assert!(!signal.inherit);
        assert!(!signal.private);
    }

    #[test]
    fn test_parse_signal_inherit() {
        let method: ForeignItemFn = parse_quote! {
            #[inherit]
            fn ready(self: Pin<&mut MyObject>);
        };
        let signal = ParsedSignal::parse(method, Safety::Safe).unwrap();

        let expected_method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut MyObject>);
        };
        assert_eq!(signal.method, expected_method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters, vec![]);
        assert_eq!(signal.name, Name::new(format_ident!("ready")));
        assert!(signal.safe);
        assert!(signal.inherit);
        assert!(!signal.private);
    }

    #[test]
    fn test_parse_signal_mutable_err() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(self: &MyObject);
        };
        // Can't be immutable
        assert!(ParsedSignal::parse(method, Safety::Safe).is_err());
    }

    #[test]
    fn test_parse_signal_namespace_err() {
        let method: ForeignItemFn = parse_quote! {
            #[namespace = "disallowed_namespace"]
            fn ready(self: Pin<&mut MyObject>);
        };
        // Can't have a namespace attr
        assert!(ParsedSignal::parse(method, Safety::Safe).is_err());
    }

    #[test]
    fn test_parse_signal_parameters() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut MyObject>, x: f64, y: f64);
        };
        let signal = ParsedSignal::parse(method.clone(), Safety::Safe).unwrap();
        assert_eq!(signal.method, method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters.len(), 2);
        assert_eq!(signal.parameters[0].ident, format_ident!("x"));
        assert_eq!(signal.parameters[0].ty, f64_type());
        assert_eq!(signal.parameters[1].ident, format_ident!("y"));
        assert_eq!(signal.parameters[1].ty, f64_type());
        assert_eq!(signal.name, Name::new(format_ident!("ready")));
        assert!(signal.safe);
        assert!(!signal.inherit);
        assert!(!signal.private);
    }

    #[test]
    fn test_parse_signal_private() {
        let method: ForeignItemFn = parse_quote! {
            pub(self) fn ready(self: Pin<&mut MyObject>);
        };
        let signal = ParsedSignal::parse(method.clone(), Safety::Safe).unwrap();
        assert_eq!(signal.method, method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters, vec![]);
        assert_eq!(signal.name, Name::new(format_ident!("ready")));
        assert!(signal.safe);
        assert!(!signal.inherit);
        assert!(signal.private);
    }

    #[test]
    fn test_parse_signal_qobject_self_missing() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(x: f64);
        };
        // Can't have a missing self
        assert!(ParsedSignal::parse(method, Safety::Safe).is_err());
    }

    #[test]
    fn test_parse_signal_qobject_ident_missing() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(&self);
        };
        // Can't have a missing ident
        assert!(ParsedSignal::parse(method, Safety::Safe).is_err());
    }

    #[test]
    fn test_parse_signal_unsafe() {
        let method: ForeignItemFn = parse_quote! {
            unsafe fn ready(self: Pin<&mut MyObject>);
        };
        let signal = ParsedSignal::parse(method.clone(), Safety::Unsafe).unwrap();
        assert_eq!(signal.method, method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters, vec![]);
        assert_eq!(signal.name, Name::new(format_ident!("ready")));
        assert!(!signal.safe);
        assert!(!signal.inherit);
        assert!(!signal.private);
    }

    #[test]
    fn test_parse_signal_unsafe_error() {
        let method: ForeignItemFn = parse_quote! {
            fn ready(self: Pin<&mut MyObject>);
        };
        // Can't be safe on the block and the method
        assert!(ParsedSignal::parse(method, Safety::Unsafe).is_err());
    }
}
