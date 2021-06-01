use convert_case::{Case, Casing};
use derivative::*;
use indoc::formatdoc;
use itertools::join;
use proc_macro2::TokenStream;
use std::collections::BTreeSet;
use std::result::Result;
use syn::{spanned::Spanned, *};

/// Describes a function parameter
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
pub struct QObject {
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

/// Describes a result of a CppType which has been converted
#[derive(Debug)]
struct CppTypeConverted {
    /// The ident of the converted data
    ident: String,
    /// Whether this type recommends using std::move
    should_move: bool,
    /// The source which converts an input into data as ident
    source: String,
}

/// Describes a C++ type
#[derive(Debug)]
enum CppTypes {
    String,
    I32,
}

/// A trait which CppTypes implements allowing retrieval of attributes of the enum value.
trait CppType {
    /// Generates any conversion code for the CppType using the given ident
    fn convert(&self, ident: &str) -> Option<CppTypeConverted>;
    /// Any includes that are required for the CppType
    fn include(&self) -> Option<&'static str>;
    /// The C++ type name of the CppType
    fn type_ident(&self) -> &'static str;
}

impl CppType for CppTypes {
    /// Generates any conversion code for the CppType using the given ident
    fn convert(&self, ident: &str) -> Option<CppTypeConverted> {
        match self {
            Self::I32 => None,
            Self::String => {
                let rust_ident = format!("rust{}", ident.to_case(Case::Title));
                let source = format!(
                    "auto {rust_ident} = rust::string({ident}.toUtf8().data(), bytes.length());",
                    ident = ident,
                    rust_ident = rust_ident
                );
                Some(CppTypeConverted {
                    ident: rust_ident,
                    should_move: true,
                    source,
                })
            }
        }
    }

    /// Any includes that are required for the CppType
    fn include(&self) -> Option<&'static str> {
        match self {
            Self::I32 => None,
            Self::String => Some("#include <QString>"),
        }
    }

    /// The C++ type name of the CppType
    fn type_ident(&self) -> &'static str {
        match self {
            Self::I32 => "int",
            // We convert between a rust::String and QString so we can always use const QString&
            Self::String => "const QString&",
        }
    }
}

/// Describes a C++ parameter, which is a name combined with a type
#[derive(Debug)]
struct CppParameter {
    /// The ident of the parameter
    ident: String,
    /// The type of the parameter
    type_ident: CppTypes,
}

/// Describes a C++ invokable, with header, source, and include parts
#[derive(Debug)]
struct CppInvokable {
    /// The header definition of the invokable
    header: String,
    /// Any includes which this invokable requires
    includes: Vec<String>,
    /// The source implementation of the invokable
    source: String,
}

/// Describes a C++ header and source files of a C++ class
#[derive(Debug)]
pub struct CppObject {
    /// The header of the C++ class
    pub header: String,
    /// The source of the C++ class
    pub source: String,
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
            return Err(Error::new(item.span(), "Only methods are supported.").to_compile_error());
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
        invokables: object_invokables,
        properties: object_properties,
    })
}

/// Generate a C++ type for a given rust ident
fn generate_type_cpp(type_ident: &Ident) -> Result<CppTypes, TokenStream> {
    match type_ident.to_string().as_str() {
        "str" => Ok(CppTypes::String),
        "i32" => Ok(CppTypes::I32),
        _other => Err(
            Error::new(type_ident.span(), "Unknown type ident to convert to C++.")
                .to_compile_error(),
        ),
    }
}

/// Generate a string of parameters with their type in C++ style from a given list of rust parameters
fn generate_parameters_cpp(parameters: &[Parameter]) -> Result<Vec<CppParameter>, TokenStream> {
    let mut items: Vec<CppParameter> = vec![];

    for parameter in parameters {
        items.push(CppParameter {
            ident: parameter.ident.to_string(),
            type_ident: generate_type_cpp(&parameter.type_ident)?,
        });
    }

    Ok(items)
}

