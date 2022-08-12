// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::parser::{signals::ParsedSignalsEnum, Parser};
use convert_case::{Case, Casing};
use derivative::*;
use proc_macro2::{Span, TokenStream};
use std::result::Result;
use syn::{spanned::Spanned, token::Brace, *};

/// Describes an ident which has a different name in C++ and Rust
#[derive(Debug, PartialEq)]
pub(crate) struct CppRustIdent {
    /// The ident for C++
    pub(crate) cpp_ident: Ident,
    /// The ident for rust
    pub(crate) rust_ident: Ident,
}

/// Describes a Qt type
#[derive(Debug, PartialEq)]
pub(crate) enum QtTypes {
    Bool,
    /// A CppObj which is being passed as a parameter in a method
    CppObj {
        /// The ident of the type for C++, eg the MyObject or CppObj from:
        /// C++ - MyObject or cxx_qt::sub_object::CppObj
        cpp_type_idents: Vec<Ident>,
        /// A cache of the C++ type as a string with the namespace
        /// eg "cxx_qt::module::CppObj"
        cpp_type_idents_string: String,
        /// The ident of the type for Rust, eg the MyObject or CppObj from:
        /// Rust - MyObject or crate::sub_object::CppObj
        rust_type_idents: Vec<Ident>,
        /// The combined name of the type, this is a single word which is used
        /// as the type name of the C++ type in the Rust cxx bridge
        /// eg MyObject or SubObject
        /// This needs the match the name of the C++ class without namespace
        /// as we add this in the macro attribute
        //
        // TODO: later can we use the fully qualified path
        // eg crate::my_module::CppObj to Crate_MyModule_CppObj?
        combined_name: Ident,
    },
    F32,
    F64,
    I8,
    I16,
    I32,
    QColor,
    QDate,
    QDateTime,
    QPoint,
    QPointF,
    QRect,
    QRectF,
    QSize,
    QSizeF,
    QString,
    QTime,
    QUrl,
    QVariant,
    U8,
    U16,
    U32,
    UniquePtr {
        inner: Box<QtTypes>,
    },
    Unknown,
}

impl QtTypes {
    /// Whether this type is allowed to be a ref mut
    fn ref_mut_is_valid(&self) -> bool {
        match self {
            Self::CppObj { .. } => true,
            _others => false,
        }
    }

    /// Whether this type is opaque so will be a UniquePtr<T> when returned from Rust to C++
    pub(crate) fn is_opaque(&self) -> bool {
        match self {
            Self::CppObj { .. } => true,
            Self::UniquePtr { .. } => true,
            _others => false,
        }
    }
}

/// Describes a type
#[derive(Debug)]
pub(crate) struct ParameterType {
    /// If this parameter is mutable
    pub(crate) is_mut: bool,
    /// If this parameter is a reference
    pub(crate) is_ref: bool,
    /// The detected Qt type of the parameter
    pub(crate) qt_type: QtTypes,
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
    pub(crate) ident: CppRustIdent,
    /// If the invokable needs a wrapper, this is it's ident
    pub(crate) ident_wrapper: Option<CppRustIdent>,
    /// The parameters that the function takes in
    pub(crate) parameters: Vec<Parameter>,
    /// The return type information
    pub(crate) return_type: Option<ParameterType>,
    /// Whether this invokable is using mut self or not
    pub(crate) mutable: bool,
    /// The original Rust method for the invokable
    #[derivative(Debug = "ignore")]
    pub(crate) original_method: ImplItemMethod,
}

