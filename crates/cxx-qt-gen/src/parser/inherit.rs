use crate::{
    parser::parameter::ParsedFunctionParameter, syntax::implitemmethod::is_method_mutable,
};
use syn::{
    parse::{Parse, ParseStream},
    ForeignItemFn, Result,
};

/// This type is used when parsing the `cxx_qt::inherit!` macro contents into raw ForeignItemFn items
pub struct InheritMethods {
    pub base_functions: Vec<ForeignItemFn>,
}

impl Parse for InheritMethods {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut base_functions = Vec::new();
        while !input.is_empty() {
            base_functions.push(input.parse::<ForeignItemFn>()?);
        }
        Ok(InheritMethods { base_functions })
    }
}

/// Describes a method found in cxx_qt::inherit!
pub struct ParsedInheritedMethod {
    /// The original [syn::ForeignItemFn] of the inherited method declaration
    pub method: ForeignItemFn,
    /// whether the inherited method is marked as mutable
    pub mutable: bool,
    /// the parameters of the method, without the `self` argument
    pub parameters: Vec<ParsedFunctionParameter>,
}

impl ParsedInheritedMethod {
    pub fn parse(method: ForeignItemFn) -> Result<Self> {
        let mutable = is_method_mutable(&method.sig);

        let parameters = ParsedFunctionParameter::parse_all_without_receiver(&method.sig)?;

        Ok(Self {
            method,
            mutable,
            parameters,
        })
    }
}
