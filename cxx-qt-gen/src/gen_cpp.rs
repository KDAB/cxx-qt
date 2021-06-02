use convert_case::{Case, Casing};
use indoc::formatdoc;
use itertools::join;
use proc_macro2::TokenStream;
use std::collections::BTreeSet;
use std::result::Result;
use syn::*;

use crate::extract::{Invokable, Parameter, QObject};

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
    /// Whether this type is a reference
    fn is_ref(&self) -> bool;
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
            // We convert between a rust::String and QString so we can always use const QString&
            Self::String => Some("#include <QString>"),
        }
    }

    /// Whether this type is a reference
    fn is_ref(&self) -> bool {
        match self {
            Self::I32 => false,
            Self::String => true,
        }
    }

    /// The C++ type name of the CppType
    fn type_ident(&self) -> &'static str {
        match self {
            Self::I32 => "int",
            Self::String => "const QString",
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
                            "{type_ident} {is_ref}{ident}",
                            ident = parameter.ident,
                            is_ref = if parameter.type_ident.is_ref() {
                                "&"
                            } else {
                                ""
                            },
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

    use crate::extract_qobject;

    use pretty_assertions::assert_eq;
    use std::fs;

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
