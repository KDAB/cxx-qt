use proc_macro::TokenStream;
use quote::*;
use syn::*;

use cxx_qt_gen::{extract_qobject, generate_qobject_cpp};

#[proc_macro_attribute]
pub fn make_qobject(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as ItemMod);

    let qobject;
    match extract_qobject(module) {
        Ok(o) => qobject = o,
        Err(e) => return e.into(),
    }

    // TODO: remove this print once the qobject is actually used
    println!("Parsed QObject: {:#?}", qobject);

    match generate_qobject_cpp(&qobject) {
        Ok(cpp_object) => {
            println!("Cpp Header: {}", cpp_object.header);
            println!("Cpp Source: {}", cpp_object.source);
        }
        Err(err) => return err.into(),
    }

    let expanded = quote! {
        // TODO: put something back :)
    };
    TokenStream::from(expanded)
}
