// SPDX-FileCopyrightText: CXX Authors
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: David Tolnay <dtolnay@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::cfg::{parse_attribute, CfgExpr};
use cxx_gen::{CfgEvaluator, CfgResult};
use quote::quote;
use syn::{Attribute, Error, LitStr};

pub(crate) fn try_eval_attributes(
    cfg_evaluator: &dyn CfgEvaluator,
    attrs: &[Attribute],
) -> Result<bool, Error> {
    // Build a single CfgExpr from the Attributes
    let cfg_expr = attrs
        .iter()
        .map(parse_attribute)
        .collect::<Result<Vec<CfgExpr>, Error>>()?
        .into_iter()
        .reduce(|mut acc, e| {
            acc.merge(e);
            acc
        });

    // Evaluate the CfgExpr against the CfgEvaluator
    if let Some(cfg_expr) = cfg_expr {
        try_eval(cfg_evaluator, &cfg_expr).map_err(|errs| {
            errs.into_iter()
                .reduce(|mut acc, e| {
                    acc.combine(e);
                    acc
                })
                .expect("There should be at least one error")
        })
    } else {
        Ok(true)
    }
}

fn try_eval(cfg_evaluator: &dyn CfgEvaluator, expr: &CfgExpr) -> Result<bool, Vec<Error>> {
    match expr {
        CfgExpr::Unconditional => Ok(true),
        CfgExpr::Eq(ident, string) => {
            let key = ident.to_string();
            let value = string.as_ref().map(LitStr::value);
            match cfg_evaluator.eval(&key, value.as_deref()) {
                CfgResult::True => Ok(true),
                CfgResult::False => Ok(false),
                CfgResult::Undetermined { msg } => {
                    let span = quote!(#ident #string);
                    Err(vec![Error::new_spanned(span, msg)])
                }
            }
        }
        CfgExpr::All(list) => {
            let mut all_errors = Vec::new();
            for subexpr in list {
                match try_eval(cfg_evaluator, subexpr) {
                    Ok(true) => {}
                    Ok(false) => return Ok(false),
                    Err(errors) => all_errors.extend(errors),
                }
            }
            if all_errors.is_empty() {
                Ok(true)
            } else {
                Err(all_errors)
            }
        }
        CfgExpr::Any(list) => {
            let mut all_errors = Vec::new();
            for subexpr in list {
                match try_eval(cfg_evaluator, subexpr) {
                    Ok(true) => return Ok(true),
                    Ok(false) => {}
                    Err(errors) => all_errors.extend(errors),
                }
            }
            if all_errors.is_empty() {
                Ok(false)
            } else {
                Err(all_errors)
            }
        }
        CfgExpr::Not(subexpr) => match try_eval(cfg_evaluator, subexpr) {
            Ok(value) => Ok(!value),
            Err(errors) => Err(errors),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{generator::UnsupportedCfgEvaluator, tests::CfgEvaluatorTest};
    use syn::{parse_quote, ItemMod};

    fn assert_eval_insert(module: ItemMod, cfgs: &[&str], [before, after]: [bool; 2]) {
        let mut cfg_evaluator = Box::new(CfgEvaluatorTest::default());
        assert_eq!(
            try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
            before
        );

        for chunk in cfgs.chunks(2) {
            assert_eq!(
                try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
                before
            );

            if let [key, value] = chunk {
                cfg_evaluator.cfgs.insert(key, Some(value));
            }
        }

        assert_eq!(
            try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs).unwrap(),
            after
        );
    }

    fn assert_eval_insert_false_true(module: ItemMod, cfgs: &[&str]) {
        assert_eval_insert(module, cfgs, [false, true]);
    }

    #[test]
    fn test_try_eval_attributes_eq() {
        assert_eval_insert_false_true(
            parse_quote! {
                #[cfg(a = "1")]
                #[cfg(b = "2")]
                mod module;
            },
            &["c", "3", "a", "1", "b", "2"],
        );
    }

    #[test]
    fn test_try_eval_attributes_any() {
        assert_eval_insert_false_true(
            parse_quote! {
                #[cfg(any(a = "1", b = "2"))]
                mod module;
            },
            &["c", "3", "a", "1"],
        );
    }

    #[test]
    fn test_try_eval_attributes_all() {
        assert_eval_insert_false_true(
            parse_quote! {
                #[cfg(all(a = "1", b = "2"))]
                mod module;
            },
            &["c", "3", "a", "1", "b", "2"],
        );
    }

    #[test]
    fn test_try_eval_attributes_not() {
        assert_eval_insert(
            parse_quote! {
                #[cfg(not(a = "1"))]
                mod module;
            },
            &["c", "3", "a", "1"],
            [true, false],
        );
    }

    #[test]
    fn test_try_eval_unconditional() {
        let cfg_expr = CfgExpr::Unconditional;
        let cfg_evaluator = Box::new(UnsupportedCfgEvaluator);
        assert_eq!(try_eval(cfg_evaluator.as_ref(), &cfg_expr).unwrap(), true);
    }

    #[test]
    fn test_try_eval_attributes_undetermined_err() {
        let module: ItemMod = parse_quote! {
            #[cfg(a = "1")]
            #[cfg(all(a = "1", b = "2"))]
            #[cfg(any(a = "1", b = "2"))]
            #[cfg(not(a = "1"))]
            mod module;
        };
        let cfg_evaluator = Box::new(UnsupportedCfgEvaluator);
        assert!(try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs[0..1]).is_err());
        assert!(try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs[1..2]).is_err());
        assert!(try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs[2..3]).is_err());
        assert!(try_eval_attributes(cfg_evaluator.as_ref(), &module.attrs[3..4]).is_err());
    }
}
