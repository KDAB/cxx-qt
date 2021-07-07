// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use clang_format::{clang_format, ClangFormatStyle, CLANG_FORMAT_STYLE};
use convert_case::{Case, Casing};
use indoc::formatdoc;
use proc_macro2::TokenStream;
use std::result::Result;
use syn::*;

use crate::extract::{Invokable, Parameter, ParameterType, Property, QObject};

/// Describes a C++ type
#[derive(Debug)]
enum CppTypes {
    String,
    Str,
    I32,
}

/// A trait which CppTypes implements allowing retrieval of attributes of the enum value.
trait CppType {
    /// Any converter that is required to convert this type into C++
    fn convert_into_cpp(&self) -> Option<&'static str>;
    /// Any converter that is required to convert this type into rust
    fn convert_into_rust(&self) -> Option<&'static str>;
    /// Whether this type is a const (when used as an input to methods)
    fn is_const(&self) -> bool;
    /// Whether this type is a reference
    fn is_ref(&self) -> bool;
    /// The C++ type name of the CppType
    fn type_ident(&self) -> &'static str;
}

impl CppType for CppTypes {
    /// Any converter that is required to convert this type into C++
    fn convert_into_cpp(&self) -> Option<&'static str> {
        match self {
            Self::I32 => None,
            Self::Str => Some("rustStrToQString"),
            Self::String => Some("rustStringToQString"),
        }
    }

    /// Any converter that is required to convert this type into rust
    fn convert_into_rust(&self) -> Option<&'static str> {
        match self {
            Self::I32 => None,
            Self::Str => Some("qStringToRustStr"),
            Self::String => Some("qStringToRustString"),
        }
    }

    /// Whether this type is a const (when used as an input to methods)
    fn is_const(&self) -> bool {
        match self {
            Self::I32 => false,
            Self::Str => true,
            Self::String => true,
        }
    }

    /// Whether this type is a reference
    ///
    /// TODO: read from the extract ParameterType if it's a ref first
    /// so that we can return or take &int, but also consider that String is
    /// always ref in arguments?
    fn is_ref(&self) -> bool {
        match self {
            Self::I32 => false,
            Self::Str => true,
            Self::String => true,
        }
    }

    /// The C++ type name of the CppType
    fn type_ident(&self) -> &'static str {
        match self {
            Self::I32 => "int",
            Self::Str => "QString",
            Self::String => "QString",
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

/// Describes a C++ invokable with header and source parts
#[derive(Debug)]
struct CppInvokable {
    /// The header definition of the invokable
    header: String,
    /// The source implementation of the invokable
    source: String,
}
/// Describes a C++ property with header and source parts
#[derive(Debug)]
struct CppProperty {
    /// The header meta definition of the invokable
    header_meta: String,
    /// The header public definition of the invokable
    header_public: String,
    /// The header signals definition of the invokable
    header_signals: String,
    /// The header slots definition of the invokable
    header_slots: String,
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
fn generate_type_cpp(type_ident: &ParameterType) -> Result<CppTypes, TokenStream> {
    match type_ident.ident.to_string().as_str() {
        "str" => Ok(CppTypes::Str),
        "String" => Ok(CppTypes::String),
        "i32" => Ok(CppTypes::I32),
        other => Err(Error::new(
            type_ident.ident.span(),
            format!("Unknown type ident to convert to C++: {}", other),
        )
        .to_compile_error()),
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

    // A helper which allows us to flatten data from vec of parameters
    struct CppParameterHelper {
        args: Vec<String>,
        names: Vec<String>,
    }

    for invokable in invokables {
        // Query for parameters and flatten them into a helper
        let parameters = generate_parameters_cpp(&invokable.parameters)?
            .drain(..)
            .fold(
                CppParameterHelper {
                    args: vec![],
                    names: vec![],
                },
                |mut acc, parameter| {
                    // Build the parameter as a type argument
                    acc.args.push(format!(
                        "{is_const} {type_ident}{is_ref} {ident}",
                        ident = parameter.ident,
                        is_const = if parameter.type_ident.is_const() {
                            "const"
                        } else {
                            ""
                        },
                        is_ref = if parameter.type_ident.is_ref() {
                            "&"
                        } else {
                            ""
                        },
                        type_ident = parameter.type_ident.type_ident()
                    ));
                    // If there is a converter then use it
                    if let Some(converter_ident) = parameter.type_ident.convert_into_rust() {
                        acc.names
                            .push(format!("{}({})", converter_ident, parameter.ident));
                    } else {
                        // No converter so use the same name
                        acc.names.push(parameter.ident);
                    }
                    acc
                },
            );
        let parameter_arg_line = parameters.args.join(", ");

        // Extract the return type of the invokable if there is one
        let return_type = if let Some(return_type) = &invokable.return_type {
            Some(generate_type_cpp(&return_type)?)
        } else {
            None
        };

        // Prepare the body of the invokable, we may return or wrap this later
        let body = format!(
            "m_rustObj->{ident}({parameter_names})",
            ident = invokable.ident.to_string(),
            parameter_names = parameters.names.join(", ")
        );

        // Cache the return ident as it's used in both header and source
        let return_ident = if let Some(return_type) = &return_type {
            return_type.type_ident()
        } else {
            "void"
        };

        // Prepare the CppInvokable
        items.push(CppInvokable {
            // TODO: detect if method is const from whether we have &self or &mut self in rust
            header: format!(
                "Q_INVOKABLE {return_ident} {ident}({parameter_types}) const;",
                ident = invokable.ident.to_string(),
                parameter_types = parameter_arg_line,
                return_ident = return_ident,
            ),
            source: formatdoc! {
                r#"
                {return_ident} {struct_ident}::{ident}({parameter_types}) const
                {{
                    {body};
                }}
                "#,
                // Decide if the body needs a return or converter
                body = if let Some(return_type) = &return_type {
                    if let Some(converter_ident) = return_type.convert_into_cpp() {
                        format!("return {converter}({body})", converter = converter_ident, body = body)
                    } else {
                        format!("return {body}", body = body)
                    }
                } else {
                    body
                },
                ident = invokable.ident.to_string(),
                parameter_types = parameter_arg_line,
                struct_ident = struct_ident.to_string(),
                return_ident = return_ident,
            },
        });
    }

    Ok(items)
}

/// Generate a CppProperty object containing the header and source of a given list of rust properties
fn generate_properties_cpp(
    struct_ident: &Ident,
    properties: &[Property],
) -> Result<Vec<CppProperty>, TokenStream> {
    let mut items: Vec<CppProperty> = vec![];

    for property in properties {
        let parameter = CppParameter {
            ident: property.ident.to_string(),
            type_ident: generate_type_cpp(&property.type_ident)?,
        };
        let converter_getter = parameter.type_ident.convert_into_cpp();
        let converter_setter = parameter.type_ident.convert_into_rust();
        // TODO: for now we assume that all properties have a getter/setter/notify
        let ident_getter = property.getter.as_ref().unwrap().cpp_ident.to_string();
        let ident_setter = property.setter.as_ref().unwrap().cpp_ident.to_string();
        let ident_changed = property.notify.as_ref().unwrap().cpp_ident.to_string();
        let is_const = if parameter.type_ident.is_const() {
            "const"
        } else {
            ""
        };
        let is_ref = if parameter.type_ident.is_ref() {
            "&"
        } else {
            ""
        };
        let rust_getter = format!("m_rustObj->{ident_getter}()", ident_getter = ident_getter);
        let type_ident = parameter.type_ident.type_ident();

        items.push(CppProperty {
            header_meta: format!("Q_PROPERTY({type_ident} {ident} READ {ident_getter} WRITE {ident_setter} NOTIFY {ident_changed})",
                ident = parameter.ident,
                ident_changed = ident_changed,
                ident_getter = ident_getter,
                ident_setter = ident_setter,
                type_ident = type_ident,
            ),
            header_public: format!("{type_ident} {ident_getter}() const;",
                ident_getter = ident_getter,
                type_ident = type_ident,
            ),
            header_signals: format!("void {ident_changed}();", ident_changed = ident_changed),
            header_slots: format!("void {ident_setter}({is_const} {type_ident}{is_ref} value);",
                ident_setter = ident_setter,
                is_const = is_const,
                is_ref = is_ref,
                type_ident = type_ident,
            ),
            // TODO: {converter_setter} needs to start on the same line as the { so that when
            // there is no converter we don't have an empty line at the start of the setter.
            // As clang-format doesn't remove this empty line. Is there a better way ?
            source: formatdoc! {
                r#"
                {type_ident}
                {struct_ident}::{ident_getter}() const
                {{
                    {converter_getter}
                }}

                void
                {struct_ident}::{ident_setter}({is_const} {type_ident}{is_ref} value)
                {{{converter_setter}
                    if ({converter_setter_ident} != m_rustObj->{ident_getter}()) {{
                        m_rustObj->{ident_setter}({converter_setter_ident_move});

                        Q_EMIT {ident_changed}();
                    }}
                }}
                "#,
                converter_getter = if let Some(converter_ident) = converter_getter {
                    format!("return {converter_ident}({value});",
                        converter_ident = converter_ident,
                        value = rust_getter,
                    )
                } else {
                    format!("return {rust_getter};", rust_getter = rust_getter)
                },
                // Build a converter which creates rustValue if required
                converter_setter = if let Some(converter_ident) = converter_setter {
                    format!("auto rustValue = {converter_ident}(value);",
                        converter_ident = converter_ident,
                    )
                } else {
                    "".to_owned()
                },
                // Determine if we should be using rustValue or value
                converter_setter_ident = if converter_setter.is_some() {
                    "rustValue"
                } else {
                    "value"
                },
                // If there is a setter converter then it means we have created a named variable
                // so then we should use std::move
                converter_setter_ident_move = if converter_setter.is_some() {
                    "std::move(rustValue)"
                } else {
                    "value"
                },
                ident_changed = ident_changed,
                ident_getter = ident_getter,
                ident_setter = ident_setter,
                is_const = is_const,
                is_ref = is_ref,
                struct_ident = struct_ident.to_string(),
                type_ident = type_ident,
            },
        });
    }

    Ok(items)
}

/// Generate a CppObject object containing the header and source of a given rust QObject
pub fn generate_qobject_cpp(obj: &QObject) -> Result<CppObject, TokenStream> {
    let struct_ident_str = obj.ident.to_string();
    let rust_struct_ident_str = obj.rust_struct_ident.to_string();

    // A helper which allows us to flatten data from vec of properties
    struct CppPropertyHelper {
        headers_meta: Vec<String>,
        headers_public: Vec<String>,
        headers_signals: Vec<String>,
        headers_slots: Vec<String>,
        sources: Vec<String>,
    }

    // Query for properties
    let properties = generate_properties_cpp(&obj.ident, &obj.properties)?
        .drain(..)
        .fold(
            CppPropertyHelper {
                headers_meta: vec![],
                headers_public: vec![],
                headers_signals: vec![],
                headers_slots: vec![],
                sources: vec![],
            },
            |mut acc, property| {
                acc.headers_meta.push(property.header_meta);
                acc.headers_public.push(property.header_public);
                acc.headers_signals.push(property.header_signals);
                acc.headers_slots.push(property.header_slots);
                acc.sources.push(property.source);
                acc
            },
        );

    // A helper which allows us to flatten data from vec of invokables
    struct CppInvokableHelper {
        headers: Vec<String>,
        sources: Vec<String>,
    }

    // Query for invokables and flatten them into a helper
    let invokables = generate_invokables_cpp(&obj.ident, &obj.invokables)?
        .drain(..)
        .fold(
            CppInvokableHelper {
                headers: vec![],
                sources: vec![],
            },
            |mut acc, invokable| {
                acc.headers.push(invokable.header);
                acc.sources.push(invokable.source);
                acc
            },
        );

    // Generate C++ header part
    let signals = if properties.headers_signals.is_empty() {
        "".to_owned()
    } else {
        formatdoc! {r#"
            Q_SIGNALS:
            {properties_signals}
            "#,
            properties_signals = properties.headers_signals.join("\n"),
        }
    };
    let public_slots = if properties.headers_slots.is_empty() {
        "".to_owned()
    } else {
        formatdoc! {r#"
            public Q_SLOTS:
            {properties_slots}
            "#,
            properties_slots = properties.headers_slots.join("\n"),
        }
    };
    let header = formatdoc! {r#"
        #pragma once

        #include "rust/cxx_qt.h"

        class {rust_struct_ident};

        class {ident} : public QObject {{
            Q_OBJECT
        {properties_meta}

        public:
            explicit {ident}(QObject *parent = nullptr);
            ~{ident}();

        {properties_public}

        {invokables}

        {public_slots}

        {signals}

        private:
            rust::Box<{rust_struct_ident}> m_rustObj;
        }};

        std::unique_ptr<{ident}> new{ident}();
        "#,
    ident = struct_ident_str,
    invokables = invokables.headers.join("\n"),
    properties_meta = properties.headers_meta.join("\n"),
    properties_public = properties.headers_public.join("\n"),
    rust_struct_ident = rust_struct_ident_str,
    signals = signals,
    public_slots = public_slots,
    };

    // Generate C++ source part
    let source = formatdoc! {r#"
        #include "cxx-qt-gen/include/{ident_snake}.h"
        #include "cxx-qt-gen/src/{ident_snake}.rs.h"

        {ident}::{ident}(QObject *parent)
            : QObject(parent)
            , m_rustObj(create{ident}Rs())
        {{
        }}

        {ident}::~{ident}() = default;

        {properties}

        {invokables}

        std::unique_ptr<{ident}> new{ident}()
        {{
            return std::make_unique<{ident}>();
        }}
        "#,
        ident = struct_ident_str,
        ident_snake = struct_ident_str.to_case(Case::Snake),
        invokables = invokables.sources.join("\n"),
        properties = properties.sources.join("\n"),
    };

    Ok(CppObject {
        // TODO: handle clang-format errors?
        header: clang_format(&header).unwrap_or(header),
        source: clang_format(&source).unwrap_or(source),
    })
}

/// Set the clang-format style to the given ClangFormatStyle or fallback to using Mozilla
/// This is used for formatting any resultant C++ headers or sources.
pub fn generate_format(style: Option<ClangFormatStyle>) -> Result<(), ClangFormatStyle> {
    CLANG_FORMAT_STYLE.set(style.unwrap_or(ClangFormatStyle::Mozilla))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::extract_qobject;

    use pretty_assertions::assert_eq;

    #[test]
    fn generates_basic_invokable_and_properties() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_invokable_and_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_header = clang_format(include_str!(
            "../test_outputs/basic_invokable_and_properties.h"
        ))
        .unwrap();
        let expected_source = clang_format(include_str!(
            "../test_outputs/basic_invokable_and_properties.cpp"
        ))
        .unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_basic_only_invokables() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/basic_only_invokable.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/basic_only_invokable.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_basic_only_invokables_with_return() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_invokable_return.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_header = clang_format(include_str!(
            "../test_outputs/basic_only_invokable_return.h"
        ))
        .unwrap();
        let expected_source = clang_format(include_str!(
            "../test_outputs/basic_only_invokable_return.cpp"
        ))
        .unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_basic_only_properties() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_only_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(module).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/basic_only_properties.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/basic_only_properties.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }
}
