// SPDX-FileCopyrightText: CXX Authors
// SPDX-FileContributor: David Tolnay <dtolnay@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Originally found in the CXX repository
// https://github.com/dtolnay/cxx/blob/e26474acf4284235895d526c5ed12575cd9c0cce/syntax/cfg.rs

use proc_macro2::Ident;
use std::mem;
use syn::parse::{Error, ParseStream, Result};
use syn::{parenthesized, spanned::Spanned, token, Attribute, LitStr, Token};

#[derive(Clone)]
pub(crate) enum CfgExpr {
    Unconditional,
    Eq(Ident, Option<LitStr>),
    All(Vec<CfgExpr>),
    Any(Vec<CfgExpr>),
    Not(Box<CfgExpr>),
}

impl CfgExpr {
    pub(crate) fn merge(&mut self, expr: CfgExpr) {
        if let CfgExpr::Unconditional = self {
            *self = expr;
        } else if let CfgExpr::All(list) = self {
            list.push(expr);
        } else {
            let prev = mem::replace(self, CfgExpr::Unconditional);
            *self = CfgExpr::All(vec![prev, expr]);
        }
    }
}

pub(crate) fn parse_attribute(attr: &Attribute) -> Result<CfgExpr> {
    // Ensure that the attribute is a cfg attribute
    if attr.path().require_ident()? != "cfg" {
        return Err(Error::new(attr.span(), "Expected #[cfg(...)] attribute"));
    }

    attr.parse_args_with(|input: ParseStream| {
        let cfg_expr = input.call(parse_single)?;
        input.parse::<Option<Token![,]>>()?;
        Ok(cfg_expr)
    })
}

fn parse_single(input: ParseStream) -> Result<CfgExpr> {
    let ident: Ident = input.parse()?;
    let lookahead = input.lookahead1();
    if input.peek(token::Paren) {
        let content;
        parenthesized!(content in input);
        if ident == "all" {
            let list = content.call(parse_multiple)?;
            Ok(CfgExpr::All(list))
        } else if ident == "any" {
            let list = content.call(parse_multiple)?;
            Ok(CfgExpr::Any(list))
        } else if ident == "not" {
            let expr = content.call(parse_single)?;
            content.parse::<Option<Token![,]>>()?;
            Ok(CfgExpr::Not(Box::new(expr)))
        } else {
            Err(Error::new(ident.span(), "unrecognized cfg expression"))
        }
    } else if lookahead.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        let string: LitStr = input.parse()?;
        Ok(CfgExpr::Eq(ident, Some(string)))
    } else if lookahead.peek(Token![,]) || input.is_empty() {
        Ok(CfgExpr::Eq(ident, None))
    } else {
        // CODECOV_EXCLUDE_START
        Err(lookahead.error())
        // CODECOV_EXCLUDE_STOP
    }
}

fn parse_multiple(input: ParseStream) -> Result<Vec<CfgExpr>> {
    let mut vec = Vec::new();
    while !input.is_empty() {
        let expr = input.call(parse_single)?;
        vec.push(expr);
        if input.is_empty() {
            break;
        }
        input.parse::<Token![,]>()?;
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    use syn::{parse_quote, ItemMod};

    #[test]
    fn test_merge() {
        let mut cfg = CfgExpr::Unconditional;

        let module: ItemMod = parse_quote! {
            #[cfg(all(a, b))]
            #[cfg(not(a))]
            mod test;
        };
        let cfg_all = parse_attribute(&module.attrs[0]).unwrap();
        assert!(matches!(cfg_all, CfgExpr::All(ref items) if items.len() == 2));
        assert!(matches!(cfg, CfgExpr::Unconditional));

        // Merge all into unconditional
        cfg.merge(cfg_all.clone());
        assert!(matches!(cfg, CfgExpr::All(ref items) if items.len() == 2));

        // Merge all with all
        cfg.merge(cfg_all.clone());
        assert!(matches!(cfg, CfgExpr::All(ref items) if items.len() == 3));

        // Merge not with other
        let mut cfg_not = parse_attribute(&module.attrs[1]).unwrap();
        assert!(matches!(cfg_not, CfgExpr::Not(..)));
        cfg_not.merge(cfg_all);
        assert!(matches!(cfg_not, CfgExpr::All(items) if items.len() == 2));
    }

    #[test]
    fn test_parse_attribute() {
        let module: ItemMod = parse_quote! {
            #[cfg(a = "b")]
            #[cfg(a)]
            #[unknown]
            mod test;
        };
        let cfg_eq = parse_attribute(&module.attrs[0]).unwrap();
        assert!(matches!(cfg_eq, CfgExpr::Eq(.., Some(..))));

        let cfg_single = parse_attribute(&module.attrs[1]).unwrap();
        assert!(matches!(cfg_single, CfgExpr::Eq(.., None)));

        let cfg_unknown = parse_attribute(&module.attrs[2]);
        assert!(cfg_unknown.is_err());
    }

    #[test]
    fn test_parse_attribute_parenthesis() {
        let module: ItemMod = parse_quote! {
            #[cfg(all(a, b))]
            #[cfg(any(a, b))]
            #[cfg(not(a))]
            #[cfg(unknown(a))]
            mod test;
        };

        let cfg_all = parse_attribute(&module.attrs[0]).unwrap();
        assert!(matches!(cfg_all, CfgExpr::All(items) if items.len() == 2));

        let cfg_any = parse_attribute(&module.attrs[1]).unwrap();
        assert!(matches!(cfg_any, CfgExpr::Any(items) if items.len() == 2));

        let cfg_not = parse_attribute(&module.attrs[2]).unwrap();
        assert!(matches!(cfg_not, CfgExpr::Not(..)));

        let cfg_unknown = parse_attribute(&module.attrs[3]);
        assert!(cfg_unknown.is_err());
    }
}
