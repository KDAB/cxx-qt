use crate::{
    generator::naming::CombinedIdent,
    parser::parameter::ParsedFunctionParameter,
    syntax::{
        attribute::{attribute_tokens_to_ident, attribute_tokens_to_value},
        implitemmethod::is_method_mutable,
    },
};
use quote::format_ident;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, ForeignItemFn, LitStr, Result,
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
    /// the name of the function in Rust, as well as C++
    pub ident: CombinedIdent,
}

impl ParsedInheritedMethod {
    pub fn parse(method: ForeignItemFn) -> Result<Self> {
        let mutable = is_method_mutable(&method.sig);

        let parameters = ParsedFunctionParameter::parse_all_without_receiver(&method.sig)?;

        let mut ident = CombinedIdent::from_rust_function(method.sig.ident.clone());
        for attribute in &method.attrs {
            if !attribute.path.is_ident(&format_ident!("cxx_name")) {
                return Err(Error::new(
                    attribute.span(),
                    "Unsupported attribute in cxx_qt::inherit!",
                ));
            }

            let name = attribute_tokens_to_value::<LitStr>(attribute)?;

            ident.cpp = format_ident!("{}", name.value());
        }
        ident.cpp = format_ident!("{}_cxxqt_inherit", &ident.cpp);

        Ok(Self {
            method,
            mutable,
            parameters,
            ident,
        })
    }
}
