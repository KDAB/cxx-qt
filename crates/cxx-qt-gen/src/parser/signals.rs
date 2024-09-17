// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{
    parser::{check_safety, method::MethodFields, require_attributes, separate_docs},
    syntax::{path::path_compare_str, safety::Safety},
};
use core::ops::Deref;
use syn::{spanned::Spanned, Attribute, Error, ForeignItemFn, Result, Visibility};
#[derive(Clone)]
/// Describes an individual Signal
pub struct ParsedSignal {
    /// The common fields which are available on all callable types
    pub method_fields: MethodFields,
    /// If the signal is defined in the base class
    pub inherit: bool,
    /// Whether the signal is private
    pub private: bool,
    /// All the doc attributes (each line) of the signal
    pub docs: Vec<Attribute>,
}

impl ParsedSignal {
    const ALLOWED_ATTRS: [&'static str; 5] = ["cxx_name", "rust_name", "inherit", "doc", "qsignal"];

    #[cfg(test)]
    /// Test fn for creating a mocked signal from a method body
    pub fn mock(method: &ForeignItemFn) -> Self {
        Self::parse(method.clone(), Safety::Safe).unwrap()
    }

    pub fn parse(mut method: ForeignItemFn, safety: Safety) -> Result<Self> {
        check_safety(&method, &safety)?;

        let docs = separate_docs(&mut method);
        let fields = MethodFields::parse(method)?;
        let attrs = require_attributes(&fields.method.attrs, &Self::ALLOWED_ATTRS)?;

        if !fields.mutable {
            return Err(Error::new(
                fields.method.span(),
                "signals must be mutable, use Pin<&mut T> instead of T for the self type",
            ));
        }

        let inherit = attrs.contains_key("inherit");

        let private = if let Visibility::Restricted(vis_restricted) = &fields.method.vis {
            path_compare_str(&vis_restricted.path, &["self"])
        } else {
            false
        };

        Ok(Self {
            method_fields: fields,
            inherit,
            private,
            docs,
        })
    }
}

impl Deref for ParsedSignal {
    type Target = MethodFields;

    fn deref(&self) -> &Self::Target {
        &self.method_fields
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    use crate::naming::Name;
    use crate::parser::tests::f64_type;
    use crate::tests::assert_parse_errors;
    use quote::format_ident;
    #[test]
    fn test_parse_signal_invalid() {
        assert_parse_errors! {
            |input| ParsedSignal::parse(input, Safety::Safe) =>

            // No immutable signals
            { fn ready(self: &MyObject); }
            {
                // No namespaces
                #[namespace = "disallowed_namespace"]
                fn ready(self: Pin<&mut MyObject>);
            }
            // Missing self
            { fn ready(x: f64); }
            // Self needs to be receiver like self: &T instead of &self
            { fn ready(&self); }
        }
    }

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
        let signal = ParsedSignal::parse(method.clone(), Safety::Safe).unwrap();

        assert_eq!(signal.method, method);
        assert_eq!(signal.qobject_ident, format_ident!("MyObject"));
        assert!(signal.mutable);
        assert_eq!(signal.parameters, vec![]);
        assert_eq!(signal.name, Name::new(format_ident!("ready")));
        assert!(signal.safe);
        assert!(signal.inherit);
        assert!(!signal.private);
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
