use derivative::*;
use proc_macro::TokenStream;
use quote::*;
use syn::{spanned::Spanned, *};

/// Describes a function paramter
#[derive(Debug)]
struct Parameter {
    /// The ident of the parameter
    ident: Ident,
    /// The type of the parameter
    type_ident: Ident,
}

/// Describes a function that can be invoked from QML
#[derive(Derivative)]
#[derivative(Debug)]
struct Invokable {
    /// The ident of the function
    ident: Ident,
    /// The parameters that the function takes in
    parameters: Vec<Parameter>,
    /// The original Rust method for the invokable
    #[derivative(Debug = "ignore")]
    _original_method: ImplItemMethod,
}

/// Describes all the properties of a QObject class
#[derive(Debug)]
struct QObject {
    /// The ident of the Rust module that represents the QObject
    module_ident: Ident,
    /// The ident of the original struct and name of the C++ class that represents the QObject
    ident: Ident,
    /// All the methods that can be invoked from QML
    invokables: Vec<Invokable>,
}

/// Extracts all the member functions from a module and generates invokables from them
fn extract_invokables(items: &[ImplItem]) -> std::result::Result<Vec<Invokable>, TokenStream> {
    let mut invokables = Vec::new();

    for item in items {
        let method;
        if let ImplItem::Method(m) = item {
            method = m;
        } else {
            return Err(Error::new(item.span(), "Only methods are supported.")
                .to_compile_error()
                .into());
        }

        let method_ident = &method.sig.ident;
        let inputs = &method.sig.inputs;
        let mut parameters = Vec::new();

        for arg in inputs {
            if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
                let arg_ident;
                let _arg_by_ref;
                let type_ident;

                if let Pat::Ident(PatIdent { ident, by_ref, .. }) = &**pat {
                    arg_ident = ident;
                    _arg_by_ref = by_ref;
                } else {
                    return Err(Error::new(arg.span(), "Invalid argument ident format")
                        .to_compile_error()
                        .into());
                }

                let ty_path;

                match &**ty {
                    Type::Path(path) => {
                        ty_path = path;
                    }
                    Type::Reference(TypeReference { elem, .. }) => {
                        if let Type::Path(path) = &**elem {
                            ty_path = path;
                        } else {
                            return Err(Error::new(arg.span(), "Invalid argument type format")
                                .to_compile_error()
                                .into());
                        }
                    }
                    _others => {
                        return Err(Error::new(arg.span(), "Invalid argument type format")
                            .to_compile_error()
                            .into());
                    }
                }

                let segments = &ty_path.path.segments;

                if segments.len() != 1 {
                    return Err(
                        Error::new(arg.span(), "Argument should only have one segment.")
                            .to_compile_error()
                            .into(),
                    );
                }

                type_ident = &segments[0].ident;

                // TODO: we probably need to track if parameters are by reference two
                let parameter = Parameter {
                    ident: arg_ident.to_owned(),
                    type_ident: type_ident.to_owned(),
                };
                parameters.push(parameter);
            }
        }

        let invokable = Invokable {
            ident: method_ident.to_owned(),
            parameters,
            _original_method: method.to_owned(),
        };
        invokables.push(invokable);
    }

    Ok(invokables)
}

#[proc_macro_attribute]
pub fn make_qobject(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as ItemMod);

    let module_ident = &module.ident;

    let items = &module
        .content
        .expect("Incorrect module format encountered.")
        .1;
    if items.is_empty() {
        panic!("Empty modules are not supported.");
    }

    let original_struct;
    if let Item::Struct(s) = &items[0] {
        original_struct = s;
    } else {
        panic!("The first item in the module needs to be a struct with the name of the C++ class.");
    }
    let struct_ident = &original_struct.ident;

    let object_invokables;

    match items.len() {
        1 => {
            // If there is only a struct then there are no invokables
            object_invokables = vec![];
        }
        2 => {
            let original_impl;
            if let Item::Impl(i) = &items[1] {
                original_impl = i;
            } else {
                panic!("If the module has a second item, it has to be a struct.");
            }

            if let Type::Path(TypePath { path, .. }) = &*original_impl.self_ty {
                if path.segments.len() != 1 {
                    panic!("Invalid path on impl block.");
                }

                let impl_ident = &path.segments[0].ident;
                if *impl_ident != *struct_ident {
                    return Error::new(
                        impl_ident.span(),
                        "The impl block needs to match the struct.",
                    )
                    .to_compile_error()
                    .into();
                }
            }

            let invokables = extract_invokables(&original_impl.items);

            match invokables {
                Err(err) => return err,
                Ok(i) => object_invokables = i,
            }
        }
        _other => {
            panic!("The module can only contain 1 struct and optionally an impl on that struct.");
        }
    }

    let qobject = QObject {
        module_ident: module_ident.to_owned(),
        ident: struct_ident.to_owned(),
        invokables: object_invokables,
    };

    // TODO: remove this print once the qobject is actually used
    println!("Parsed QObject: {:#?}", qobject);

    let expanded = quote! {
        // TODO: put something back :)
    };
    TokenStream::from(expanded)
}
