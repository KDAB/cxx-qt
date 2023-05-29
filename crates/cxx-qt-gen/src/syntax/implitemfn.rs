// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{FnArg, Receiver, Signature};

use super::types::{is_pin_mut, is_pin_of_self};

/// Return whether the first parameter of a method is Pin<&mut Self>
pub fn is_method_mutable_pin_of_self(signature: &Signature) -> bool {
    match signature.inputs.first() {
        Some(FnArg::Receiver(Receiver { ty, .. })) => {
            // Check if Pin<T> is mut and T is self
            is_pin_mut(ty) && is_pin_of_self(ty)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::{parse_quote, ImplItemFn};

    #[test]
    fn test_is_method_mutable_self() {
        let item: ImplItemFn = parse_quote! {
            fn invokable(&self) {}
        };
        assert!(!is_method_mutable_pin_of_self(&item.sig));

        let item: ImplItemFn = parse_quote! {
            fn invokable(&mut self) {}
        };
        assert!(!is_method_mutable_pin_of_self(&item.sig));

        let item: ImplItemFn = parse_quote! {
            fn invokable_with_return_type(self: Pin<&mut Self>) -> f64 {}
        };
        assert!(is_method_mutable_pin_of_self(&item.sig));

        let item: ImplItemFn = parse_quote! {
            fn invokable_with_return_type(mut self: Pin<&mut Self>) -> f64 {}
        };
        assert!(is_method_mutable_pin_of_self(&item.sig));
    }

    #[test]
    fn test_is_method_mutable_value() {
        let item: ImplItemFn = parse_quote! {
            fn invokable(value: T) {}
        };
        assert!(!is_method_mutable_pin_of_self(&item.sig));

        let item: ImplItemFn = parse_quote! {
            fn invokable_with_return_type(value: Pin<&mut T>) -> f64 {}
        };
        assert!(!is_method_mutable_pin_of_self(&item.sig));

        let item: ImplItemFn = parse_quote! {
            fn invokable_with_return_type(mut value: Pin<&mut T>) -> f64 {}
        };
        assert!(!is_method_mutable_pin_of_self(&item.sig));

        let item: ImplItemFn = parse_quote! {
            fn invokable_with_return_type(mut value: T) -> f64 {}
        };
        assert!(!is_method_mutable_pin_of_self(&item.sig));
    }
}