/// Describes a property that can be used from QML
#[derive(Debug)]
pub(crate) struct Property {
    /// The ident of the property
    pub(crate) ident: CppRustIdent,
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

/// Describes a signal that can be used from QML
#[derive(Debug)]
pub(crate) struct Signal {
    /// The C++ and Rust names of the method to emit the signal as queued
    /// eg emitDataChanged and emit_data_changed
    pub(crate) emit_ident: CppRustIdent,
    /// The Rust name of the enum entry, eg DataChanged
    pub(crate) enum_ident: Ident,
    /// The parameters of the Signal
    pub(crate) parameters: Vec<Parameter>,
    /// The C++ and Rust names of the method to emit the signal as immediate
    /// eg dataChanged and data_changed
    pub(crate) signal_ident: CppRustIdent,
}

/// Describes all the properties of a QObject class
#[derive(Debug)]
pub struct QObject {
    /// The ident of the C++ class that represents the QObject
    pub ident: Ident,
    /// All the methods that can also be invoked from QML
    pub(crate) invokables: Vec<Invokable>,
    /// All the methods that cannot be invoked from QML or C++, but are in the context of C++
    pub(crate) methods: Vec<ImplItemMethod>,
    /// All the properties that can be used from QML
    pub(crate) properties: Vec<Property>,
    /// All the signals that can be emitted/connected from QML
    pub(crate) signals: Vec<Signal>,
    /// The name of the signals enum that is used
    pub(crate) signal_ident: Option<Ident>,
    /// The namespace to use for C++
    pub(crate) namespace: String,
    /// Items we just pass through to the CXX bridge
    pub(crate) cxx_items: Vec<Item>,
    /// The original Rust mod for the struct
    pub(crate) original_mod: ItemMod,
    /// The original Data struct that the object was generated from
    pub(crate) original_data_struct: ItemStruct,
    /// The original Rust struct that the object was generated from
    pub(crate) original_rust_struct: ItemStruct,
    /// The original Signal enum that the signals were generated from
    pub(crate) original_signal_enum: Option<ItemEnum>,
    /// The original Rust declarations from the mod that will be directly passed through
    pub(crate) original_passthrough_decls: Vec<Item>,
    /// The Rust impl that has optionally been provided to handle updates
    pub(crate) handle_updates_impl: Option<ItemImpl>,
    /// The base class of the QObject
    pub(crate) base_class: Option<String>,
}

/// Describe the error type from extract_qt_type and extract_type_ident
enum ExtractTypeIdentError {
    /// We do not support AngleBracketed or Parenthesized rust types
    InvalidArguments(Span),
    /// This is not a valid rust type
    InvalidType(Span),
    /// There are no idents in the type
    IdentEmpty(Span),
    /// There are multiple idents but didn't start with crate::
    UnknownAndNotCrate(Span),
}

/// Extract the Qt type from a list of Ident's
fn extract_qt_type(
    idents: &[Ident],
    original_ty: &syn::Type,
    qt_ident: &Ident,
) -> Result<QtTypes, ExtractTypeIdentError> {
    // TODO: can we support generic Qt types as well eg like QObject or QAbstractListModel?
    // so that QML can set a C++/QML type into the property ? or is that not useful?

    // Check that the type has at least one ident
    if idents.is_empty() {
        Err(ExtractTypeIdentError::IdentEmpty(original_ty.span()))
    // If there is one entry then try to convert using our defined types
    } else if idents.len() == 1 {
        // We can assume that idents has an entry at index zero, because there is one entry
        match idents[0].to_string().as_str() {
            "bool" => Ok(QtTypes::Bool),
            "CppObj" => Ok(QtTypes::CppObj {
                cpp_type_idents: vec![qt_ident.clone()],
                cpp_type_idents_string: qt_ident.to_string(),
                rust_type_idents: vec![quote::format_ident!("{}Qt", qt_ident)],
                combined_name: qt_ident.clone(),
            }),
            "f32" => Ok(QtTypes::F32),
            "f64" => Ok(QtTypes::F64),
            "i8" => Ok(QtTypes::I8),
            "i16" => Ok(QtTypes::I16),
            "i32" => Ok(QtTypes::I32),
            "QColor" => Ok(QtTypes::QColor),
            "QDate" => Ok(QtTypes::QDate),
            "QDateTime" => Ok(QtTypes::QDateTime),
            "QPoint" => Ok(QtTypes::QPoint),
            "QPointF" => Ok(QtTypes::QPointF),
            "QRect" => Ok(QtTypes::QRect),
            "QRectF" => Ok(QtTypes::QRectF),
            "QSize" => Ok(QtTypes::QSize),
            "QSizeF" => Ok(QtTypes::QSizeF),
            "QString" => Ok(QtTypes::QString),
            "QTime" => Ok(QtTypes::QTime),
            "QUrl" => Ok(QtTypes::QUrl),
            "QVariant" => Ok(QtTypes::QVariant),
            "u8" => Ok(QtTypes::U8),
            "u16" => Ok(QtTypes::U16),
            "u32" => Ok(QtTypes::U32),
            _other => Ok(QtTypes::Unknown),
        }
    // This is a UniquePtr<T> field
    } else if idents.len() > 1 && idents.first().unwrap().to_string().as_str() == "UniquePtr" {
        Ok(QtTypes::UniquePtr {
            inner: Box::new(extract_qt_type(&idents[1..], original_ty, qt_ident)?),
        })
    // This is an unknown type that did not start with crate and has multiple parts
    } else {
        // We can assume that idents has an entry at index zero, because it is not empty
        Err(ExtractTypeIdentError::UnknownAndNotCrate(idents[0].span()))
    }
}

/// Converts a given path to a vector of idents
fn path_to_idents(path: &syn::Path) -> Result<Vec<Ident>, ExtractTypeIdentError> {
    // We do support UniquePtr<T> for now
    if let Some(segment) = path.segments.first() {
        if segment.ident == "UniquePtr" {
            if let PathArguments::AngleBracketed(angled) = &segment.arguments {
                if let Some(GenericArgument::Type(Type::Path(type_path))) = angled.args.first() {
                    return path_to_idents(&type_path.path).map(|mut idents| {
                        idents.insert(0, quote::format_ident!("UniquePtr"));
                        idents
                    });
                }
            }
        }
    }

    path.segments
        .iter()
        .map(|segment| {
            // We do not support PathArguments for types in properties or arguments
            //
            // eg we do not support AngleBracketed - the <'a, T> in std::slice::iter<'a, T>
            // eg we do not support Parenthesized - the (A, B) -> C in Fn(A, B) -> C
            if segment.arguments == PathArguments::None {
                return Ok(segment.ident.to_owned());
            }

            Err(ExtractTypeIdentError::InvalidArguments(segment.span()))
        })
        .collect::<Result<Vec<Ident>, ExtractTypeIdentError>>()
}

/// Extract the type ident from a given syn::Type
fn extract_type_ident(
    ty: &syn::Type,
    qt_ident: &Ident,
) -> Result<ParameterType, ExtractTypeIdentError> {
    // Temporary storage of the current syn::TypePath if one is found
    let ty_path;
    let is_mut;
    // Whether this syn::Type is a reference or not
    let is_ref;

    match ty {
        // The type is simply a path (eg std::slice::Iter)
        Type::Path(path) => {
            is_mut = false;
            is_ref = false;
            ty_path = path;
        }
        // The type is a reference, so see if it contains a path
        Type::Reference(TypeReference {
            mutability, elem, ..
        }) => {
            // If the type is a path then extract it and mark is_ref
            if let Type::Path(path) = &**elem {
                is_mut = mutability.is_some();
                is_ref = true;
                ty_path = path;
            } else {
                return Err(ExtractTypeIdentError::InvalidType(ty.span()));
            }
        }
        _others => {
            return Err(ExtractTypeIdentError::InvalidType(ty.span()));
        }
    }

    let idents = path_to_idents(&ty_path.path)?;
    // Extract the Qt type this is used in C++ and Rust generation
    let qt_type = extract_qt_type(&idents, ty, qt_ident)?;

    // Check if this Qt type is allowed to be a ref mut
    if is_mut && is_ref && !qt_type.ref_mut_is_valid() {
        return Err(ExtractTypeIdentError::InvalidType(ty.span()));
    }

    // Create and return a ParameterType
    Ok(ParameterType {
        is_mut,
        is_ref,
        qt_type,
    })
}

/// Extract the parameters for a given ImplItemMethod
pub(crate) fn extract_method_params(
    method: &ImplItemMethod,
    qt_ident: &Ident,
) -> Result<Vec<Parameter>, TokenStream> {
    method.sig.inputs
        .iter()
        .map(|parameter| {
            // Check that the parameter is typed
            //
            // If it is not typed (it is a syn::Receiver) then this means it is the self parameter
            // but without a type, eg self: Box<Self> would be Typed
            //
            // TODO: does this mean that if self is Typed we need to skip it?
            // so should we ignore the first parameter if it is named "self"?
            if let FnArg::Typed(PatType { pat, ty, .. }) = parameter {
                // The type ident of the parameter
                let type_ident;

                // Try to extract the name of the parameter
                let parameter_ident = if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                    ident
                } else {
                    return Err(
                        Error::new(parameter.span(), "Invalid argument ident format.")
                            .to_compile_error(),
                    );
                };

                // Try to extract the type of the parameter
                match extract_type_ident(ty, qt_ident) {
                    Ok(result) => type_ident = result,
                    Err(ExtractTypeIdentError::InvalidArguments(span)) => {
                        return Err(Error::new(
                            span,
                            "Argument should not be angle bracketed or parenthesized.",
                        )
                        .to_compile_error());
                    }
                    Err(ExtractTypeIdentError::InvalidType(span)) => {
                        return Err(
                            Error::new(span, "Invalid argument ident format.").to_compile_error()
                        )
                    }
                    Err(ExtractTypeIdentError::IdentEmpty(span)) => {
                        return Err(Error::new(span, "Argument type ident must have at least one segment").to_compile_error())
                    }
                    Err(ExtractTypeIdentError::UnknownAndNotCrate(span)) => {
                        return Err(Error::new(span, "First argument type ident segment must start with 'crate' if there are multiple").to_compile_error())
                    }
                }

                // Build and push the parameter
                Ok(Some(Parameter {
                    ident: parameter_ident.to_owned(),
                    type_ident,
                }))
            } else {
                Ok(None)
            }
        })
        // Turn Result<Option<T>, E> -> Option<Result<T, E>>
        //
        // If the Option<T> is None then we return None
        // If the Option<T> is Some then we return Some(Ok(T))
        // If the Result is an Err then we return Some(Err(E))
        .filter_map(|result| result.map_or_else(|e| Some(Err(e)), |v| v.map(Ok)))
        // Collect the Vec<Result<T, E>> into Result<Vec<T>, E>
        .collect::<Result<Vec<Parameter>, TokenStream>>()
}

