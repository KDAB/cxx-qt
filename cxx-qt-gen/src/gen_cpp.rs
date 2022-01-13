// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use clang_format::{clang_format, ClangFormatStyle, CLANG_FORMAT_STYLE};
use convert_case::{Case, Casing};
use indoc::formatdoc;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::extract::{Invokable, Parameter, Property, QObject, QtTypes};

/// A trait which we implement on QtTypes allowing retrieval of attributes of the enum value.
trait CppType {
    /// String representation of the const part of this type
    fn as_const_str(&self) -> &str;
    /// String representation of the pointer part of this type
    fn as_ptr_str(&self) -> &str;
    /// String representation of the ref part of this type
    fn as_ref_str(&self) -> &str;
    /// Any converter that is required to convert this type into C++
    fn convert_into_cpp(&self) -> Option<&'static str>;
    /// Any include paths for this type, this is used for Ptr types
    /// for example so that when Object uses SubObject it includes sub_object.h
    fn include_paths(&self) -> Vec<String>;
    /// Whether this type is a const (when used as an input to methods)
    fn is_const(&self) -> bool;
    /// Whether this type is a Pin<T>
    fn is_pin(&self) -> bool;
    /// Whether this type is a pointer
    fn is_ptr(&self) -> bool;
    /// Whether this type is a reference
    fn is_ref(&self) -> bool;
    /// Whether this type is a this (eg the T in Pin<T>)
    fn is_this(&self) -> bool;
    /// The C++ type name of the CppType
    fn type_ident(&self) -> &str;
}

impl CppType for QtTypes {
    /// String representation of the const part of this type
    fn as_const_str(&self) -> &str {
        if self.is_const() {
            "const"
        } else {
            ""
        }
    }

    /// String representation of the pointer part of this type
    fn as_ptr_str(&self) -> &str {
        if self.is_ptr() {
            "*"
        } else {
            ""
        }
    }

    /// String representation of the ref part of this type
    fn as_ref_str(&self) -> &str {
        if self.is_ref() {
            "&"
        } else {
            ""
        }
    }

