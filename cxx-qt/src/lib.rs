use derivative::*;
use proc_macro::TokenStream;
use quote::*;
use std::result::Result;
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

/// Describes a property that can be used from QML
#[derive(Debug)]
struct Property {
    /// The ident of the property
    ident: Ident,
    /// The type of the property
    type_ident: Ident,
    // TODO: later we will have possibility for custom setter, getter, notify, constant etc
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
    /// All the properties that can be used from QML
    properties: Vec<Property>,
}

/// Describe the error type from extract_type_ident
enum ExtractTypeIdentError {
    InvalidSegments,
    InvalidType,
}

/// Extract the type ident from a given syn::Type
fn extract_type_ident(ty: &syn::Type) -> Result<Ident, ExtractTypeIdentError> {
    let ty_path;

    match ty {
        Type::Path(path) => {
            ty_path = path;
        }
        Type::Reference(TypeReference { elem, .. }) => {
            if let Type::Path(path) = &**elem {
                ty_path = path;
            } else {
                return Err(ExtractTypeIdentError::InvalidType);
            }
        }
        _others => {
            return Err(ExtractTypeIdentError::InvalidType);
        }
    }

    let segments = &ty_path.path.segments;
    if segments.len() != 1 {
        return Err(ExtractTypeIdentError::InvalidSegments);
    }

    Ok(segments[0].ident.to_owned())
}

/// Extracts all the member functions from a module and generates invokables from them
fn extract_invokables(items: &[ImplItem]) -> Result<Vec<Invokable>, TokenStream> {
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
                    return Err(Error::new(arg.span(), "Invalid argument ident format.")
                        .to_compile_error()
                        .into());
                }

                match extract_type_ident(ty) {
                    Ok(result) => type_ident = result,
                    Err(ExtractTypeIdentError::InvalidType) => {
                        return Err(Error::new(arg.span(), "Invalid argument ident format.")
                            .to_compile_error()
                            .into())
                    }
                    Err(ExtractTypeIdentError::InvalidSegments) => {
                        return Err(Error::new(
                            arg.span(),
                            "Argument should only have one segment.",
                        )
                        .to_compile_error()
                        .into());
                    }
                }

                // TODO: we probably need to track if parameters are by reference two
                let parameter = Parameter {
                    ident: arg_ident.to_owned(),
                    type_ident,
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

/// Extracts all the attributes from a struct and generates properties from them
fn extract_properties(s: &ItemStruct) -> Result<Vec<Property>, TokenStream> {
    let mut properties = Vec::new();

    // Read the properties from the struct
    if let ItemStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    } = s
    {
        for name in named {
            if let Field {
                // TODO: later we'll need to read the attributes (eg qt_property) here
                // attrs,
                ident: Some(ident),
                ty,
                ..
            } = name
            {
                let type_ident;

                match extract_type_ident(ty) {
                    Ok(result) => type_ident = result,
                    Err(ExtractTypeIdentError::InvalidType) => {
                        return Err(Error::new(name.span(), "Invalid name field ident format.")
                            .to_compile_error()
                            .into())
                    }
                    Err(ExtractTypeIdentError::InvalidSegments) => {
                        return Err(Error::new(
                            name.span(),
                            "Named field should only have one segment.",
                        )
                        .to_compile_error()
                        .into());
                    }
                }

                // TODO: read attrs to see if there are any non default qt_property options
                properties.push(Property {
                    ident: ident.to_owned(),
                    type_ident,
                });
            }
        }
    }

    Ok(properties)
}

/// Parses a module in order to extract a QObject description from it
fn extract_qobject(module: ItemMod) -> Result<QObject, TokenStream> {
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
    let object_properties;

    // Read properties from the struct
    match extract_properties(original_struct) {
        Err(err) => return Err(err),
        Ok(properties) => object_properties = properties,
    }

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
                    return Err(Error::new(
                        impl_ident.span(),
                        "The impl block needs to match the struct.",
                    )
                    .to_compile_error()
                    .into());
                }
            }

            let invokables = extract_invokables(&original_impl.items);

            match invokables {
                Err(err) => return Err(err),
                Ok(i) => object_invokables = i,
            }
        }
        _other => {
            panic!("The module can only contain 1 struct and optionally an impl on that struct.");
        }
    }

    Ok(QObject {
        module_ident: module_ident.to_owned(),
        ident: struct_ident.to_owned(),
        invokables: object_invokables,
        properties: object_properties,
    })
}

#[proc_macro_attribute]
pub fn make_qobject(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as ItemMod);

    let qobject;
    match extract_qobject(module) {
        Ok(o) => qobject = o,
        Err(e) => return e,
    }

    // TODO: remove this print once the qobject is actually used
    println!("Parsed QObject: {:#?}", qobject);

    let expanded = quote! {
        // TODO: put something back :)
    };
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_invokable_and_properties() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_invokable_and_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        // Check that it got the invokables and properties
        // We only check the counts as the only_invokables and only_properties
        // will test more than the number.
        assert_eq!(qobject.invokables.len(), 1);
        assert_eq!(qobject.properties.len(), 2);
    }

    #[test]
    fn parses_basic_only_invokable() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        // Check that it got the names right
        assert_eq!(qobject.ident.to_string(), "MyObject");
        assert_eq!(qobject.module_ident.to_string(), "my_object");

        // Check that it got the invokables
        assert_eq!(qobject.invokables.len(), 1);

        // Check invokable ident
        let invokable = &qobject.invokables[0];
        assert_eq!(invokable.ident.to_string(), "say_hi");

        // Check invokable parameters ident and type ident
        assert_eq!(invokable.parameters.len(), 2);

        let param_first = &invokable.parameters[0];
        assert_eq!(param_first.ident.to_string(), "string");
        // TODO: add extra checks when we read if this is a & or &mut
        // eg this would need to also check an is_reference field
        assert_eq!(param_first.type_ident.to_string(), "str");

        let param_second = &invokable.parameters[1];
        assert_eq!(param_second.ident.to_string(), "number");
        assert_eq!(param_second.type_ident.to_string(), "i32");
    }

    #[test]
    fn parses_basic_only_properties() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        // Check that it got the properties and that the idents are correct
        assert_eq!(qobject.properties.len(), 2);

        let prop_first = &qobject.properties[0];
        assert_eq!(prop_first.ident.to_string(), "string");
        assert_eq!(prop_first.type_ident.to_string(), "String");

        let prop_second = &qobject.properties[1];
        assert_eq!(prop_second.ident.to_string(), "number");
        assert_eq!(prop_second.type_ident.to_string(), "i32");
    }
}
