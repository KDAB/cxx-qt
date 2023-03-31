// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{FnArg, Receiver, Signature};

use super::types::is_pin_mut;

/// Return whether the first parameter of a method is a mutable self argument
pub fn is_method_mutable(signature: &Signature) -> bool {
    match signature.inputs.first() {
        Some(FnArg::Receiver(Receiver { mutability, ty, .. })) => {
            // Check if Pin<T> is mut or fallback to normal mutability
            is_pin_mut(ty) || mutability.is_some()
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::ImplItemFn;

    #[test]
    fn test_is_method_mutable_self() {
        assert!(!is_method_mutable(
            &tokens_to_syn::<ImplItemFn>(quote! {
                fn invokable(&self) {}
            })
            .sig
        ));

        assert!(is_method_mutable(
            &tokens_to_syn::<ImplItemFn>(quote! {
                fn invokable_with_return_cxx_type(self: Pin<&mut Self>) -> f64 {}
            })
            .sig
        ));
    }

    #[test]
    fn test_is_method_mutable_value() {
        assert!(!is_method_mutable(
            &tokens_to_syn::<ImplItemFn>(quote! {
                fn invokable(value: T) {}
            })
            .sig
        ));

        assert!(!is_method_mutable(
            &tokens_to_syn::<ImplItemFn>(quote! {
                fn invokable_with_return_cxx_type(value: Pin<&mut T>) -> f64 {}
            })
            .sig
        ));

        assert!(!is_method_mutable(
            &tokens_to_syn::<ImplItemFn>(quote! {
                fn invokable_with_return_cxx_type(mut value: T) -> f64 {}
            })
            .sig
        ));
    }
}
