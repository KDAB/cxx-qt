// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{
    FnArg, GenericArgument, Pat, PatIdent, PatType, PathArguments, Receiver, Signature, Type,
    TypePath, TypeReference,
};

/// Return whether the first parameter of a method is a mutable self argument
//
// Note that self: Box<Self> is parsed as FnArg::Typed not FnArg::Receiver so will be false
// but we don't use this case with CXX, so this can be ignored.
pub fn is_method_mutable(signature: &Signature) -> bool {
    match signature.inputs.first() {
        Some(FnArg::Receiver(Receiver { mutability, .. })) => mutability.is_some(),
        Some(FnArg::Typed(PatType { ty, pat, .. })) => {
            // Check if the parameter name is self, if it isn't then ignore
            if let Pat::Ident(PatIdent { ident, .. }) = pat.as_ref() {
                if ident != "self" {
                    return false;
                }
            }

            if let Type::Path(TypePath { path, .. }) = ty.as_ref() {
                // If we aren't a `self: Pin<T>` then ignore
                //
                // TODO: do we need to handle `self: &mut Self` or `self: mut Self` etc?
                if !crate::syntax::path::path_compare_str(path, &["Pin"]) {
                    return false;
                }

                // Read the contents of the T
                if let Some(last) = path.segments.last() {
                    if let PathArguments::AngleBracketed(args) = &last.arguments {
                        // TODO: Maybe check that the reference is of type `Self`.
                        //
                        // TODO: we only handle the Pin being a reference, do we need to handle other cases?
                        if let Some(GenericArgument::Type(Type::Reference(TypeReference {
                            mutability: Some(_),
                            ..
                        }))) = args.args.first()
                        {
                            return true;
                        }
                    }
                }
            }
            false
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::ImplItemMethod;

    #[test]
    fn test_is_method_mutable_self() {
        assert!(!is_method_mutable(
            &tokens_to_syn::<ImplItemMethod>(quote! {
                fn invokable(&self) {}
            })
            .sig
        ));

        assert!(is_method_mutable(
            &tokens_to_syn::<ImplItemMethod>(quote! {
                fn invokable_with_return_cxx_type(self: Pin<&mut Self>) -> f64 {}
            })
            .sig
        ));
    }

    #[test]
    fn test_is_method_mutable_value() {
        assert!(!is_method_mutable(
            &tokens_to_syn::<ImplItemMethod>(quote! {
                fn invokable(value: T) {}
            })
            .sig
        ));

        assert!(!is_method_mutable(
            &tokens_to_syn::<ImplItemMethod>(quote! {
                fn invokable_with_return_cxx_type(value: Pin<&mut T>) -> f64 {}
            })
            .sig
        ));

        assert!(!is_method_mutable(
            &tokens_to_syn::<ImplItemMethod>(quote! {
                fn invokable_with_return_cxx_type(mut value: T) -> f64 {}
            })
            .sig
        ));
    }
}