    /// Any converter that is required to convert this type into C++
    fn convert_into_cpp(&self) -> Option<&'static str> {
        match self {
            Self::Bool => None,
            Self::F32 | Self::F64 => None,
            Self::I8 | Self::I16 | Self::I32 => None,
            Self::QPoint => None,
            Self::QPointF => None,
            Self::QSizeF => None,
            Self::Str => Some("rustStrToQString"),
            Self::String => Some("rustStringToQString"),
            Self::QVariant => None,
            Self::Variant => Some("::CxxQt::rustVariantToQVariant"),
            Self::U8 | Self::U16 | Self::U32 => None,
            _others => unreachable!(),
        }
    }

    /// Any include paths for this type, this is used for Ptr types
    /// for example so that when Object uses SubObject it includes sub_object.h
    fn include_paths(&self) -> Vec<String> {
        match self {
            // If we are an external CppObj then we need to build an include path
            //
            // TODO: once we generate sub folders for nested modules, this will need to use all
            // type idents other than first and last.
            // https://github.com/KDAB/cxx-qt/issues/19
            Self::CppObj {
                external,
                cpp_type_idents,
                ..
            } if external == &true && cpp_type_idents.len() > 2 => vec![format!(
                "#include \"cxx-qt-gen/include/{}.h\"",
                cpp_type_idents[cpp_type_idents.len() - 2]
            )],
            Self::QPoint => vec!["#include <QtCore/QPoint>".to_owned()],
            Self::QPointF => vec!["#include <QtCore/QPointF>".to_owned()],
            Self::QSizeF => vec!["#include <QtCore/QSizeF>".to_owned()],
            // FIXME: do we need both variant and qvariant here?
            Self::QVariant | Self::Variant => vec!["#include <QtCore/QVariant>".to_owned()],
            _others => vec![],
        }
    }

    /// Whether this type is a const (when used as an input to methods)
    ///
    /// For now this means that we consider the type in C++ to be const
    /// eg String => const QString and not whether the rust type was const.
    fn is_const(&self) -> bool {
        match self {
            Self::Bool => false,
            Self::CppObj { .. } => false,
            Self::F32 | Self::F64 => false,
            Self::I8 | Self::I16 | Self::I32 => false,
            Self::QPoint => true,
            Self::QPointF => true,
            Self::QSizeF => true,
            Self::Str => true,
            Self::String => true,
            Self::QString => true,
            Self::QVariant | Self::Variant => true,
            Self::U8 | Self::U16 | Self::U32 => false,
            _other => unreachable!(),
        }
    }

    /// Whether this type is a Pin<T> this is then used in method definitions
    /// to add *this to this or *arg to arg.
    fn is_pin(&self) -> bool {
        match self {
            Self::CppObj { .. } => true,
            _others => false,
        }
    }

    /// Whether this type is a pointer
    fn is_ptr(&self) -> bool {
        match self {
            Self::CppObj { .. } => true,
            _other => false,
        }
    }

    /// Whether this type is a reference (when used as an input to methods)
    ///
    /// For now this means that we consider the type in C++ to be a ref
    /// eg String => QString& and not whether the rust type was a ref.
    ///
    /// TODO: read from the extract ParameterType if it's a ref first
    /// so that we can return or take &int, but also consider that String is
    /// always ref in arguments?
    fn is_ref(&self) -> bool {
        match self {
            Self::Bool => false,
            Self::CppObj { .. } => false,
            Self::F32 | Self::F64 => false,
            Self::I8 | Self::I16 | Self::I32 => false,
            Self::QPoint => true,
            Self::QPointF => true,
            Self::QSizeF => true,
            Self::Str => true,
            Self::String => true,
            Self::QString => true,
            Self::QVariant | Self::Variant => true,
            Self::U8 | Self::U16 | Self::U32 => false,
            _other => unreachable!(),
        }
    }

    /// Whether this type is_this, this is used to determine if the ident is changed in method
    /// definitions and if the parameter should be skipped in method declarations
    fn is_this(&self) -> bool {
        match self {
            Self::CppObj { external, .. } => external == &false,
            _others => false,
        }
    }

    /// The C++ type name of the CppType
    fn type_ident(&self) -> &str {
        match self {
            Self::Bool => "bool",
            Self::CppObj {
                cpp_type_idents_string,
                ..
            } => cpp_type_idents_string,
            Self::F32 => "float",
            Self::F64 => "double",
            Self::I8 => "qint8",
            Self::I16 => "qint16",
            Self::I32 => "qint32",
            Self::QPoint => "QPoint",
            Self::QPointF => "QPointF",
            Self::QSizeF => "QSizeF",
            Self::Str | Self::String | Self::QString => "QString",
            Self::QVariant | Self::Variant => "QVariant",
            Self::U8 => "quint8",
            Self::U16 => "quint16",
            Self::U32 => "quint32",
            _other => unreachable!(),
        }
    }
}

/// Describes a C++ parameter, which is a name combined with a type
#[derive(Debug)]
struct CppParameter<'a> {
    /// The ident of the parameter
    ident: String,
    /// The type of the parameter
    type_ident: &'a QtTypes,
}

/// Describes a C++ invokable with header and source parts
#[derive(Debug)]
struct CppInvokable {
    /// Any extra include that is required for the invokable
    header_includes: Vec<String>,
    /// The header definition of the invokable
    header: String,
    /// The source implementation of the invokable
    source: String,
}
/// Describes a C++ property with header and source parts
#[derive(Debug)]
struct CppProperty {
    /// Any extra include that is required for the property
    header_includes: Vec<String>,
    /// Any members that are required for the property
    header_members: Vec<String>,
    /// The header meta definition of the invokable
    header_meta: Vec<String>,
    /// The header public definition of the invokable
    header_public: Vec<String>,
    /// The header signals definition of the invokable
    header_signals: Vec<String>,
    /// The header slots definition of the invokable
    header_slots: Vec<String>,
    /// The source implementation of the invokable
    source: Vec<String>,
}

