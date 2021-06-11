use convert_case::{Case, Casing};
use derivative::*;
use proc_macro2::TokenStream;
use std::result::Result;
use syn::{spanned::Spanned, *};

/// Describes an ident which has a different name in C++ and Rust
#[derive(Debug)]
pub(crate) struct CppRustIdent {
    /// The ident for C++
    pub(crate) cpp_ident: Ident,
    /// The ident for rust
    pub(crate) rust_ident: Ident,
}

/// Describes a type
#[derive(Debug)]
pub(crate) struct ParameterType {
    /// The type of the parameter
    pub(crate) ident: Ident,
    /// If this parameter is a reference
    pub(crate) is_ref: bool,
}

/// Describes a function parameter
#[derive(Debug)]
pub(crate) struct Parameter {
    /// The ident of the parameter
    pub(crate) ident: Ident,
    /// The type of the parameter
    pub(crate) type_ident: ParameterType,
}

/// Describes a function that can be invoked from QML
#[derive(Derivative)]
#[derivative(Debug)]
pub(crate) struct Invokable {
    /// The ident of the function
    pub(crate) ident: Ident,
    /// The parameters that the function takes in
    pub(crate) parameters: Vec<Parameter>,
    /// The return type information
    pub(crate) return_type: Option<ParameterType>,
    /// The original Rust method for the invokable
    #[derivative(Debug = "ignore")]
    pub(crate) original_method: ImplItemMethod,
}

/// Describes a property that can be used from QML
#[derive(Debug)]
pub(crate) struct Property {
    /// The ident of the property
    pub(crate) ident: Ident,
    /// The type of the property
    pub(crate) type_ident: ParameterType,
    /// The getter ident of the property (used for READ)
    pub(crate) getter: Option<CppRustIdent>,
    /// The setter ident of the property (used for WRITE)
    pub(crate) setter: Option<CppRustIdent>,
    /// The notify ident of the property (used for NOTIFY)
    pub(crate) notify: Option<CppRustIdent>,
    // TODO: later we will further possibilities such as CONSTANT or FINAL
}

/// Describes all the properties of a QObject class
#[derive(Debug)]
pub struct QObject {
    /// The ident of the Rust module that represents the QObject
    pub(crate) module_ident: Ident,
    /// The ident of the original struct and name of the C++ class that represents the QObject
    pub ident: Ident,
    /// The ident of the new Rust struct that will be generated and will form the internals of the QObject
    pub(crate) rust_struct_ident: Ident,
    /// All the methods that can be invoked from QML
    pub(crate) invokables: Vec<Invokable>,
    /// All the properties that can be used from QML
    pub(crate) properties: Vec<Property>,
    /// The original Rust struct that the object was generated from
    pub(crate) original_struct: ItemStruct,
}

/// Describe the error type from extract_type_ident
enum ExtractTypeIdentError {
    InvalidSegments,
    InvalidType,
}

/// Extract the type ident from a given syn::Type
fn extract_type_ident(ty: &syn::Type) -> Result<ParameterType, ExtractTypeIdentError> {
    let ty_path;
    let is_ref;

    match ty {
        Type::Path(path) => {
            is_ref = false;
            ty_path = path;
        }
        Type::Reference(TypeReference { elem, .. }) => {
            if let Type::Path(path) = &**elem {
                is_ref = true;
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

    Ok(ParameterType {
        ident: segments[0].ident.to_owned(),
        is_ref,
    })
}

/// Extracts all the member functions from a module and generates invokables from them
fn extract_invokables(items: &[ImplItem]) -> Result<Vec<Invokable>, TokenStream> {
    let mut invokables = Vec::new();

    for item in items {
        let method;
        if let ImplItem::Method(m) = item {
            method = m;
        } else {
            return Err(Error::new(item.span(), "Only methods are supported.").to_compile_error());
        }

        let method_ident = &method.sig.ident;
        let inputs = &method.sig.inputs;
        let output = &method.sig.output;
        let mut parameters = Vec::new();

        for arg in inputs {
            if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
                let arg_ident;
                let type_ident;

                if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                    arg_ident = ident;
                } else {
                    return Err(
                        Error::new(arg.span(), "Invalid argument ident format.").to_compile_error()
                    );
                }

                match extract_type_ident(ty) {
                    Ok(result) => type_ident = result,
                    Err(ExtractTypeIdentError::InvalidType) => {
                        return Err(Error::new(arg.span(), "Invalid argument ident format.")
                            .to_compile_error())
                    }
                    Err(ExtractTypeIdentError::InvalidSegments) => {
                        return Err(Error::new(
                            arg.span(),
                            "Argument should only have one segment.",
                        )
                        .to_compile_error());
                    }
                }

                let parameter = Parameter {
                    ident: arg_ident.to_owned(),
                    type_ident,
                };
                parameters.push(parameter);
            }
        }

        let return_type = if let ReturnType::Type(_, ty) = output {
            match extract_type_ident(ty) {
                Ok(result) => Some(result),
                Err(ExtractTypeIdentError::InvalidType) => {
                    return Err(
                        Error::new(output.span(), "Invalid return type format.").to_compile_error()
                    )
                }
                Err(ExtractTypeIdentError::InvalidSegments) => {
                    return Err(Error::new(
                        output.span(),
                        "Return type should only have one segment.",
                    )
                    .to_compile_error());
                }
            }
        } else {
            None
        };

        let invokable = Invokable {
            ident: method_ident.to_owned(),
            parameters,
            return_type,
            original_method: method.to_owned(),
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
                            .to_compile_error())
                    }
                    Err(ExtractTypeIdentError::InvalidSegments) => {
                        return Err(Error::new(
                            name.span(),
                            "Named field should only have one segment.",
                        )
                        .to_compile_error());
                    }
                }

                // Build the getter/setter/notify idents
                //
                // TODO: later these can be optional and have custom names from macro attributes
                // we might also need to store whether a custom method is already implemented or
                // whether a method needs to be auto generated on the rust side
                let ident_str = ident.to_string();
                let getter = Some(CppRustIdent {
                    cpp_ident: quote::format_ident!("get{}", ident_str.to_case(Case::Title)),
                    rust_ident: quote::format_ident!("{}", ident_str.to_case(Case::Snake)),
                });
                let setter = Some(CppRustIdent {
                    cpp_ident: quote::format_ident!("set{}", ident_str.to_case(Case::Title)),
                    rust_ident: quote::format_ident!("set_{}", ident_str.to_case(Case::Snake)),
                });
                let notify = Some(CppRustIdent {
                    cpp_ident: quote::format_ident!("{}Changed", ident_str.to_case(Case::Camel)),
                    // TODO: rust doesn't have notify on it's side?
                    rust_ident: quote::format_ident!("{}", ident_str.to_case(Case::Snake)),
                });

                properties.push(Property {
                    ident: ident.to_owned(),
                    type_ident,
                    getter,
                    setter,
                    notify,
                });
            }
        }
    }

    Ok(properties)
}

