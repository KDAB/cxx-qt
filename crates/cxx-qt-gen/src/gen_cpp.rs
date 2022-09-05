// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use clang_format::{ClangFormatStyle, CLANG_FORMAT_STYLE};
use indoc::formatdoc;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::extract::{Invokable, Parameter, ParameterType, Property, QObject, QtTypes, Signal};
use crate::generator::cpp::{
    fragment::CppFragmentPair, qobject::GeneratedCppQObjectBlocks, GeneratedCppBlocks,
};
use crate::generator::{naming, naming::property::QPropertyName};
use crate::writer::cpp::write_cpp;

/// A trait which we implement on QtTypes allowing retrieval of attributes of the enum value.
trait CppType {
    /// String representation of the const part of this type
    fn as_const_str(&self) -> &str;
    /// String representation of the ref part of this type
    fn as_ref_str(&self) -> &str;
    /// Whether this type is a const (when used as an input to methods)
    fn is_const(&self) -> bool;
    /// Whether this type is a reference
    fn is_ref(&self) -> bool;
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

    /// String representation of the ref part of this type
    fn as_ref_str(&self) -> &str {
        if self.is_ref() {
            "&"
        } else {
            ""
        }
    }

    /// Whether this type is a const (when used as an input to methods)
    ///
    /// For now this means that we consider the type in C++ to be const
    /// eg String => const QString and not whether the rust type was const.
    fn is_const(&self) -> bool {
        match self {
            Self::Bool => false,
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

    /// The C++ type name of the CppType
    fn type_ident(&self) -> &str {
        match self {
            Self::Bool => "bool",
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
struct CppParameter<'a> {
    /// The ident of the parameter
    ident: String,
    /// The type of the parameter
    type_ident: &'a QtTypes,
}

/// Describes a C++ invokable with header and source parts
struct CppInvokable {
    /// The header definition of the invokable
    header: String,
    /// The source implementation of the invokable
    source: String,
}
/// Describes a C++ signal with header and source parts
struct CppSignal {
    /// Any public methods that are defined by the signal
    header_public: Vec<String>,
    /// Any signals that are defined by the signal
    header_signals: Vec<String>,
    /// The source implementation of the signal
    source: String,
}
/// Describes a C++ property with header and source parts
struct CppProperty {
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
            ident: parameter.ident.to_string(),
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
                    names: vec![],
                },
                |mut acc, parameter| {
                    // Build the parameter as a type argument
                    acc.args.push(format!(
                        "{is_const} {type_ident}{is_ref} {ident}",
                        ident = parameter.ident,
                        is_const = parameter.type_ident.as_const_str(),
                        is_ref = parameter.type_ident.as_ref_str(),
                        type_ident = parameter.type_ident.type_ident()
                    ));

                    // Build the parameter names
                    acc.names.push(parameter.ident);
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
            ident = invokable.ident_wrapper.cpp,
            parameter_names = (vec!["*this".to_owned()]
                .into_iter()
                .chain(parameters.names.into_iter()))
            .collect::<Vec<_>>()
            .join(", ")
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
            // TODO: also needs to consider if there is a Pin<&mut T> as we need non-const if
            // we are passing *this across for cpp objects in rust.
            header: format!(
                "Q_INVOKABLE {return_ident} {ident}({parameter_types});",
                ident = invokable.ident.cpp,
                parameter_types = parameter_arg_line,
                return_ident = return_ident,
            ),
            source: formatdoc! {
                r#"
                {return_ident} {struct_ident}::{ident}({parameter_types})
                {{
                    const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
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
                ident = invokable.ident.cpp.to_string(),
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
        let property_ident = QPropertyName::from(&property.ident);

        // Build a CppParameter for the name and type of the property
        let parameter = CppParameter {
            ident: property_ident.name.cpp.to_string(),
            type_ident: &property.type_ident.qt_type,
        };

        // Collect the C++ idents for the getter, setter, notify of the property
        //
        // TODO: for now we assume that all properties have a getter/setter/notify
        let ident_getter = property_ident.getter.cpp.to_string();
        let ident_setter = property_ident.setter.cpp.to_string();
        let ident_changed = property_ident.notify.cpp.to_string();
        let ident_emit = property_ident.emit.cpp.to_string();

        // Cache the type ident of the property as this is used multiple times
        let type_ident = parameter.type_ident.type_ident();

        // Build a basic C++ property with parts that are defined if the property is a pointer or not
        let mut cpp_property = CppProperty {
            // Set the Q_PROPERTY for the C++ class
            header_meta: vec![format!("Q_PROPERTY({type_ident} {ident} READ {ident_getter} WRITE {ident_setter} NOTIFY {ident_changed})",
                ident = parameter.ident,
                ident_changed = ident_changed,
                ident_getter = ident_getter,
                ident_setter = ident_setter,
                type_ident = type_ident,
            )],
            // Set basic getter, more are added later for only pointer
            header_public: vec![format!("const {type_ident}& {ident_getter}() const;",
                ident_getter = ident_getter,
                type_ident = type_ident,
            ), format!("void {ident_emit}();", ident_emit = ident_emit)],
            // Set the notify signals
            header_signals: vec![format!("void {ident_changed}();", ident_changed = ident_changed)],
            // Set the slots for the setter
            header_slots: vec![format!("void {ident_setter}(const {type_ident}& value);",
                ident_setter = ident_setter,
                type_ident = type_ident,
            )],
            // The source is created later
            source: vec![],
        };

        cpp_property.source.push(formatdoc! {
            r#"
            const {type_ident}&
            {struct_ident}::{ident_getter}() const
            {{
                const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
                return rust::cxxqtlib1::cxx_qt_convert<const {type_ident}&, const {rust_type_ident}&>{{}}(m_rustObj->{ident_getter}(*this));
            }}

            void
            {struct_ident}::{ident_setter}(const {type_ident}& value)
            {{
                const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
                m_rustObj->{ident_setter}(*this, rust::cxxqtlib1::cxx_qt_convert<{rust_type_ident}, const {type_ident}&>{{}}(value));
            }}

            void
            {struct_ident}::{ident_emit}()
            {{
                const auto signalSuccess = QMetaObject::invokeMethod(this, "{ident_changed}", Qt::QueuedConnection);
                Q_ASSERT(signalSuccess);
            }}
            "#,
            ident_changed = ident_changed,
            ident_getter = ident_getter,
            ident_setter = ident_setter,
            ident_emit = ident_emit,
            struct_ident = struct_ident.to_string(),
            type_ident = type_ident,
            rust_type_ident = if parameter.type_ident.is_opaque() {
                format!("std::unique_ptr<{}>", type_ident)
            } else {
                type_ident.to_string()
            },
        });

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
        let mut header_public = vec![];
        let mut header_signals = vec![];

        let queued_ident_cpp = signal.emit_ident.cpp.to_string();
        let signal_ident_cpp = signal.signal_ident.cpp.to_string();

        let type_ident_to_immediate = |type_ident: &ParameterType| -> String {
            format!(
                "{is_const} {type_ident}{is_ref}",
                is_const = type_ident.qt_type.as_const_str(),
                is_ref = type_ident.qt_type.as_ref_str(),
                type_ident = type_ident.qt_type.type_ident(),
            )
        };
        let type_ident_to_queued = |type_ident: &ParameterType| -> String {
            if type_ident.qt_type.is_opaque() {
                format!("std::unique_ptr<{}>", type_ident.qt_type.type_ident())
            } else {
                type_ident.qt_type.type_ident().to_owned()
            }
        };

        let parameters_with_type = signal
            .parameters
            .iter()
            .map(|parameter| {
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
            header_public,
            header_signals,
            source,
        })
    }

    Ok(items)
}

/// Generate a CppObject object containing the header and source of a given rust QObject
pub fn generate_qobject_cpp(obj: &QObject) -> Result<CppObject, TokenStream> {
    let qobject_idents = naming::qobject::QObjectName::from(&obj.ident);
    let rust_struct_ident = qobject_idents.rust_struct.cpp.to_string();

    // TODO: For now we proxy the gen_cpp code into what the writer phase expects
    // later this code will be moved into a generator phase
    let mut metaobjects: Vec<String> = vec![];
    let mut methods: Vec<CppFragmentPair> = vec![];
    let mut signals: Vec<String> = vec![];
    let mut slots: Vec<CppFragmentPair> = vec![];

    for mut property in generate_properties_cpp(&obj.ident, &obj.properties)?.drain(..) {
        metaobjects.append(&mut property.header_meta);
        methods.append(
            &mut property
                .header_public
                .drain(..)
                .map(|header| CppFragmentPair {
                    header,
                    source: "".to_owned(),
                })
                .collect::<Vec<CppFragmentPair>>(),
        );
        signals.append(&mut property.header_signals);
        slots.append(
            &mut property
                .header_slots
                .drain(..)
                .map(|header| CppFragmentPair {
                    header,
                    source: "".to_owned(),
                })
                .collect::<Vec<CppFragmentPair>>(),
        );
        methods.append(
            &mut property
                .source
                .drain(..)
                .map(|source| CppFragmentPair {
                    header: "".to_owned(),
                    source,
                })
                .collect::<Vec<CppFragmentPair>>(),
        );
    }

    for invokable in generate_invokables_cpp(&obj.ident, &obj.invokables)?.drain(..) {
        methods.push(CppFragmentPair {
            header: invokable.header,
            source: invokable.source,
        });
    }

    for mut signal in generate_signals_cpp(&obj.ident, &obj.signals)?.drain(..) {
        methods.append(
            &mut signal
                .header_public
                .drain(..)
                .map(|header| CppFragmentPair {
                    header,
                    source: "".to_owned(),
                })
                .collect::<Vec<CppFragmentPair>>(),
        );
        signals.append(&mut signal.header_signals);
        methods.push(CppFragmentPair {
            header: "".to_owned(),
            source: signal.source,
        });
    }

    // Create the namespace for internal use
    let namespace_internals =
        naming::namespace::NamespaceName::from_pair_str(&obj.namespace, &obj.ident).internal;

    // For now we only create a single QObject
    let qobjects = vec![GeneratedCppQObjectBlocks {
        ident: obj.ident.to_string(),
        rust_ident: rust_struct_ident,
        cxx_qt_thread_ident: qobject_idents.cxx_qt_thread_class.to_string(),
        namespace_internals,
        base_class: obj
            .base_class
            .clone()
            .unwrap_or_else(|| "QObject".to_owned()),
        metaobjects,
        methods,
        slots,
        signals,
    }];

    // For now convert our gen_cpp code into the GeneratedCppBlocks struct
    let generated = GeneratedCppBlocks {
        cxx_stem: naming::module::cxx_stem_from_ident(&obj.ident).to_string(),
        namespace: obj.namespace.clone(),
        qobjects,
    };

    // Use our writer phase to convert to a string
    let writer = write_cpp(&generated);

    Ok(CppObject {
        header: writer.header,
        source: writer.source,
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
    use clang_format::clang_format;

    use pretty_assertions::assert_str_eq;
    use syn::ItemMod;

    #[test]
    fn generates_invokables() {
        let source = include_str!("../test_inputs/invokables.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_header = clang_format(include_str!("../test_outputs/invokables.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/invokables.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_str_eq!(cpp_object.header, expected_header);
        assert_str_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_naming() {
        let source = include_str!("../test_inputs/naming.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_header = clang_format(include_str!("../test_outputs/naming.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/naming.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_str_eq!(cpp_object.header, expected_header);
        assert_str_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_signals() {
        let source = include_str!("../test_inputs/signals.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_header = clang_format(include_str!("../test_outputs/signals.h")).unwrap();
        let expected_source = clang_format(include_str!("../test_outputs/signals.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_str_eq!(cpp_object.header, expected_header);
        assert_str_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_types_primitive_property() {
        let source = include_str!("../test_inputs/types_primitive_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/types_primitive_property.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/types_primitive_property.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_str_eq!(cpp_object.header, expected_header);
        assert_str_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_types_qt_property() {
        let source = include_str!("../test_inputs/types_qt_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/types_qt_property.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/types_qt_property.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_str_eq!(cpp_object.header, expected_header);
        assert_str_eq!(cpp_object.source, expected_source);
    }

    #[test]
    fn generates_types_qt_invokable() {
        let source = include_str!("../test_inputs/types_qt_invokable.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        let expected_header =
            clang_format(include_str!("../test_outputs/types_qt_invokable.h")).unwrap();
        let expected_source =
            clang_format(include_str!("../test_outputs/types_qt_invokable.cpp")).unwrap();
        let cpp_object = generate_qobject_cpp(&qobject).unwrap();
        assert_str_eq!(cpp_object.header, expected_header);
        assert_str_eq!(cpp_object.source, expected_source);
    }
}