/// Describes a C++ header and source files of a C++ class
#[derive(Debug)]
pub struct CppObject {
    /// The header of the C++ class
    pub header: String,
    /// The source of the C++ class
    pub source: String,
}

/// Generate a string of parameters with their type in C++ style from a given list of rust parameters
fn generate_parameters_cpp(parameters: &[Parameter]) -> Result<Vec<CppParameter>, TokenStream> {
    let mut items: Vec<CppParameter> = vec![];

    // Extract the ident and type_ident from each parameter
    for parameter in parameters {
        items.push(CppParameter {
            // If the Pin<T> is "this" then we need to rename to using "this" as the ident
            //
            // Note the * for method definitions is added later when generating CppParameterHelper's
            // We don't add the * here otherwise we'll have double * in the method declaration.
            ident: if parameter.type_ident.qt_type.is_this() {
                "this".to_owned()
            } else {
                parameter.ident.to_string()
            },
            type_ident: &parameter.type_ident.qt_type,
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
        // These are a list of definitions for the parameters
        // This includes the const, ref, ptr, type, and ident of the parameter
        // eg this could be "const QString& string" or "MyObject* object"
        args: Vec<String>,
        // These are a list of the include paths for the parameters types
        // This is used for if a SubObject is in a parameter.
        include_paths: Vec<String>,
        // These are a list of names of the parameters
        // If the parameter has a converter then it could be the name wrapped in a converted
        // eg this could be "arg1" or "converter(arg1)" or "*this"
        names: Vec<String>,
    }

    for invokable in invokables {
        // Query for parameters and flatten them into a helper
        let parameters = generate_parameters_cpp(&invokable.parameters)?
            .drain(..)
            .fold(
                CppParameterHelper {
                    args: vec![],
                    include_paths: vec![],
                    names: vec![],
                },
                |mut acc, parameter| {
                    // Only add to args if we are not is_this
                    // Because we use *this and do not take an argument in the declaration
                    //
                    // If we are Pin<T> but not "this" then we can continue as normal as we want *T
                    if !parameter.type_ident.is_this() {
                        // Build the parameter as a type argument
                        acc.args.push(format!(
                            "{is_const} {type_ident}{is_ref}{is_ptr} {ident}",
                            ident = parameter.ident,
                            is_const = parameter.type_ident.as_const_str(),
                            is_ref = parameter.type_ident.as_ref_str(),
                            is_ptr = parameter.type_ident.as_ptr_str(),
                            type_ident = parameter.type_ident.type_ident()
                        ));

                        // Add any includes paths for the type
                        //
                        // We do not need to do this when we are "this"
                        acc.include_paths
                            .append(&mut parameter.type_ident.include_paths());
                    }

                    // Build the parameter names
                    //
                    // When a Pin<T> is used in a method we need to use *name to get a reference
                    let param_ident = if parameter.type_ident.is_pin() {
                        format!("*{}", parameter.ident)
                    } else {
                        parameter.ident
                    };

                    acc.names.push(param_ident);
                    acc
                },
            );

        // Cache an argument line of all the parameters as this is used in both header and source
        let parameter_arg_line = parameters.args.join(", ");

        // Extract the return type of the invokable if there is one
        let return_type = invokable
            .return_type
            .as_ref()
            .map(|return_type| &return_type.qt_type);

        // Prepare the body of the invokable, we may return or wrap this later
        let body = format!(
            "m_rustObj->{ident}({parameter_names})",
            // If we are a pointer to a CppObj then we need a wrapper
            ident = if let Some(ident_wrapper) = &invokable.ident_wrapper {
                ident_wrapper.cpp_ident.to_string()
            } else {
                invokable.ident.cpp_ident.to_string()
            },
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
            header_includes: parameters.include_paths,
            // TODO: detect if method is const from whether we have &self or &mut self in rust
            // TODO: also needs to consider if there is a Pin<&mut T> as we need non-const if
            // we are passing *this across for cpp objects in rust.
            header: format!(
                "Q_INVOKABLE {return_ident} {ident}({parameter_types});",
                ident = invokable.ident.cpp_ident,
                parameter_types = parameter_arg_line,
                return_ident = return_ident,
            ),
            source: formatdoc! {
                r#"
                {return_ident} {struct_ident}::{ident}({parameter_types})
                {{
                    const std::lock_guard<std::mutex> guard(m_rustObjMutex);
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
                ident = invokable.ident.cpp_ident.to_string(),
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
    has_property_change_handler: bool,
) -> Result<Vec<CppProperty>, TokenStream> {
    let mut items: Vec<CppProperty> = vec![];

    for property in properties {
        // Build a CppParameter for the name and type of the property
        let parameter = CppParameter {
            ident: property.ident.cpp_ident.to_string(),
            type_ident: &property.type_ident.qt_type,
        };

        // Collect the C++ idents for the getter, setter, notify of the property
        //
        // TODO: for now we assume that all properties have a getter/setter/notify
        let ident_getter = property.getter.as_ref().unwrap().cpp_ident.to_string();
        let ident_setter = property.setter.as_ref().unwrap().cpp_ident.to_string();
        let ident_changed = property.notify.as_ref().unwrap().cpp_ident.to_string();

        // We shouldn't currently have a case where ref and ptr together makes any sense
        assert!(!(parameter.type_ident.is_ref() && parameter.type_ident.is_ptr()));

        // Build the C++ strings for whether the const, ref, and ptr are set for this property
        let is_const = parameter.type_ident.as_const_str();
        let is_ref = parameter.type_ident.as_ref_str();
        let is_ptr = parameter.type_ident.as_ptr_str();

        // Cache the type ident of the property as this is used multiple times
        let type_ident = parameter.type_ident.type_ident();

        // Build a basic C++ property with parts that are defined if the property is a pointer or not
        let mut cpp_property = CppProperty {
            // Set any includes from the type of the property
            // eg this is used if the type is a pointer to include that type
            header_includes: parameter.type_ident.include_paths(),
            // Members are defined later for only the pointer
            header_members: vec![],
            // Set the Q_PROPERTY for the C++ class
            header_meta: vec![format!("Q_PROPERTY({type_ident}{is_ptr} {ident} READ {ident_getter} WRITE {ident_setter} NOTIFY {ident_changed})",
                ident = parameter.ident,
                ident_changed = ident_changed,
                ident_getter = ident_getter,
                ident_setter = ident_setter,
                is_ptr = is_ptr,
                type_ident = type_ident,
            )],
            // Set basic getter, more are added later for only pointer
            header_public: vec![format!("{is_const} {type_ident}{is_ptr}{is_ref} {ident_getter}() const;",
                ident_getter = ident_getter,
                is_const = is_const,
                is_ptr = is_ptr,
                is_ref = is_ref,
                type_ident = type_ident,
            )],
            // Set the notify signals
            header_signals: vec![format!("void {ident_changed}();", ident_changed = ident_changed)],
            // Set the slots for the setter
            header_slots: vec![format!("void {ident_setter}({is_const} {type_ident}{is_ref}{is_ptr} value);",
                ident_setter = ident_setter,
                is_const = is_const,
                is_ref = is_ref,
                is_ptr = is_ptr,
                type_ident = type_ident,
            )],
            // The source is created later
            source: vec![],
        };

        // Build as pascal version of the ident
        // this is used for the owned member and extra pointer specific methods
        let parameter_ident_pascal = parameter.ident.to_case(Case::Pascal);

        let call_property_change_handler = if has_property_change_handler {
            formatdoc! {
                r#"
                runOnGUIThread([&]() {{
                    const std::lock_guard<std::mutex> guard(m_rustObjMutex);
                    m_rustObj->handlePropertyChange(*this, Property::{ident});
                }});
                "#,
                ident = parameter_ident_pascal
            }
        } else {
            "".to_owned()
        };

        // TODO: we eventually want to also support handling property changes inside subobjects
        // by wiring up the relevant changed signals to m_rustObj->handlePropertyChange.

        // If we are a pointer type then add specific methods
        if parameter.type_ident.is_ptr() {
            // Pointers are stored in the C++ object, so build a member and owned ident
            let member_ident = format!("m_{}", parameter.ident);
            let member_owned_ident = format!("m_owned{}", parameter_ident_pascal);

            // Add raw pointer getter and setter
            //
            // Note that the setter is different to the non-pointer source
            cpp_property.source.push(formatdoc! {
                r#"
                {is_const} {type_ident}{is_ptr}{is_ref}
                {struct_ident}::{ident_getter}() const
                {{
                    return {member_ident};
                }}

                void
                {struct_ident}::{ident_setter}({is_const} {type_ident}{is_ref}{is_ptr} value)
                {{
                    if (value != {member_ident}) {{
                        if ({member_owned_ident}) {{
                            {member_owned_ident}.reset();
                        }}

                        {member_ident} = value;

                        runOnGUIThread([&]() {{ Q_EMIT {ident_changed}(); }});

                        {call_property_change_handler}
                    }}
                }}
                "#,
                ident_changed = ident_changed,
                ident_getter = ident_getter,
                ident_setter = ident_setter,
                is_const = is_const,
                is_ref = is_ref,
                is_ptr = is_ptr,
                member_ident = member_ident,
                member_owned_ident = member_owned_ident,
                struct_ident = struct_ident.to_string(),
                type_ident = type_ident,
                call_property_change_handler = call_property_change_handler,
            });

            // Add members to the reference and own it
            cpp_property.header_members.push(format!(
                "{type_ident}* m_{ident} = nullptr;",
                ident = parameter.ident,
                type_ident = type_ident
            ));
            cpp_property.header_members.push(format!(
                "std::unique_ptr<{type_ident}> {member_owned_ident};",
                member_owned_ident = member_owned_ident,
                type_ident = type_ident
            ));

            // Add a unique_ptr getter for taking the object
            cpp_property.header_public.push(format!(
                "std::unique_ptr<{type_ident}> take{ident}();",
                type_ident = type_ident,
                ident = parameter_ident_pascal,
            ));

            // Add a unique_ptr setter
            // Note that this cannot be added to the Q_SLOTS as moc can't handle the unique_ptr
            cpp_property.header_public.push(format!(
                "void give{ident}(std::unique_ptr<{type_ident}> value);",
                type_ident = type_ident,
                ident = parameter_ident_pascal,
            ));

            // Add unique_ptr getter/setter
            cpp_property.source.push(formatdoc!(
                r#"
                std::unique_ptr<{type_ident}>
                {struct_ident}::take{ident_pascal}()
                {{
                  auto value = std::move({member_owned_ident});
                  {ident_setter}(nullptr);
                  return value;
                }}

                void
                {struct_ident}::give{ident_pascal}(std::unique_ptr<{type_ident}> value)
                {{
                  Q_ASSERT(value.get() != {member_ident});

                  {member_owned_ident} = std::move(value);
                  {member_ident} = {member_owned_ident}.get();

                  runOnGUIThread([&]() {{ Q_EMIT {ident_changed}(); }});

                  {call_change_handler}
                }}
                "#,
                ident_changed = ident_changed,
                ident_pascal = parameter_ident_pascal,
                ident_setter = ident_setter,
                member_ident = member_ident,
                member_owned_ident = member_owned_ident,
                struct_ident = struct_ident.to_string(),
                type_ident = type_ident,
                call_change_handler = call_property_change_handler,
            ));
        } else {
            cpp_property.source.push(formatdoc! {
                r#"
                {is_const} {type_ident}{is_ptr}{is_ref}
                {struct_ident}::{ident_getter}() const
                {{
                    return {member_ident};
                }}

                void
                {struct_ident}::{ident_setter}({is_const} {type_ident}{is_ref}{is_ptr} value)
                {{
                    if (!m_initialised) {{
                        {member_ident} = value;
                        return;
                    }}

                    if (value != {member_ident}) {{
                        {member_ident} = value;

                        runOnGUIThread([&]() {{ Q_EMIT {ident_changed}(); }});

                        {call_change_handler}
                    }}
                }}
                "#,
                ident_changed = ident_changed,
                ident_getter = ident_getter,
                ident_setter = ident_setter,
                is_const = is_const,
                is_ref = is_ref,
                is_ptr = is_ptr,
                struct_ident = struct_ident.to_string(),
                type_ident = type_ident,
                member_ident = format!("m_{}", parameter.ident),
                call_change_handler = call_property_change_handler,
            });

            // Own the member on the C++ side
            // TODO: start using these in the getters and setters
            // TODO: remove Rust side ownership
            cpp_property.header_members.push(format!(
                "{type_ident} m_{ident};",
                ident = parameter.ident,
                type_ident = type_ident
            ));
        }

        items.push(cpp_property);
    }

    Ok(items)
}

/// Generate a CppObject object containing the header and source of a given rust QObject
pub fn generate_qobject_cpp(obj: &QObject) -> Result<CppObject, TokenStream> {
    let struct_ident_str = obj.ident.to_string();
    const RUST_STRUCT_IDENT_STR: &str = "RustObj";

    // A helper which allows us to flatten data from vec of properties
    struct CppPropertyHelper {
        headers_includes: Vec<String>,
        headers_members: Vec<String>,
        headers_meta: Vec<String>,
        headers_public: Vec<String>,
        headers_signals: Vec<String>,
        headers_slots: Vec<String>,
        sources: Vec<String>,
    }

    // Build CppProperty's for the object, then drain them into our CppPropertyHelper
    let properties = generate_properties_cpp(
        &obj.ident,
        &obj.properties,
        obj.handle_property_change_impl.is_some(),
    )?
    .drain(..)
    .fold(
        CppPropertyHelper {
            headers_includes: vec![],
            headers_members: vec![],
            headers_meta: vec![],
            headers_public: vec![],
            headers_signals: vec![],
            headers_slots: vec![],
            sources: vec![],
        },
        |mut acc, mut property| {
            acc.headers_includes.append(&mut property.header_includes);
            acc.headers_meta.append(&mut property.header_meta);
            acc.headers_members.append(&mut property.header_members);
            acc.headers_public.append(&mut property.header_public);
            acc.headers_signals.append(&mut property.header_signals);
            acc.headers_slots.append(&mut property.header_slots);
            acc.sources.append(&mut property.source);
            acc
        },
    );

    // A helper which allows us to flatten data from vec of invokables
    struct CppInvokableHelper {
        headers_includes: Vec<String>,
        headers: Vec<String>,
        sources: Vec<String>,
    }

    // Build CppInvokable's for the object, then drain them into our CppInvokableHelper
    let mut invokables = generate_invokables_cpp(&obj.ident, &obj.invokables)?
        .drain(..)
        .fold(
            CppInvokableHelper {
                headers_includes: vec![],
                headers: vec![],
                sources: vec![],
            },
            |mut acc, mut invokable| {
                acc.headers_includes.append(&mut invokable.header_includes);
                acc.headers.push(invokable.header);
                acc.sources.push(invokable.source);
                acc
            },
        );

    // If there are signals then prepare a string otherwise leave an empty string
    // We need to do this otherwise we are left with a Q_SIGNALS with nothing after it
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
    // If there are public slots then prepare a string otherwise leave an empty string
    // We need to do this otherwise we are left with a Q_SLOTS with nothing after it
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

    let mut private_method_headers = vec![];
    let mut private_method_sources = vec![];

    if obj.handle_updates_impl.is_some() {
        private_method_headers.push("Q_INVOKABLE void requestUpdate();");
        private_method_headers.push("void updateState();");

        private_method_sources.push(formatdoc! {r#"
            void {ident}::requestUpdate() {{
                runOnGUIThread([&]() {{ updateState(); }});
            }}
        "#,
        ident = struct_ident_str,
        });
        private_method_sources.push(formatdoc! {r#"
            void {ident}::updateState() {{
                const std::lock_guard<std::mutex> guard(m_rustObjMutex);
                m_rustObj->handleUpdateRequest(*this);
            }}
        "#,
        ident = struct_ident_str,
        });
    }

    let namespace = obj.namespace.join("::");
    // let combined_ident = obj.namespace
    //     .iter()
    //     .cloned()
    //     .chain(vec!["FFICppObj".to_owned()])
    //     .collect::<Vec<String>>()
    //     .join("_").to_case(Case::Pascal);

    // Generate the C++ header part
    let header = formatdoc! {r#"
        #pragma once

        #include <mutex>

        #include "rust/cxx_qt.h"

        {includes}

        namespace {namespace} {{

        class {rust_struct_ident};

        class {ident} : public CxxQObject {{
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
            std::mutex m_rustObjMutex;
            bool m_initialised = false;

            {members_private}

            {private_method_headers}
        }};

        typedef {ident} CppObj;

        std::unique_ptr<CppObj> newCppObject();

        }} // namespace {namespace}

        Q_DECLARE_METATYPE({namespace}::CppObj*)
        "#,
    ident = struct_ident_str,
    invokables = invokables.headers.join("\n"),
    members_private = properties.headers_members.join("\n"),
    includes = {
        let mut includes = properties.headers_includes;
        includes.append(&mut invokables.headers_includes);
        // Sort and remove duplicates
        includes.sort();
        includes.dedup();
        includes.join("\n")
    },
    namespace = namespace,
    properties_meta = properties.headers_meta.join("\n"),
    properties_public = properties.headers_public.join("\n"),
    rust_struct_ident = RUST_STRUCT_IDENT_STR,
    signals = signals,
    public_slots = public_slots,
    private_method_headers = private_method_headers.join("\n"),
    };

    // Generate C++ source part
    let source = formatdoc! {r#"
        #include "cxx-qt-gen/include/{ident_snake}.h"
        #include "cxx-qt-gen/src/{ident_snake}.rs.h"

        namespace {namespace} {{

        {ident}::{ident}(QObject *parent)
            : CxxQObject(parent)
            , m_rustObj(createRs())
        {{
            initialiseCpp(*this);
            m_initialised = true;
        }}

        {ident}::~{ident}() = default;

        {properties}

        {invokables}

        {private_method_sources}

        std::unique_ptr<CppObj> newCppObject()
        {{
            return std::make_unique<CppObj>();
        }}

        }} // namespace {namespace}
        "#,
        ident = struct_ident_str,
        ident_snake = struct_ident_str.to_case(Case::Snake),
        invokables = invokables.sources.join("\n"),
        namespace = namespace,
        properties = properties.sources.join("\n"),
        private_method_sources = private_method_sources.join("\n"),
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
    use syn::ItemMod;

    #[test]
    fn generates_basic_invokable_and_properties() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_invokable_and_properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

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
    fn generates_basic_ident_changes() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_ident_changes.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/basic_ident_changes.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/basic_ident_changes.cpp")).unwrap();
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
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

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
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

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
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/basic_only_properties.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/basic_only_properties.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_basic_pin_invokable() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_pin_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/basic_pin_invokable.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/basic_pin_invokable.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_basic_unknown_rust_obj_type() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_unknown_rust_obj_type.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header = clang_format(include_str!(
            "../test_outputs/basic_unknown_rust_obj_type.h"
        ))
        .unwrap();
        let expected_source = clang_format(include_str!(
            "../test_outputs/basic_unknown_rust_obj_type.cpp"
        ))
        .unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_subobject_property() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/subobject_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/subobject_property.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/subobject_property.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_subobject_pin_invokable() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/subobject_pin_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/subobject_pin_invokable.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/subobject_pin_invokable.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_basic_update_requester() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_update_requester.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/basic_update_requester.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/basic_update_requester.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_basic_change_handler() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/basic_change_handler.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/basic_change_handler.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/basic_change_handler.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_types_primitive_property() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/types_primitive_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/types_primitive_property.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/types_primitive_property.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_types_qt_property() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/types_qt_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/types_qt_property.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/types_qt_property.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_types_qt_invokable() {
        // TODO: we probably want to parse all the test case files we have
        // only once as to not slow down different tests on the same input.
        // This can maybe be done with some kind of static object somewhere.
        let source = include_str!("../test_inputs/types_qt_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/types_qt_invokable.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/types_qt_invokable.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }
}