/// Return whether the first parameter of a method is a mutable self argument
//
// Note that self: Box<Self> is parsed as FnArg::Typed not FnArg::Receiver so will be false
// but we don't use this case with CXX, so this can be ignored.
fn is_method_mutable(method: &ImplItemMethod) -> bool {
    if let Some(FnArg::Receiver(Receiver { mutability, .. })) = method.sig.inputs.first() {
        return mutability.is_some();
    }

    false
}

fn extract_invokable(method: &ImplItemMethod, qt_ident: &Ident) -> Result<Invokable, TokenStream> {
    let method_ident = &method.sig.ident;
    let output = &method.sig.output;

    let mutable = is_method_mutable(method);
    let parameters = extract_method_params(method, qt_ident)?;

    let return_type = if let ReturnType::Type(_, ty) = output {
        // This output has a return type, so extract the type
        match extract_type_ident(ty, qt_ident) {
            Ok(result) => Some(result),
            Err(ExtractTypeIdentError::InvalidArguments(span)) => {
                return Err(Error::new(
                    span,
                    "Return type should not be angle bracketed or parenthesized.",
                )
                .to_compile_error());
            }
            Err(ExtractTypeIdentError::InvalidType(span)) => {
                return Err(Error::new(span, "Invalid return type format.").to_compile_error())
            }
            Err(ExtractTypeIdentError::IdentEmpty(span)) => {
                return Err(
                    Error::new(span, "Return type ident must have at least one segment")
                        .to_compile_error(),
                )
            }
            Err(ExtractTypeIdentError::UnknownAndNotCrate(span)) => {
                return Err(Error::new(
                    span,
                    "First return type ident segment must start with 'crate' if there are multiple",
                )
                .to_compile_error())
            }
        }
    } else {
        None
    };

    let ident_str = method_ident.to_string();
    let ident_method = CppRustIdent {
        cpp_ident: quote::format_ident!("{}", ident_str.to_case(Case::Camel)),
        rust_ident: quote::format_ident!("{}", ident_str.to_case(Case::Snake)),
    };

    // We need a wrapper for any opaque types or pointers in the parameters or return types
    let ident_wrapper = if return_type
        .as_ref()
        .map_or(false, |return_type| return_type.qt_type.is_opaque())
        || parameters
            .iter()
            .any(|parameter| parameter.type_ident.qt_type.is_opaque())
    {
        Some(CppRustIdent {
            cpp_ident: quote::format_ident!("{}Wrapper", ident_method.cpp_ident),
            rust_ident: quote::format_ident!("{}_wrapper", ident_method.rust_ident),
        })
    } else {
        None
    };

    Ok(Invokable {
        ident: ident_method,
        ident_wrapper,
        mutable,
        parameters,
        return_type,
        original_method: method.to_owned(), // TODO: remove to_owned once extract_invokable is split
    })
}

