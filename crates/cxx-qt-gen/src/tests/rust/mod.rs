// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod common {
    use crate::tests::utils::tokens_to_syn;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::Type;

    /// Helper which returns a f64 as a [syn::Type]
    pub fn f64_type() -> Type {
        tokens_to_syn(quote! { f64 })
    }

    pub fn impl_wrap(ty: TokenStream, inner: TokenStream) -> TokenStream {
        quote! {
            impl #ty {
                #inner
            }
        }
    }

    pub fn method() -> TokenStream {
        quote! {
            fn method() {}
        }
    }

    pub fn module_wrap(m: TokenStream, inner: TokenStream) -> TokenStream {
        quote! {
            mod #m {
                #inner
            }
        }
    }

    pub fn struct_other() -> TokenStream {
        quote! {
            struct Other;
        }
    }

    pub fn use_std_type() -> TokenStream {
        quote! {
            use std::collections::HashMap;
        }
    }
}

pub mod cxx {
    use proc_macro2::TokenStream;
    use quote::quote;

    pub fn extern_rust() -> TokenStream {
        quote! {
            extern "Rust" {
                fn test();
            }
        }
    }

    pub fn module_wrap(inner: TokenStream) -> TokenStream {
        crate::tests::rust::common::module_wrap(quote! { ffi }, inner)
    }
}

pub mod cxx_qt {
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};
    use syn::Ident;

    /// Minimal enum for signals
    pub fn enum_qsignals() -> TokenStream {
        quote! {
            enum MySignals {
                Ready,
            }
        }
    }

    pub fn impl_qobject_ident() -> TokenStream {
        quote! { qobject::MyObject }
    }

    pub fn impl_qobject_invalid_ident() -> TokenStream {
        quote! { qobject::MyObject::Invalid }
    }

    pub fn impl_qobject_unknown_ident() -> TokenStream {
        quote! { qobject::UnknownObject }
    }

    pub fn macro_bridge() -> TokenStream {
        quote! {
            #[cxx_qt::bridge]
        }
    }

    pub fn macro_bridge_with_namespace() -> TokenStream {
        quote! {
            #[cxx_qt::bridge(namespace = "cxx_qt")]
        }
    }

    pub fn macro_qinvokable() -> TokenStream {
        quote! {
            #[qinvokable]
        }
    }

    pub fn macro_qobject() -> TokenStream {
        quote! {
            #[cxx_qt::qobject]
        }
    }

    /// Macro for qsignals with the default struct for QObject
    pub fn macro_qsignals() -> TokenStream {
        quote! {
            #[cxx_qt::qsignals(MyObject)]
        }
    }

    pub fn macro_qsignals_no_qobject() -> TokenStream {
        quote! {
            #[cxx_qt::qsignals]
        }
    }

    pub fn macro_qsignals_unknown_qobject() -> TokenStream {
        quote! {
            #[cxx_qt::qsignals(UnknownObject)]
        }
    }

    pub fn method_cpp_context() -> TokenStream {
        quote! {
            fn cpp_context() {}
        }
    }

    pub fn method_invokable() -> TokenStream {
        quote! {
            fn invokable() {}
        }
    }

    /// Minimal struct for a QObject
    pub fn struct_qobject() -> TokenStream {
        let ident = struct_qobject_ident();
        quote! {
            struct #ident;
        }
    }

    pub fn struct_qobject_ident() -> Ident {
        format_ident!("MyObject")
    }

    pub fn struct_qobject_second() -> TokenStream {
        let ident = struct_qobject_second_ident();
        quote! {
            struct #ident;
        }
    }

    pub fn struct_qobject_second_ident() -> Ident {
        format_ident!("SecondObject")
    }
}