/// Generate a CppInvokable object containing the header and source of a given list of rust invokables
fn generate_invokables_cpp(
    struct_ident: &Ident,
    invokables: &[Invokable],
) -> Result<Vec<CppInvokable>, TokenStream> {
    let mut items: Vec<CppInvokable> = vec![];

    if !invokables.is_empty() {
        // A helper which allows us to flatten data from vec of parameters
        struct CppParameterHelper {
            args: Vec<String>,
            converters: Vec<String>,
            names: Vec<String>,
            includes: Vec<String>,
        }

        for invokable in invokables {
            // Query for parameters and flatten them into a helper
            let parameters = generate_parameters_cpp(&invokable.parameters)?
                .drain(..)
                .fold(
                    CppParameterHelper {
                        args: vec![],
                        converters: vec![],
                        names: vec![],
                        includes: vec![],
                    },
                    |mut acc, parameter| {
                        // Build the parameter as a type argument
                        acc.args.push(format!(
                            "{type_ident} {ident}",
                            ident = parameter.ident,
                            type_ident = parameter.type_ident.type_ident()
                        ));
                        // If there is a converter then build our converter and new name
                        if let Some(converted) = parameter.type_ident.convert(&parameter.ident) {
                            acc.converters.push(converted.source);
                            // Consider if the ident needs to be moved
                            if converted.should_move {
                                acc.names.push(format!("std::move({})", converted.ident));
                            } else {
                                acc.names.push(converted.ident);
                            }
                        } else {
                            // No converter so use the same name
                            acc.names.push(parameter.ident);
                        }
                        // See if there are any includes for the type
                        if let Some(include) = parameter.type_ident.include() {
                            acc.includes.push(include.to_owned());
                        }
                        acc
                    },
                );
            let parameter_arg_line = parameters.args.join(", ");

            // Prepare the CppInvokable
            items.push(CppInvokable {
                // TODO: detect if method is const from whether we have &self or &mut self in rust
                header: format!(
                    "Q_INVOKABLE void {ident}({parameter_types}) const;",
                    ident = invokable.ident.to_string(),
                    parameter_types = parameter_arg_line
                ),
                source: formatdoc!(
                    r#"
                    void {struct_ident}::{ident}({parameter_types}) const
                    {{
                        {converters}
                        m_rustObj->{ident}({parameter_names});
                    }}"#,
                    // TODO: if converters is empty, we'll get an extra empty line
                    // can we remove this?
                    converters = parameters.converters.join("\n"),
                    ident = invokable.ident.to_string(),
                    parameter_names = parameters.names.join(", "),
                    parameter_types = parameter_arg_line,
                    struct_ident = struct_ident.to_string(),
                ),
                includes: parameters.includes,
            });
        }
    }

    Ok(items)
}

/// Generate a CppObject object containing the header and source of a given rust QObject
pub fn generate_qobject_cpp(obj: &QObject) -> Result<CppObject, TokenStream> {
    let mut generic_includes: BTreeSet<String> = BTreeSet::new();
    generic_includes.insert("#include <QObject>".to_owned());
    let rust_suffix = "Rs";
    let struct_ident_str = obj.ident.to_string();

    // A helper which allows us to flatten data from vec of invokables
    struct CppInvokableHelper {
        headers: Vec<String>,
        includes: BTreeSet<String>,
        sources: Vec<String>,
    }

    // Query for invokables and flatten them into a helper
    let invokables = generate_invokables_cpp(&obj.ident, &obj.invokables)?
        .drain(..)
        .fold(
            CppInvokableHelper {
                headers: vec![],
                includes: BTreeSet::new(),
                sources: vec![],
            },
            |mut acc, invokable| {
                acc.headers.push(invokable.header);
                acc.includes.extend(invokable.includes);
                acc.sources.push(invokable.source);
                acc
            },
        );

    // Generate C++ header part
    let header = formatdoc! {r#"
        #pragma once

        {includes}

        #include "rust/cxx.h"

        class {ident}{rust_suffix};

        class {ident} : public QObject {{
            Q_OBJECT

        public:
            {ident}(QObject *parent = nullptr);
            ~{ident}();

            {invokables}

        private:
            rust::Box<{ident}{rust_suffix}> m_rustObj;
        }};

        std::unique_ptr<{ident}> new_{ident}();
        "#,
    ident = struct_ident_str,
    // TODO: merge with property includes
    includes = join(generic_includes.union(&invokables.includes), "\n"),
    invokables = invokables.headers.join("\n"),
    rust_suffix = rust_suffix
    };

    // Generate C++ source part
    let source = formatdoc! {r#"
        #include "cxx-qt-gen/include/{ident_snake}.h"
        #include "cxx-qt-gen/src/{ident_snake}.rs.h"

        {ident}::{ident}(QObject *parent)
            : QObject(parent)
            , m_rustObj(create_{ident_snake}_rs())
        {{
        }}

        {ident}::~{ident}() = default;

        {invokables}

        std::unique_ptr<{ident}> new_{ident}()
        {{
            return std::make_unique<{ident}>();
        }}
        "#,
        ident = struct_ident_str,
        ident_snake = struct_ident_str.to_case(Case::Snake),
        invokables = invokables.sources.join("\n")
    };

    Ok(CppObject { header, source })
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use std::fs;

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

    #[test]
    fn generates_basic_only_invokables() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_header = fs::read_to_string("test_outputs/basic_only_invokable.h").unwrap();
        let expected_source = fs::read_to_string("test_outputs/basic_only_invokable.cpp").unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }
}
