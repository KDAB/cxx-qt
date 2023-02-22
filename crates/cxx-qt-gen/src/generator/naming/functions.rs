use convert_case::{Case, Casing};
use quote::format_ident;
use syn::Ident;

use super::*;

impl CombinedIdent {
    /// Generate a CombinedIdent from a rust function name.
    /// C++ will use the CamelCase version of the function name.
    pub fn from_rust_function(ident: Ident) -> Self {
        Self {
            cpp: format_ident!("{}", ident.to_string().to_case(Case::Camel)),
            rust: ident,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_rust_function_camel_case_conversion() {
        let ident = format_ident!("test_function");
        let combined = CombinedIdent::from_rust_function(ident.clone());
        assert_eq!(combined.cpp, format_ident!("testFunction"));
        assert_eq!(combined.rust, ident);
    }

    #[test]
    fn test_from_rust_function_single_word() {
        let ident = format_ident!("test");
        let combined = CombinedIdent::from_rust_function(ident.clone());
        assert_eq!(combined.cpp, ident);
        assert_eq!(combined.rust, ident);
    }
}
