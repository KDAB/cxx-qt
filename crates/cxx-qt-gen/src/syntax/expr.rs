// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use syn::{spanned::Spanned, Error, Expr, ExprLit, Lit, Result};

/// Convert a given [syn::Expr] to a String
pub fn expr_to_string(expr: &Expr) -> Result<String> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(lit_str),
        ..
    }) = expr
    {
        return Ok(lit_str.value());
    }

    Err(Error::new(expr.span(), "expected a literal string"))
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_expr_lit_int() {
        assert!(expr_to_string(&parse_quote! { 1 }).is_err());
    }

    #[test]
    fn test_expr_lit_str() {
        assert_eq!(
            expr_to_string(&parse_quote! { "literal" }).unwrap(),
            "literal".to_owned()
        );
    }

    #[test]
    fn test_expr_path() {
        assert!(expr_to_string(&parse_quote! { std::collections::HashMap }).is_err());
    }
}