/// Extracts all the attributes from a struct and generates properties from them
fn extract_properties(s: &ItemStruct, qt_ident: &Ident) -> Result<Vec<Property>, TokenStream> {
    let mut properties = Vec::new();

    // TODO: we need to set up an exclude list of properties names and give
    // the user an error if they use one of those names.
    // For instance "rustObj" is not allowed as that would cause a collision.

    // Read the properties from the struct
    //
    // Extract only the named fields (eg "Point { x: f64, y: f64 }") and ignore any
    // unnamed fields (eg "Some(T)") or units (eg "None")
    if let ItemStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    } = s
    {
        // Process each named field individually
        for name in named {
            // Extract only fields with an ident (should be all as these are named fields).
            if let Field {
                // TODO: later we'll need to read the attributes (eg qt_property) here
                // attrs,
                ident: Some(ident),
                ty,
                ..
            } = name
            {
                // Extract the type of the field
                let type_ident;

                match extract_type_ident(ty, qt_ident) {
                    Ok(result) => type_ident = result,
                    Err(ExtractTypeIdentError::InvalidArguments(span)) => {
                        return Err(Error::new(
                            span,
                            "Named field should not be angle bracketed or parenthesized.",
                        )
                        .to_compile_error());
                    }
                    Err(ExtractTypeIdentError::InvalidType(span)) => {
                        return Err(
                            Error::new(span, "Invalid name field ident format.").to_compile_error()
                        )
                    }
                    Err(ExtractTypeIdentError::IdentEmpty(span)) => {
                        return Err(Error::new(span, "Named field type ident must have at least one segment").to_compile_error())
                    }
                    Err(ExtractTypeIdentError::UnknownAndNotCrate(span)) => {
                        return Err(Error::new(span, "First named field type ident segment must start with 'crate' if there are multiple").to_compile_error())
                    }
                }

                // Build the getter/setter/notify idents with their Rust and C++ idents
                //
                // TODO: later these can be optional and have custom names from macro attributes
                //
                // TODO: we might also need to store whether a custom method is already implemented
                // or whether a method needs to be auto generated on the rust side
                //
                // TODO: later support an attribute to keep original or override renaming
                let ident_str = ident.to_string();
                let ident_prop = CppRustIdent {
                    cpp_ident: quote::format_ident!("{}", ident_str.to_case(Case::Camel)),
                    rust_ident: quote::format_ident!("{}", ident_str.to_case(Case::Snake)),
                };
                let getter = Some(CppRustIdent {
                    cpp_ident: quote::format_ident!("get{}", ident_str.to_case(Case::Pascal)),
                    rust_ident: quote::format_ident!("{}", ident_str.to_case(Case::Snake)),
                });
                let setter = Some(CppRustIdent {
                    cpp_ident: quote::format_ident!("set{}", ident_str.to_case(Case::Pascal)),
                    rust_ident: quote::format_ident!("set_{}", ident_str.to_case(Case::Snake)),
                });
                let notify = Some(CppRustIdent {
                    cpp_ident: quote::format_ident!("{}Changed", ident_str.to_case(Case::Camel)),
                    // TODO: rust doesn't have notify on it's side?
                    rust_ident: quote::format_ident!("{}", ident_str.to_case(Case::Snake)),
                });

                // Build and push the property
                properties.push(Property {
                    ident: ident_prop,
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

/// Extracts all the fields from an enum and generates signals from them
//
// TODO: for now we still extract the ParsedSignals into a Signal blocks
// later when we have the generate phase this will be removed
fn extract_signals(
    signals: &ParsedSignalsEnum,
    qt_ident: &Ident,
) -> Result<Vec<Signal>, TokenStream> {
    signals.signals.iter().map(|signal| {
        let ident_str = signal.ident.to_string();
        Ok(Signal {
            emit_ident: CppRustIdent {
                cpp_ident: quote::format_ident!("emit{}", ident_str.to_case(Case::Pascal)),
                rust_ident: quote::format_ident!("emit_{}", ident_str.to_case(Case::Snake)),
            },
            enum_ident: signal.ident.clone(),
            parameters: signal.parameters.iter().map(|parameter| {
                Ok(Parameter {
                    ident: parameter.ident.clone(),
                    type_ident: match extract_type_ident(&parameter.ty, qt_ident) {
                        Ok(result) => result,
                        Err(ExtractTypeIdentError::InvalidArguments(span)) => {
                            return Err(Error::new(
                                span,
                                "Named field should not be angle bracketed or parenthesized.",
                            )
                            .to_compile_error());
                        }
                        Err(ExtractTypeIdentError::InvalidType(span)) => {
                            return Err(
                                Error::new(span, "Invalid name field ident format.").to_compile_error()
                            )
                        }
                        Err(ExtractTypeIdentError::IdentEmpty(span)) => {
                            return Err(Error::new(span, "Named field type ident must have at least one segment").to_compile_error())
                        }
                        Err(ExtractTypeIdentError::UnknownAndNotCrate(span)) => {
                            return Err(Error::new(span, "First named field type ident segment must start with 'crate' if there are multiple").to_compile_error())
                        }
                    },
                })
            }).collect::<Result<Vec<Parameter>, TokenStream>>()?,
            signal_ident: CppRustIdent {
                cpp_ident: quote::format_ident!("{}", ident_str.to_case(Case::Camel)),
                rust_ident: quote::format_ident!("{}", ident_str.to_case(Case::Snake)),
            },
        })
    }).collect()
}

/// Parses a module in order to extract a QObject description from it
pub fn extract_qobject(module: &ItemMod) -> Result<QObject, TokenStream> {
    // Build a parser for the given ItemMod
    //
    // TODO: in the future steps from this extract.rs file will be moved into module parts
    // of Parser
    let mut parser = Parser::from(module.to_owned()).map_err(|err| err.to_compile_error())?;

    // TODO: for now we only support one QObject per ItemMod block
    // so extract the first qobject we find
    if parser.cxx_qt_data.qobjects.len() != 1 {
        return Err(Error::new(
            module.span(),
            "Only one QObject is currently supported in the ItemMod.",
        )
        .to_compile_error());
    }
    let (qt_ident, qobject) = parser.cxx_qt_data.qobjects.drain().take(1).next().unwrap();

    // Find the items from the module
    let original_mod = parser.passthrough_module.clone();

    // Prepare variables to store struct, invokables, and other data
    //
    // The original Data Item::Struct if one is found
    let original_data_struct = qobject.data_struct;
    // The original #[cxx_qt::qobject] marked struct Item::Struct if one is found
    //
    // qobject_struct will always exist if we have a qobject, so unwrap for now
    let original_rust_struct = qobject.qobject_struct.unwrap();

    // A list of the invokables for the struct
    let object_invokables = qobject
        .invokables
        .iter()
        .map(|invokable| extract_invokable(invokable, &qt_ident))
        .collect::<Result<Vec<Invokable>, TokenStream>>()?;
    // A list of the normal methods (i.e. not invokables) for the struct
    let object_methods = qobject.methods.to_vec();
    // A list of insignificant declarations for the mod that will be directly passed through (eg `use crate::thing`)
    let original_passthrough_decls = parser
        .cxx_qt_data
        .uses
        .iter()
        .chain(qobject.others.iter())
        .cloned()
        .collect::<Vec<Item>>();
    // A list of items we will pass through to the CXX bridge
    //
    // TODO: for now this just includes ItemForeignMod but later this will switch to all non CXX-Qt items
    let cxx_items = parser
        .passthrough_module
        .content
        .unwrap_or((Brace::default(), vec![]))
        .1;

    // Determines if (and how) this object can respond to update requests
    let handle_updates_impl = qobject.update_requester_handler;

    // Read properties from the Data struct
    let object_properties = if let Some(ref original_struct) = original_data_struct {
        extract_properties(original_struct, &qt_ident)?
    } else {
        vec![]
    };

    // Read signals from the Signal enum
    //
    // TODO: for now we still extract the ParsedSignals into a Signal blocks
    // later when we have the generate phase this will be removed
    let object_signals = if let Some(signals) = &qobject.signals {
        extract_signals(signals, &qt_ident)?
    } else {
        vec![]
    };
    let signal_ident = qobject
        .signals
        .as_ref()
        .map(|signals| signals.ident.clone());
    let original_signal_enum = qobject.signals.map(|signals| signals.item);

    Ok(QObject {
        ident: qt_ident,
        invokables: object_invokables,
        methods: object_methods,
        properties: object_properties,
        signals: object_signals,
        signal_ident,
        namespace: parser.cxx_qt_data.namespace,
        cxx_items,
        original_mod,
        original_data_struct: original_data_struct
            .unwrap_or_else(|| syn::parse_str("pub struct Data;").unwrap()),
        original_signal_enum,
        original_rust_struct,
        original_passthrough_decls,
        handle_updates_impl,
        base_class: qobject.base_class,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn parses_custom_default() {
        let source = include_str!("../test_inputs/custom_default.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        // Check that it got the invokables and properties
        assert_eq!(qobject.invokables.len(), 0);
        assert_eq!(qobject.properties.len(), 1);

        assert_eq!(qobject.original_passthrough_decls.len(), 2);

        // Check that impl Default was found for Data
        if let Item::Impl(trait_impl) = &qobject.original_passthrough_decls[0] {
            if let Type::Path(TypePath { path, .. }) = &*trait_impl.self_ty {
                assert_eq!(path.segments.len(), 1);
                assert_eq!(path.segments[0].ident.to_string(), "Data");
            } else {
                panic!("Trait impl was not a TypePath");
            }
        } else {
            panic!("Expected an impl block");
        }

        // Check that impl Default was found for RustObj
        if let Item::Impl(trait_impl) = &qobject.original_passthrough_decls[1] {
            if let Type::Path(TypePath { path, .. }) = &*trait_impl.self_ty {
                assert_eq!(path.segments.len(), 1);
                assert_eq!(path.segments[0].ident.to_string(), "MyObject");
            } else {
                panic!("Trait impl was not a TypePath");
            }
        } else {
            panic!("Expected an impl block");
        }
    }

    #[test]
    fn parses_invokables() {
        let source = include_str!("../test_inputs/invokables.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        // Check that it got the names right
        assert_eq!(qobject.ident.to_string(), "MyObject");
        assert_eq!(qobject.original_mod.ident.to_string(), "ffi");
        assert_eq!(qobject.original_rust_struct.ident.to_string(), "MyObject");

        // Check that it got the invokables
        assert_eq!(qobject.invokables.len(), 9);

        let mut invokables = qobject.invokables.into_iter();
        // Check empty invokable ident
        let invokable = invokables.next().unwrap();
        assert_eq!(invokable.ident.cpp_ident.to_string(), "invokable");
        assert_eq!(invokable.ident.rust_ident.to_string(), "invokable");
        assert_eq!(invokable.parameters.len(), 0);
        assert!(invokable.return_type.is_none());
        assert!(!invokable.mutable);

        // Check CppObj invokable
        let invokable = invokables.next().unwrap();
        assert_eq!(invokable.ident.cpp_ident.to_string(), "invokableCppObj");
        assert_eq!(invokable.ident.rust_ident.to_string(), "invokable_cpp_obj");
        assert_eq!(invokable.parameters.len(), 1);
        assert!(invokable.return_type.is_none());
        assert!(!invokable.mutable);
        let parameter = &invokable.parameters[0];
        assert_eq!(parameter.ident.to_string(), "cpp");
        assert!(std::matches!(
            &parameter.type_ident.qt_type,
            QtTypes::CppObj { .. }
        ));
        assert!(parameter.type_ident.is_ref);
        assert!(parameter.type_ident.is_mut);

        // Check the mutable invokable
        let invokable = invokables.next().unwrap();
        assert_eq!(invokable.ident.cpp_ident.to_string(), "invokableMutable");
        assert_eq!(invokable.ident.rust_ident.to_string(), "invokable_mutable");
        assert_eq!(invokable.parameters.len(), 0);
        assert!(invokable.return_type.is_none());
        assert!(invokable.mutable);

        // Check the mutable CppObj invokable
        let invokable = invokables.next().unwrap();
        assert_eq!(
            invokable.ident.cpp_ident.to_string(),
            "invokableMutableCppObj"
        );
        assert_eq!(
            invokable.ident.rust_ident.to_string(),
            "invokable_mutable_cpp_obj"
        );
        assert_eq!(invokable.parameters.len(), 1);
        assert!(invokable.return_type.is_none());
        assert!(invokable.mutable);
        let parameter = &invokable.parameters[0];
        assert_eq!(parameter.ident.to_string(), "cpp");
        assert!(std::matches!(
            &parameter.type_ident.qt_type,
            QtTypes::CppObj { .. }
        ));
        assert!(parameter.type_ident.is_ref);
        assert!(parameter.type_ident.is_mut);

        // Check Parameters invokable
        let invokable = invokables.next().unwrap();
        assert_eq!(invokable.ident.cpp_ident.to_string(), "invokableParameters");
        assert_eq!(
            invokable.ident.rust_ident.to_string(),
            "invokable_parameters"
        );
        assert_eq!(invokable.parameters.len(), 3);
        assert!(invokable.return_type.is_none());
        assert!(!invokable.mutable);
        let parameter = &invokable.parameters[0];
        assert_eq!(parameter.ident.to_string(), "opaque");
        assert!(parameter.type_ident.is_ref);
        assert!(!parameter.type_ident.is_mut);
        let parameter = &invokable.parameters[1];
        assert_eq!(parameter.ident.to_string(), "trivial");
        assert!(parameter.type_ident.is_ref);
        assert!(!parameter.type_ident.is_mut);
        let parameter = &invokable.parameters[2];
        assert_eq!(parameter.ident.to_string(), "primitive");
        assert!(!parameter.type_ident.is_ref);
        assert!(!parameter.type_ident.is_mut);

        // Check Parameters CppObj invokable
        let invokable = invokables.next().unwrap();
        assert_eq!(
            invokable.ident.cpp_ident.to_string(),
            "invokableParametersCppObj"
        );
        assert_eq!(
            invokable.ident.rust_ident.to_string(),
            "invokable_parameters_cpp_obj"
        );
        assert_eq!(invokable.parameters.len(), 2);
        assert!(invokable.return_type.is_none());
        assert!(!invokable.mutable);
        let parameter = &invokable.parameters[0];
        assert_eq!(parameter.ident.to_string(), "primitive");
        assert!(!parameter.type_ident.is_ref);
        assert!(!parameter.type_ident.is_mut);
        let parameter = &invokable.parameters[1];
        assert_eq!(parameter.ident.to_string(), "cpp");
        assert!(std::matches!(
            &parameter.type_ident.qt_type,
            QtTypes::CppObj { .. }
        ));
        assert!(parameter.type_ident.is_ref);
        assert!(parameter.type_ident.is_mut);

        // Check return opaque invokable
        let invokable = invokables.next().unwrap();
        assert_eq!(
            invokable.ident.cpp_ident.to_string(),
            "invokableReturnOpaque"
        );
        assert_eq!(
            invokable.ident.rust_ident.to_string(),
            "invokable_return_opaque"
        );
        assert_eq!(invokable.parameters.len(), 0);
        assert!(invokable.return_type.is_some());
        assert!(invokable.mutable);

        // Check return primitive invokable
        let invokable = invokables.next().unwrap();
        assert_eq!(
            invokable.ident.cpp_ident.to_string(),
            "invokableReturnPrimitive"
        );
        assert_eq!(
            invokable.ident.rust_ident.to_string(),
            "invokable_return_primitive"
        );
        assert_eq!(invokable.parameters.len(), 0);
        assert!(invokable.return_type.is_some());
        assert!(invokable.mutable);

        // Check return static invokable
        let invokable = invokables.next().unwrap();
        assert_eq!(
            invokable.ident.cpp_ident.to_string(),
            "invokableReturnStatic"
        );
        assert_eq!(
            invokable.ident.rust_ident.to_string(),
            "invokable_return_static"
        );
        assert_eq!(invokable.parameters.len(), 0);
        assert!(invokable.return_type.is_some());
        assert!(invokable.mutable);

        // Check that the normal method was also detected
        assert_eq!(qobject.methods.len(), 4);
        assert_eq!(qobject.original_passthrough_decls.len(), 1);
    }

    #[test]
    fn parsing_naming() {
        let source = include_str!("../test_inputs/naming.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        // Check that it got the properties and that the idents are correct
        assert_eq!(qobject.properties.len(), 1);

        // Check first property
        let prop_first = &qobject.properties[0];
        assert_eq!(prop_first.ident.cpp_ident.to_string(), "propertyName");
        assert_eq!(prop_first.ident.rust_ident.to_string(), "property_name");
        assert!(!prop_first.type_ident.is_ref);

        assert!(prop_first.getter.is_some());
        let getter = prop_first.getter.as_ref().unwrap();
        assert_eq!(getter.cpp_ident.to_string(), "getPropertyName");
        assert_eq!(getter.rust_ident.to_string(), "property_name");

        assert!(prop_first.setter.is_some());
        let setter = prop_first.setter.as_ref().unwrap();
        assert_eq!(setter.cpp_ident.to_string(), "setPropertyName");
        assert_eq!(setter.rust_ident.to_string(), "set_property_name");

        assert!(prop_first.notify.is_some());
        let notify = prop_first.notify.as_ref().unwrap();
        assert_eq!(notify.cpp_ident.to_string(), "propertyNameChanged");
        // TODO: does rust need a notify ident?
        assert_eq!(notify.rust_ident.to_string(), "property_name");

        // Check that it got the invokables
        assert_eq!(qobject.invokables.len(), 1);

        // Check invokable ident
        let invokable = &qobject.invokables[0];
        assert_eq!(invokable.ident.cpp_ident.to_string(), "invokableName");
        assert_eq!(invokable.ident.rust_ident.to_string(), "invokable_name");

        // Check invokable parameters ident and type ident
        assert_eq!(invokable.parameters.len(), 0);
    }

    #[test]
    fn parses_passthrough() {
        let source = include_str!("../test_inputs/passthrough.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        // Check that it got the names right
        assert_eq!(qobject.ident.to_string(), "MyObject");
        assert_eq!(qobject.original_mod.ident.to_string(), "ffi");
        assert_eq!(qobject.original_rust_struct.ident.to_string(), "MyObject");

        // Check that it got the inovkables and properties
        assert_eq!(qobject.invokables.len(), 0);
        assert_eq!(qobject.properties.len(), 1);
        assert_eq!(qobject.methods.len(), 0);

        // Check that there is a use, enum and fn declaration
        assert_eq!(qobject.original_passthrough_decls.len(), 4);

        // Check that we have a CXX passthrough item
        assert_eq!(qobject.cxx_items.len(), 18);
    }

    #[test]
    fn parses_properties() {
        let source = include_str!("../test_inputs/properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        // Check that it got the properties and that the idents are correct
        assert_eq!(qobject.properties.len(), 2);
        assert_eq!(qobject.original_data_struct.ident.to_string(), "Data");

        // Check first property
        let prop_first = &qobject.properties[0];
        assert_eq!(prop_first.ident.cpp_ident.to_string(), "primitive");
        assert_eq!(prop_first.ident.rust_ident.to_string(), "primitive");
        assert!(!prop_first.type_ident.is_ref);

        assert!(prop_first.getter.is_some());
        let getter = prop_first.getter.as_ref().unwrap();
        assert_eq!(getter.cpp_ident.to_string(), "getPrimitive");
        assert_eq!(getter.rust_ident.to_string(), "primitive");

        assert!(prop_first.setter.is_some());
        let setter = prop_first.setter.as_ref().unwrap();
        assert_eq!(setter.cpp_ident.to_string(), "setPrimitive");
        assert_eq!(setter.rust_ident.to_string(), "set_primitive");

        assert!(prop_first.notify.is_some());
        let notify = prop_first.notify.as_ref().unwrap();
        assert_eq!(notify.cpp_ident.to_string(), "primitiveChanged");
        // TODO: does rust need a notify ident?
        assert_eq!(notify.rust_ident.to_string(), "primitive");

        // Check second property
        let prop_second = &qobject.properties[1];
        assert_eq!(prop_second.ident.cpp_ident.to_string(), "opaque");
        assert_eq!(prop_second.ident.rust_ident.to_string(), "opaque");
        assert!(!prop_second.type_ident.is_ref);

        assert!(prop_second.getter.is_some());
        let getter = prop_second.getter.as_ref().unwrap();
        assert_eq!(getter.cpp_ident.to_string(), "getOpaque");
        assert_eq!(getter.rust_ident.to_string(), "opaque");

        assert!(prop_second.setter.is_some());
        let setter = prop_second.setter.as_ref().unwrap();
        assert_eq!(setter.cpp_ident.to_string(), "setOpaque");
        assert_eq!(setter.rust_ident.to_string(), "set_opaque");

        assert!(prop_second.notify.is_some());
        let notify = prop_second.notify.as_ref().unwrap();
        assert_eq!(notify.cpp_ident.to_string(), "opaqueChanged");
        // TODO: does rust need a notify ident?
        assert_eq!(notify.rust_ident.to_string(), "opaque");
    }

    #[test]
    fn parses_signals() {
        let source = include_str!("../test_inputs/signals.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        assert_eq!(qobject.properties.len(), 0);
        assert_eq!(qobject.invokables.len(), 1);
        assert_eq!(qobject.signals.len(), 2);

        assert_eq!(
            qobject.signals[0].emit_ident.cpp_ident.to_string(),
            "emitReady"
        );
        assert_eq!(
            qobject.signals[0].emit_ident.rust_ident.to_string(),
            "emit_ready"
        );
        assert_eq!(qobject.signals[0].enum_ident.to_string(), "Ready");
        assert_eq!(qobject.signals[0].parameters.len(), 0);
        assert_eq!(
            qobject.signals[0].signal_ident.cpp_ident.to_string(),
            "ready"
        );
        assert_eq!(
            qobject.signals[0].signal_ident.rust_ident.to_string(),
            "ready"
        );

        assert_eq!(
            qobject.signals[1].emit_ident.cpp_ident.to_string(),
            "emitDataChanged"
        );
        assert_eq!(
            qobject.signals[1].emit_ident.rust_ident.to_string(),
            "emit_data_changed"
        );
        assert_eq!(qobject.signals[1].enum_ident.to_string(), "DataChanged");
        assert_eq!(qobject.signals[1].parameters.len(), 3);
        assert_eq!(qobject.signals[1].parameters[0].ident.to_string(), "first");
        assert_eq!(qobject.signals[1].parameters[1].ident.to_string(), "second");
        assert_eq!(qobject.signals[1].parameters[2].ident.to_string(), "third");
        assert_eq!(
            qobject.signals[1].signal_ident.cpp_ident.to_string(),
            "dataChanged"
        );
        assert_eq!(
            qobject.signals[1].signal_ident.rust_ident.to_string(),
            "data_changed"
        );
    }

    #[test]
    fn parses_types_primitive_property() {
        let source = include_str!("../test_inputs/types_primitive_property.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let qobject = extract_qobject(&module).unwrap();

        // Check that it got the inovkables and properties
        assert_eq!(qobject.invokables.len(), 0);
        assert_eq!(qobject.properties.len(), 9);

        assert_eq!(
            qobject.properties[0].ident.rust_ident.to_string(),
            "boolean"
        );
        assert_eq!(qobject.properties[0].type_ident.qt_type, QtTypes::Bool);

        assert_eq!(
            qobject.properties[1].ident.rust_ident.to_string(),
            "float_32"
        );
        assert_eq!(qobject.properties[1].type_ident.qt_type, QtTypes::F32);

        assert_eq!(
            qobject.properties[2].ident.rust_ident.to_string(),
            "float_64"
        );
        assert_eq!(qobject.properties[2].type_ident.qt_type, QtTypes::F64);

        assert_eq!(qobject.properties[3].ident.rust_ident.to_string(), "int_8");
        assert_eq!(qobject.properties[3].type_ident.qt_type, QtTypes::I8);

        assert_eq!(qobject.properties[4].ident.rust_ident.to_string(), "int_16");
        assert_eq!(qobject.properties[4].type_ident.qt_type, QtTypes::I16);

        assert_eq!(qobject.properties[5].ident.rust_ident.to_string(), "int_32");
        assert_eq!(qobject.properties[5].type_ident.qt_type, QtTypes::I32);

        assert_eq!(qobject.properties[6].ident.rust_ident.to_string(), "uint_8");
        assert_eq!(qobject.properties[6].type_ident.qt_type, QtTypes::U8);

        assert_eq!(
            qobject.properties[7].ident.rust_ident.to_string(),
            "uint_16"
        );
        assert_eq!(qobject.properties[7].type_ident.qt_type, QtTypes::U16);

        assert_eq!(
            qobject.properties[8].ident.rust_ident.to_string(),
            "uint_32"
        );
        assert_eq!(qobject.properties[8].type_ident.qt_type, QtTypes::U32);
    }
}
