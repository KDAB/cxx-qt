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

use crate::extract::{Invokable, Parameter, ParameterType, Property, QObject, QtTypes, Signal};

/// A trait which we implement on QtTypes allowing retrieval of attributes of the enum value.
trait CppType {
    /// String representation of the const part of this type
    fn as_const_str(&self) -> &str;
    /// String representation of the pointer part of this type
    fn as_ptr_str(&self) -> &str;
    /// String representation of the ref part of this type
    fn as_ref_str(&self) -> &str;
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
                "#include \"cxx-qt-gen/include/{}.cxxqt.h\"",
                cpp_type_idents[cpp_type_idents.len() - 2]
            )],
            Self::QColor => vec!["#include <QtGui/QColor>".to_owned()],
            Self::QDate => vec!["#include <QtCore/QDate>".to_owned()],
            Self::QDateTime => vec!["#include <QtCore/QDateTime>".to_owned()],
            Self::QPoint => vec!["#include <QtCore/QPoint>".to_owned()],
            Self::QPointF => vec!["#include <QtCore/QPointF>".to_owned()],
            Self::QRect => vec!["#include <QtCore/QRect>".to_owned()],
            Self::QRectF => vec!["#include <QtCore/QRectF>".to_owned()],
            Self::QSize => vec!["#include <QtCore/QSize>".to_owned()],
            Self::QSizeF => vec!["#include <QtCore/QSizeF>".to_owned()],
            Self::QString => vec!["#include <QtCore/QString>".to_owned()],
            Self::QTime => vec!["#include <QtCore/QTime>".to_owned()],
            Self::QUrl => vec!["#include <QtCore/QUrl>".to_owned()],
            Self::QVariant => vec!["#include <QtCore/QVariant>".to_owned()],
            Self::UniquePtr { inner } => inner.include_paths(),
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
            Self::QColor => true,
            Self::QDate => true,
            Self::QDateTime => true,
            Self::QPoint => true,
            Self::QPointF => true,
            Self::QRect => true,
            Self::QRectF => true,
            Self::QSize => true,
            Self::QSizeF => true,
            Self::QString => true,
            Self::QTime => true,
            Self::QUrl => true,
            Self::QVariant => true,
            Self::U8 | Self::U16 | Self::U32 => false,
            Self::UniquePtr { .. } => true,
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
            Self::QColor => true,
            Self::QDate => true,
            Self::QDateTime => true,
            Self::QPoint => true,
            Self::QPointF => true,
            Self::QRect => true,
            Self::QRectF => true,
            Self::QSize => true,
            Self::QSizeF => true,
            Self::QString => true,
            Self::QTime => true,
            Self::QUrl => true,
            Self::QVariant => true,
            Self::U8 | Self::U16 | Self::U32 => false,
            Self::UniquePtr { .. } => true,
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
            Self::QColor => "QColor",
            Self::QDate => "QDate",
            Self::QDateTime => "QDateTime",
            Self::QPoint => "QPoint",
            Self::QPointF => "QPointF",
            Self::QRect => "QRect",
            Self::QRectF => "QRectF",
            Self::QSize => "QSize",
            Self::QSizeF => "QSizeF",
            Self::QString => "QString",
            Self::QTime => "QTime",
            Self::QUrl => "QUrl",
            Self::QVariant => "QVariant",
            Self::U8 => "quint8",
            Self::U16 => "quint16",
            Self::U32 => "quint32",
            // TODO: for now always automatically convert UniquePtr<T> to T
            // in C++, later this will require a macro attribute to do this
            // eg for properties, invokable returns, signals
            //
            // But this may be changed once the generation pattern matching has been removed
            Self::UniquePtr { inner } => inner.type_ident(),
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
/// Describes a C++ signal with header and source parts
#[derive(Debug)]
struct CppSignal {
    /// Any extra include that is required for the signal
    header_includes: Vec<String>,
    /// Any public methods that are defined by the signal
    header_public: Vec<String>,
    /// Any signals that are defined by the signal
    header_signals: Vec<String>,
    /// The source implementation of the signal
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
                body = if let Some(return_type) = return_type {
                    format!("return rust::cxxqtlib1::cxx_qt_convert<{output_type}, {input_type}>{{}}({body})",
                        body = body,
                        output_type = return_ident,
                        input_type = if return_type.is_opaque() {
                            format!("std::unique_ptr<{}>", return_ident)
                        } else {
                            return_ident.to_owned()
                        }
                    )
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

                        const auto signalSuccess = QMetaObject::invokeMethod(this, "{ident_changed}", Qt::QueuedConnection);
                        Q_ASSERT(signalSuccess);
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

                  const auto signalSuccess = QMetaObject::invokeMethod(this, "{ident_changed}", Qt::QueuedConnection);
                  Q_ASSERT(signalSuccess);
                }}
                "#,
                ident_changed = ident_changed,
                ident_pascal = parameter_ident_pascal,
                ident_setter = ident_setter,
                member_ident = member_ident,
                member_owned_ident = member_owned_ident,
                struct_ident = struct_ident.to_string(),
                type_ident = type_ident,
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

                        const auto signalSuccess = QMetaObject::invokeMethod(this, "{ident_changed}", Qt::QueuedConnection);
                        Q_ASSERT(signalSuccess);
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

