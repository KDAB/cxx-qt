// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// use convert_case::{Case, Casing};
// use quote::format_ident;
// use syn::Ident;

// // /// Create a C++ version of the ident
// // pub fn as_cpp_ident(ident: &Ident) -> Ident {
// //     quote::format_ident!("{}", ident.to_string().to_case(Case::Pascal))
// // }

// // /// Create a Rust version of the ident
// // //
// // // TODO: do we even need this? the Rust side should just be the input?
// // pub fn as_rust_ident(ident: &Ident) -> Ident {
// //     quote::format_ident!("{}", ident.to_string().to_case(Case::Snake))
// // }

// // /// Create a wrapper of the ident
// // //
// // // TODO: we need a C++ and Rust variant of this? have ones in each module?
// // pub fn as_wrapper_ident(ident: &Ident) -> Ident {
// //     format_ident!("{}Wrapper", ident)
// // }