/// Parses a module in order to extract a QObject description from it
pub fn extract_qobject(module: ItemMod) -> Result<QObject, TokenStream> {
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

    const RUST_SUFFIX: &str = "Rs";
    let rust_struct_ident = quote::format_ident!("{}{}", struct_ident, RUST_SUFFIX);

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
                    .to_compile_error());
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
        rust_struct_ident,
        invokables: object_invokables,
        properties: object_properties,
        original_struct: original_struct.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

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
        assert_eq!(qobject.invokables.len(), 2);
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
        assert_eq!(qobject.invokables.len(), 2);

        // Check invokable ident
        let invokable = &qobject.invokables[0];
        assert_eq!(invokable.ident.to_string(), "say_hi");

        // Check invokable parameters ident and type ident
        assert_eq!(invokable.parameters.len(), 2);

        let param_first = &invokable.parameters[0];
        assert_eq!(param_first.ident.to_string(), "string");
        // TODO: add extra checks when we read if this is a mut or not
        assert_eq!(param_first.type_ident.ident.to_string(), "str");
        assert_eq!(param_first.type_ident.is_ref, true);

        let param_second = &invokable.parameters[1];
        assert_eq!(param_second.ident.to_string(), "number");
        assert_eq!(param_second.type_ident.ident.to_string(), "i32");
        assert_eq!(param_second.type_ident.is_ref, false);

        // Check invokable ident
        let invokable_second = &qobject.invokables[1];
        assert_eq!(invokable_second.ident.to_string(), "say_bye");

        // Check invokable parameters ident and type ident
        assert_eq!(invokable_second.parameters.len(), 0);
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

        // Check first property
        let prop_first = &qobject.properties[0];
        assert_eq!(prop_first.ident.to_string(), "number");
        assert_eq!(prop_first.type_ident.ident.to_string(), "i32");
        assert_eq!(prop_first.type_ident.is_ref, false);

        assert_eq!(prop_first.getter.is_some(), true);
        let getter = prop_first.getter.as_ref().unwrap();
        assert_eq!(getter.cpp_ident.to_string(), "getNumber");
        assert_eq!(getter.rust_ident.to_string(), "number");

        assert_eq!(prop_first.setter.is_some(), true);
        let setter = prop_first.setter.as_ref().unwrap();
        assert_eq!(setter.cpp_ident.to_string(), "setNumber");
        assert_eq!(setter.rust_ident.to_string(), "set_number");

        assert_eq!(prop_first.notify.is_some(), true);
        let notify = prop_first.notify.as_ref().unwrap();
        assert_eq!(notify.cpp_ident.to_string(), "numberChanged");
        // TODO: does rust need a notify ident?
        assert_eq!(notify.rust_ident.to_string(), "number");

        // Check second property
        let prop_second = &qobject.properties[1];
        assert_eq!(prop_second.ident.to_string(), "string");
        assert_eq!(prop_second.type_ident.ident.to_string(), "String");
        assert_eq!(prop_second.type_ident.is_ref, false);

        assert_eq!(prop_second.getter.is_some(), true);
        let getter = prop_second.getter.as_ref().unwrap();
        assert_eq!(getter.cpp_ident.to_string(), "getString");
        assert_eq!(getter.rust_ident.to_string(), "string");

        assert_eq!(prop_second.setter.is_some(), true);
        let setter = prop_second.setter.as_ref().unwrap();
        assert_eq!(setter.cpp_ident.to_string(), "setString");
        assert_eq!(setter.rust_ident.to_string(), "set_string");

        assert_eq!(prop_second.notify.is_some(), true);
        let notify = prop_second.notify.as_ref().unwrap();
        assert_eq!(notify.cpp_ident.to_string(), "stringChanged");
        // TODO: does rust need a notify ident?
        assert_eq!(notify.rust_ident.to_string(), "string");
    }
}