/// Generate a CppProperty object containing the header and source of a given list of rust properties
fn generate_signals_cpp(
    struct_ident: &Ident,
    signals: &[Signal],
) -> Result<Vec<CppSignal>, TokenStream> {
    let mut items: Vec<CppSignal> = vec![];

    for signal in signals {
        let mut header_includes = vec![];
        let mut header_public = vec![];
        let mut header_signals = vec![];

        let queued_ident_cpp = signal.emit_ident.cpp_ident.to_string();
        let signal_ident_cpp = signal.signal_ident.cpp_ident.to_string();

        let type_ident_to_immediate = |type_ident: &ParameterType| -> String {
            format!(
                "{is_const} {type_ident}{is_ref}{is_ptr}",
                is_const = type_ident.qt_type.as_const_str(),
                is_ref = type_ident.qt_type.as_ref_str(),
                is_ptr = type_ident.qt_type.as_ptr_str(),
                type_ident = type_ident.qt_type.type_ident(),
            )
        };
        let type_ident_to_queued = |type_ident: &ParameterType| -> String {
            format!(
                "{type_ident}{is_ptr}",
                is_ptr = type_ident.qt_type.as_ptr_str(),
                type_ident = if type_ident.qt_type.is_opaque() {
                    format!("std::unique_ptr<{}>", type_ident.qt_type.type_ident())
                } else {
                    type_ident.qt_type.type_ident().to_owned()
                },
            )
        };

        let parameters_with_type = signal
            .parameters
            .iter()
            .map(|parameter| {
                header_includes.append(&mut parameter.type_ident.qt_type.include_paths());

                format!(
                    "{type_ident} {ident}",
                    ident = parameter.ident,
                    type_ident = type_ident_to_immediate(&parameter.type_ident),
                )
            })
            .collect::<Vec<String>>()
            .join(", ");

        let parameters_with_type_queued = signal
            .parameters
            .iter()
            .map(|parameter| {
                header_includes.append(&mut parameter.type_ident.qt_type.include_paths());

                format!(
                    "{type_ident} {ident}",
                    ident = parameter.ident,
                    type_ident = type_ident_to_queued(&parameter.type_ident),
                )
            })
            .collect::<Vec<String>>()
            .join(", ");

        header_public.push(format!(
            "void {ident}({parameters});",
            ident = queued_ident_cpp,
            parameters = parameters_with_type_queued
        ));
        header_signals.push(format!(
            "void {ident}({parameters});",
            ident = signal_ident_cpp,
            parameters = parameters_with_type
        ));

        // Note that we want a lambda by value (not reference) here so that we move any values
        let mut captures = vec!["this".to_owned()];
        let mut parameter_values = vec![];
        for parameter in &signal.parameters {
            let parameter_str = parameter.ident.to_string();
            captures.push(format!("{} = std::move({})", parameter_str, parameter_str));
            parameter_values.push(format!(
                "rust::cxxqtlib1::cxx_qt_convert<{output_type}, {input_type}>{{}}({ident})",
                ident = parameter_str,
                output_type = type_ident_to_immediate(&parameter.type_ident),
                input_type = type_ident_to_queued(&parameter.type_ident),
            ));
        }

        let source = formatdoc! {
            r#"
            void
            {struct_ident}::{queued_ident_cpp}({parameters})
            {{
                const auto signalSuccess = QMetaObject::invokeMethod(
                    this, [{captures}]() {{ Q_EMIT {signal_ident_cpp}({parameter_values}); }}, Qt::QueuedConnection);
                Q_ASSERT(signalSuccess);
            }}
            "#,
            queued_ident_cpp = queued_ident_cpp,
            signal_ident_cpp = signal_ident_cpp,
            struct_ident = struct_ident,
            captures = captures.join(", "),
            parameters = parameters_with_type_queued,
            parameter_values = parameter_values.join(", "),
        };

        items.push(CppSignal {
            header_includes,
            header_public,
            header_signals,
            source,
        })
    }

    Ok(items)
}

/// Generate a CppObject object containing the header and source of a given rust QObject
pub fn generate_qobject_cpp(obj: &QObject) -> Result<CppObject, TokenStream> {
    let struct_ident_str = obj.ident.to_string();
    let rust_struct_ident = format!("{}Rust", struct_ident_str);

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
    let properties = generate_properties_cpp(&obj.ident, &obj.properties)?
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

    // A helper which allows us to flatten data from vec of signals
    struct CppSignalHelper {
        headers_includes: Vec<String>,
        headers_public: Vec<String>,
        headers_signals: Vec<String>,
        sources: Vec<String>,
    }

    let mut signals = generate_signals_cpp(&obj.ident, &obj.signals)?
        .drain(..)
        .fold(
            CppSignalHelper {
                headers_includes: vec![],
                headers_public: vec![],
                headers_signals: vec![],
                sources: vec![],
            },
            |mut acc, mut signal| {
                acc.headers_includes.append(&mut signal.header_includes);
                acc.headers_public.append(&mut signal.header_public);
                acc.headers_signals.append(&mut signal.header_signals);
                acc.sources.push(signal.source);
                acc
            },
        );

    // If there are signals then prepare a string otherwise leave an empty string
    // We need to do this otherwise we are left with a Q_SIGNALS with nothing after it
    let cpp_signals = if properties.headers_signals.is_empty() && signals.headers_signals.is_empty()
    {
        "".to_owned()
    } else {
        let mut qt_signals = properties.headers_signals;
        qt_signals.append(&mut signals.headers_signals);

        formatdoc! {r#"
            Q_SIGNALS:
            {signals}
            "#,
            signals = qt_signals.join("\n"),
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

    let mut public_method_headers = vec![];
    let mut public_method_sources = vec![];

    if obj.handle_updates_impl.is_some() {
        public_method_headers
            .push("std::unique_ptr<rust::cxxqtlib1::UpdateRequester> updateRequester();");
        public_method_headers.push("Q_INVOKABLE void updateState();");

        public_method_sources.push(formatdoc! {r#"
            std::unique_ptr<rust::cxxqtlib1::UpdateRequester> {ident}::updateRequester() {{
                return std::make_unique<rust::cxxqtlib1::UpdateRequester>(this, "updateState");
            }}
        "#,
        ident = struct_ident_str,
        });
        public_method_sources.push(formatdoc! {r#"
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

        #include "cxx-qt-lib/include/qt_types.h"

        {includes}

        namespace {namespace} {{

        class {rust_struct_ident};

        class {ident} : public QObject {{
            Q_OBJECT
        {properties_meta}

        public:
            explicit {ident}(QObject *parent = nullptr);
            ~{ident}();

        {properties_public}

        {invokables}

        {public_method_headers}

        {signal_emitters}

        {public_slots}

        {signals}

        private:
            rust::Box<{rust_struct_ident}> m_rustObj;
            std::mutex m_rustObjMutex;
            bool m_initialised = false;

            {members_private}
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
        includes.append(&mut signals.headers_includes);
        // Sort and remove duplicates
        includes.sort();
        includes.dedup();
        includes.join("\n")
    },
    namespace = namespace,
    properties_meta = properties.headers_meta.join("\n"),
    properties_public = properties.headers_public.join("\n"),
    rust_struct_ident = rust_struct_ident,
    signals = cpp_signals,
    signal_emitters = signals.headers_public.join("\n"),
    public_slots = public_slots,
    public_method_headers = public_method_headers.join("\n"),
    };

    // Generate C++ source part
    let source = formatdoc! {r#"
        #include "cxx-qt-gen/include/{ident_snake}.cxx.h"
        #include "cxx-qt-gen/include/{ident_snake}.cxxqt.h"

        namespace {namespace} {{

        {ident}::{ident}(QObject *parent)
            : QObject(parent)
            , m_rustObj(createRs())
        {{
            initialiseCpp(*this);
            m_initialised = true;
        }}

        {ident}::~{ident}() = default;

        {properties}

        {invokables}

        {public_method_sources}

        {signals}

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
        public_method_sources = public_method_sources.join("\n"),
        signals = signals.sources.join("\n"),
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
    fn generates_handlers() {
        let source = include_str!("../test_inputs/handlers.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_header = clang_format(include_str!("../test_outputs/handlers.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/handlers.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_invokables() {
        let source = include_str!("../test_inputs/invokables.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_header = clang_format(include_str!("../test_outputs/invokables.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/invokables.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_naming() {
        let source = include_str!("../test_inputs/naming.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_header = clang_format(include_str!("../test_outputs/naming.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/naming.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_properties() {
        let source = include_str!("../test_inputs/properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_header = clang_format(include_str!("../test_outputs/properties.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/properties.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_signals() {
        let source = include_str!("../test_inputs/signals.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_header = clang_format(include_str!("../test_outputs/signals.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/signals.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_types_primitive_property() {
        let source = include_str!("../test_inputs/types_primitive_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

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
        let source = include_str!("../test_inputs/types_qt_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

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
        let source = include_str!("../test_inputs/types_qt_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(&module, &cpp_namespace_prefix).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/types_qt_invokable.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/types_qt_invokable.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_eq!(cpp_object.header, expected_header);
        assert_eq!(cpp_object.source, expected_source);
    }
}
