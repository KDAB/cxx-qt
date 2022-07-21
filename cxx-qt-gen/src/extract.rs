// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::utils::type_to_namespace;
use convert_case::{Case, Casing};
use derivative::*;
use proc_macro2::{Span, TokenStream, TokenTree};
use std::result::Result;
use syn::{spanned::Spanned, *};

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
        /// Whether this CppObj is from the current module or another module
        external: bool,
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
    String,
    Str,
    QTime,
    QUrl,
    QVariant,
    U8,
    U16,
    U32,
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
            Self::QColor => true,
            Self::QDateTime => true,
            Self::String | Self::Str => true,
            Self::QUrl => true,
            Self::QVariant => true,
            _others => false,
        }
    }
}

/// Describes a type
#[derive(Debug)]
pub(crate) struct ParameterType {
    /// The type of the parameter
    pub(crate) idents: Vec<Ident>,
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
    pub(crate) emit_ident: CppRustIdent,
    pub(crate) enum_ident: Ident,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) signal_ident: CppRustIdent,
}

/// Describes all the properties of a QObject class
#[derive(Debug)]
pub struct QObject {
    /// The ident of the C++ class that represents the QObject
    pub ident: Ident,
    /// All the methods that can also be invoked from QML
    pub(crate) invokables: Vec<Invokable>,
    /// All the methods that cannot be invoked from QML
    pub(crate) normal_methods: Vec<ImplItemMethod>,
    /// All the properties that can be used from QML
    pub(crate) properties: Vec<Property>,
    /// All the signals that can be emitted/connected from QML
    pub(crate) signals: Vec<Signal>,
    /// The namespace to use for C++
    pub(crate) namespace: Vec<String>,
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
    /// The original Rust trait impls for the struct
    pub(crate) original_trait_impls: Vec<ItemImpl>,
    /// The original Rust declarations from the mod that will be directly passed through
    pub(crate) original_passthrough_decls: Vec<Item>,
    /// The Rust impl that has optionally been provided to handle updates
    pub(crate) handle_updates_impl: Option<ItemImpl>,
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
    cpp_namespace_prefix: &[&str],
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
                external: false,
                cpp_type_idents: vec![qt_ident.clone()],
                cpp_type_idents_string: qt_ident.to_string(),
                rust_type_idents: vec![qt_ident.clone()],
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
            "str" => Ok(QtTypes::Str),
            "String" => Ok(QtTypes::String),
            "QTime" => Ok(QtTypes::QTime),
            "QUrl" => Ok(QtTypes::QUrl),
            "QVariant" => Ok(QtTypes::QVariant),
            "u8" => Ok(QtTypes::U8),
            "u16" => Ok(QtTypes::U16),
            "u32" => Ok(QtTypes::U32),
            _other => Ok(QtTypes::Unknown),
        }
    // If the first ident is crate, the last is CppObj, and we have more than two parts
    // then we are an external CppObj
    } else if idents.len() > 2
        && idents.first().unwrap().to_string().as_str() == "crate"
        && idents.last().unwrap().to_string().as_str() == "CppObj"
    {
        // If the ident starts with cxx_qt_ for now this means we should remove it
        // as we are trying to access the CppObj
        //
        // TODO: this hack will be removed in the future when we move to UniquePtr
        let cpp_rewritten_idents = idents
            .iter()
            .map(|ident| {
                if ident.to_string().starts_with("cxx_qt_") {
                    quote::format_ident!("{}", ident.to_string()[7..])
                } else {
                    ident.to_owned()
                }
            })
            .collect::<Vec<Ident>>();
        let cpp_type_idents = cpp_namespace_prefix
            .to_vec()
            .iter()
            .map(|s| quote::format_ident!("{}", s))
            // TODO: once we generate sub folders for nested modules, this will need to use all
            // type idents other than first and last. as the namespace will then reflect sub dirs
            // https://github.com/KDAB/cxx-qt/issues/19
            .chain(
                cpp_rewritten_idents
                    .iter()
                    .skip(cpp_rewritten_idents.len() - 2)
                    .cloned(),
            )
            .collect::<Vec<Ident>>();
        Ok(QtTypes::CppObj {
            external: true,
            cpp_type_idents_string: cpp_type_idents
                .iter()
                .map(|ident| ident.to_string())
                .collect::<Vec<String>>()
                .join("::"),
            cpp_type_idents,
            rust_type_idents: idents.to_vec(),
            // TODO: later can we use the fully qualified path
            // eg crate::my_module::CppObj to Crate_MyModule_CppObj?
            combined_name: quote::format_ident!(
                "{}",
                cpp_rewritten_idents[cpp_rewritten_idents.len() - 2]
                    .to_string()
                    .to_case(Case::Pascal)
            ),
        })
    // This is an unknown type that did not start with crate and has multiple parts
    } else {
        // We can assume that idents has an entry at index zero, because it is not empty
        Err(ExtractTypeIdentError::UnknownAndNotCrate(idents[0].span()))
    }
}

/// Converts a given path to a vector of idents
fn path_to_idents(path: &syn::Path) -> Result<Vec<Ident>, ExtractTypeIdentError> {
    path.segments
        .iter()
        .map(|segment| {
            // We do not support PathArguments for types in properties or arguments
            //
            // eg we do not support AngleBracketed - the <'a, T> in std::slice::iter<'a, T>
            // eg we do not support Parenthesized - the (A, B) -> C in Fn(A, B) -> C
            if segment.arguments == PathArguments::None {
                Ok(segment.ident.to_owned())
            } else {
                Err(ExtractTypeIdentError::InvalidArguments(segment.span()))
            }
        })
        .collect::<Result<Vec<Ident>, ExtractTypeIdentError>>()
}

/// Extract the type ident from a given syn::Type
fn extract_type_ident(
    ty: &syn::Type,
    cpp_namespace_prefix: &[&str],
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
    let qt_type = extract_qt_type(&idents, ty, cpp_namespace_prefix, qt_ident)?;

    // Check if this Qt type is allowed to be a ref mut
    if is_mut && is_ref && !qt_type.ref_mut_is_valid() {
        return Err(ExtractTypeIdentError::InvalidType(ty.span()));
    }

    // Create and return a ParameterType
    Ok(ParameterType {
        // Read each of the path segment to turn a &syn::TypePath of std::slice::Iter
        // into an owned Vec<Ident>
        idents,
        is_mut,
        is_ref,
        qt_type,
    })
}

/// The result of extracting invokables from an impl block.
/// Only intended for internal use for now.
struct ExtractedInvokables {
    /// Impl methods that will also be exposed as invokables to Qt
    invokables: Vec<Invokable>,
    /// Impl methods that will only be visible on the Rust side
    normal_methods: Vec<ImplItemMethod>,
}

/// Extracts all the member functions from a module and generates invokables from them
fn extract_invokables(
    items: &[ImplItem],
    cpp_namespace_prefix: &[&str],
    qt_ident: &Ident,
) -> Result<ExtractedInvokables, TokenStream> {
    let mut invokables = Vec::new();
    let mut normal_methods = Vec::new();

    // TODO: we need to set up an exclude list of invokable names and give
    // the user an error if they use one of those names.
    // This is to avoid name collisions with QObject standard functions.

    // Process each impl item and turn into an Invokable or error
    for item in items {
        // Check if this item is a method
        //
        // TODO: later should we pass through unknown items
        // or should they have an attribute to ignore
        let mut method = if let ImplItem::Method(m) = item {
            m.clone()
        } else {
            return Err(Error::new(item.span(), "Only methods are supported.").to_compile_error());
        };

        let filtered_attrs: Vec<syn::Attribute> = method
            .attrs
            .iter()
            .filter(|a| {
                let segments = &a.path.segments;

                if segments.len() != 1 {
                    return true;
                }

                segments[0].ident != "invokable"
            })
            .cloned()
            .collect();

        // Skip non-invokable members
        if filtered_attrs.len() == method.attrs.len() {
            normal_methods.push(method);
            continue;
        }

        // Remove the #[invokable tag for the output]
        method.attrs = filtered_attrs;

        // Extract the ident, parameters, return type of the method
        let invokable = extract_invokable(&method, cpp_namespace_prefix, qt_ident)?;
        invokables.push(invokable);
    }

    Ok(ExtractedInvokables {
        invokables,
        normal_methods,
    })
}

/// Extract the parameters for a given ImplItemMethod
pub(crate) fn extract_method_params(
    method: &ImplItemMethod,
    cpp_namespace_prefix: &[&str],
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
                match extract_type_ident(ty, cpp_namespace_prefix, qt_ident) {
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

fn extract_invokable(
    method: &ImplItemMethod,
    cpp_namespace_prefix: &[&str],
    qt_ident: &Ident,
) -> Result<Invokable, TokenStream> {
    let method_ident = &method.sig.ident;
    let output = &method.sig.output;

    let mutable = is_method_mutable(method);
    let parameters = extract_method_params(method, cpp_namespace_prefix, qt_ident)?;

    let return_type = if let ReturnType::Type(_, ty) = output {
        // This output has a return type, so extract the type
        match extract_type_ident(ty, cpp_namespace_prefix, qt_ident) {
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
fn extract_properties(
    s: &ItemStruct,
    cpp_namespace_prefix: &[&str],
    qt_ident: &Ident,
) -> Result<Vec<Property>, TokenStream> {
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

                match extract_type_ident(ty, cpp_namespace_prefix, qt_ident) {
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
fn extract_signals(
    e: &syn::ItemEnum,
    cpp_namespace_prefix: &[&str],
    qt_ident: &Ident,
) -> Result<Vec<Signal>, TokenStream> {
    let mut signals = Vec::new();

    for variant in &e.variants {
        let ident_str = variant.ident.to_string();
        let parameters = if let Fields::Named(FieldsNamed { named, .. }) = &variant.fields {
            let mut parameters = vec![];

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

                    match extract_type_ident(ty, cpp_namespace_prefix, qt_ident) {
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

                    parameters.push(Parameter {
                        ident: ident.clone(),
                        type_ident,
                    });
                }
            }

            parameters
        } else {
            vec![]
        };

        signals.push(Signal {
            emit_ident: CppRustIdent {
                cpp_ident: quote::format_ident!("emit{}", ident_str.to_case(Case::Pascal)),
                rust_ident: quote::format_ident!("emit_{}", ident_str.to_case(Case::Snake)),
            },
            enum_ident: variant.ident.clone(),
            parameters,
            signal_ident: CppRustIdent {
                cpp_ident: quote::format_ident!("{}", ident_str.to_case(Case::Camel)),
                rust_ident: quote::format_ident!("{}", ident_str.to_case(Case::Snake)),
            },
        });
    }

    Ok(signals)
}

/// Parses a module in order to extract a QObject description from it
pub fn extract_qobject(
    module: ItemMod,
    cpp_namespace_prefix: &[&str],
) -> Result<QObject, TokenStream> {
    // Find the items from the module
    let original_mod = module.to_owned();
    let items = &mut module
        .content
        .expect("Incorrect module format encountered.")
        .1;

    // Prepare variables to store struct, invokables, and other data
    //
    // The original Data Item::Struct if one is found
    let mut original_data_struct = None;
    // The original RustObj Item::Struct if one is found
    let mut original_rust_struct = None;
    // The name of the Qt object we are creating
    let qt_ident = quote::format_ident!("{}", original_mod.ident.to_string().to_case(Case::Pascal));

    // A list of the invokables for the struct
    let mut object_invokables = vec![];
    // A list of the normal methods (i.e. not invokables) for the struct
    let mut object_normal_methods = vec![];
    // A list of original trait impls for the struct (eg `impl Default for Struct`)
    let mut original_trait_impls = vec![];
    // A list of insignificant declarations for the mod that will be directly passed through (eg `use crate::thing`)
    let mut original_passthrough_decls = vec![];
    // The original signal enum if one is found
    let mut original_signal_enum = None;
    // A list of items we will pass through to the CXX bridge
    //
    // TODO: for now this just includes ItemForeignMod but later this will switch to all non CXX-Qt items
    let mut cxx_items = vec![];

    // Determines if (and how) this object can respond to update requests
    let mut handle_updates_impl = None;

    // Process each of the items in the mod
    for item in items.drain(..) {
        match item {
            // We are an Enum
            Item::Enum(ref item_enum) => {
                match item_enum.ident.to_string().as_str() {
                    // We are a Signal definition
                    "Signal" => {
                        // Check that we are the first Signal enum
                        if original_signal_enum.is_none() {
                            original_signal_enum = Some(item_enum.clone());
                        } else {
                            return Err(Error::new(
                                item_enum.span(),
                                "Only one Signal enum is supported per mod.",
                            )
                            .to_compile_error());
                        }
                    }
                    // Passthrough unknown enums
                    _others => original_passthrough_decls.push(item.to_owned()),
                }
            }
            // We are a struct
            Item::Struct(s) => {
                match s.ident.to_string().as_str() {
                    // This is the Data struct
                    "Data" => {
                        // Check that we are the first Data struct
                        if original_data_struct.is_none() {
                            original_data_struct = Some(s);
                        } else {
                            return Err(Error::new(
                                s.span(),
                                "Only one Data struct is supported per mod.",
                            )
                            .to_compile_error());
                        }
                    }
                    "RustObj" => {
                        // Check that we are the first other struct
                        if original_rust_struct.is_none() {
                            // Move the original struct
                            original_rust_struct = Some(s);
                        } else {
                            return Err(Error::new(
                                s.span(),
                                "Only one RustObj struct is supported per mod.",
                            )
                            .to_compile_error());
                        }
                    }
                    _others => {
                        return Err(
                            Error::new(s.span(), "Unknown struct for QObject.").to_compile_error()
                        );
                    }
                }
            }
            // We are an impl
            Item::Impl(mut original_impl) => {
                // Extract the path from the type (this leads to the struct name)
                if let Type::Path(TypePath { path, .. }) = &mut *original_impl.self_ty {
                    // Check that the path contains segments
                    if path.segments.len() != 1 {
                        return Err(Error::new(
                            original_impl.span(),
                            "Invalid path on impl block.",
                        )
                        .to_compile_error());
                    }

                    // Read the name of the struct that the impl is for
                    //
                    // We can assume that segments[0] works as we have checked length to be 1
                    match path.segments[0].ident.to_string().as_str() {
                        "Data" => {
                            // Can have a trait, eg impl Default for Data
                            if original_impl.trait_.is_some() {
                                // Push the original trait impl
                                //
                                // TODO: have original_data_trait_impls so that we can keep
                                // impl Data close to struct Data
                                original_trait_impls.push(original_impl.to_owned());
                            } else {
                                // Cannot have methods
                                //
                                // TODO: later should we pass through any Data impl methods?
                                return Err(Error::new(
                                    original_impl.span(),
                                    "Data struct cannot have impl methods.",
                                )
                                .to_compile_error());
                            }
                        }
                        "RustObj" => {
                            // Ensure that the struct block has already happened
                            if original_rust_struct.is_none() {
                                return Err(Error::new(
                                    original_impl.span(),
                                    "Impl can only be declared after a RustObj struct.",
                                )
                                .to_compile_error());
                            }

                            // Needs to match the original struct name, later this check won't be needed
                            //
                            // We can assume that original_rust_struct exists as we checked it above
                            if path.segments[0].ident
                                != original_rust_struct.as_ref().unwrap().ident
                            {
                                return Err(Error::new(
                                    path.span(),
                                    "The impl block needs to match the RustObj struct.",
                                )
                                .to_compile_error());
                            }

                            // Can have custom traits, these are on the RustObj
                            if let Some(trait_) = &original_impl.trait_ {
                                // We should always have at least one segments as something is unlikely
                                // to have been parsed as a "trait" in the first place otherwise
                                match trait_.1.segments[0].ident.to_string().as_str() {
                                    "UpdateRequestHandler" => {
                                        handle_updates_impl = Some(original_impl.to_owned())
                                    }
                                    _others => original_trait_impls.push(original_impl.to_owned()),
                                }
                            } else {
                                let mut extracted = extract_invokables(
                                    &original_impl.items,
                                    cpp_namespace_prefix,
                                    &qt_ident,
                                )?;

                                object_invokables.append(&mut extracted.invokables);
                                object_normal_methods.append(&mut extracted.normal_methods);
                            }
                        }
                        _others => {
                            return Err(Error::new(
                                path.span(),
                                "Unknown struct for impl block for QObject.",
                            )
                            .to_compile_error());
                        }
                    }
                } else {
                    return Err(Error::new(
                        original_impl.span(),
                        "Expected a TypePath impl to parse.",
                    )
                    .to_compile_error());
                }
            }
            // Items we will pass through to the CXX bridge
            //
            // TODO: for now this just includes ItemForeignMod but later this will switch to all non CXX-Qt items
            //
            // Note we also need to search in Verbatim for "unsafe extern" blocks
            Item::ForeignMod(_) => cxx_items.push(item),
            Item::Verbatim(ref tokens) => {
                let is_group = |value: &Option<TokenTree>| -> bool {
                    if let Some(TokenTree::Group(_)) = value {
                        return true;
                    }

                    false
                };
                let is_ident = |value: &Option<TokenTree>, sym: &str| -> bool {
                    if let Some(TokenTree::Ident(ident)) = value {
                        return *ident == quote::format_ident!("{}", sym);
                    }

                    false
                };
                let is_punct = |value: &Option<TokenTree>, char: char| -> bool {
                    if let Some(TokenTree::Punct(punct)) = value {
                        return punct.as_char() == char;
                    }

                    false
                };
                let tokens = tokens.clone();
                let mut iter = tokens.into_iter();

                // Skip over any attributes
                //
                // which appears as a punct # and then a group []
                let mut first = iter.next();
                while is_punct(&first, '#') {
                    first = iter.next();
                    if is_group(&first) {
                        first = iter.next();
                    }
                }

                if is_ident(&first, "unsafe") && is_ident(&iter.next(), "extern") {
                    cxx_items.push(item);
                } else {
                    original_passthrough_decls.push(item);
                }
            }
            // We are an insignificant item that will be directly passed through
            other => {
                original_passthrough_decls.push(other);
            }
        }
    }

    // Read properties from the Data struct
    let object_properties = if let Some(ref original_struct) = original_data_struct {
        extract_properties(original_struct, cpp_namespace_prefix, &qt_ident)?
    } else {
        vec![]
    };

    // Read signals  from the Signal enum
    let object_signals = if let Some(ref original_enum) = original_signal_enum {
        extract_signals(original_enum, cpp_namespace_prefix, &qt_ident)?
    } else {
        vec![]
    };

    // Build the namespace for this QObject
    //
    // We build a fake valid type here, crate::module::Object
    // so that we can use our namespace helper to retieve cxx_qt::module etc.
    let namespace = type_to_namespace(
        cpp_namespace_prefix,
        &[
            quote::format_ident!("crate"),
            original_mod.ident.clone(),
            qt_ident.clone(),
        ],
    )
    .map_err(|msg| {
        Error::new(
            original_mod.ident.span(),
            format!("Could not generate namespace with module name: {}", msg),
        )
        .to_compile_error()
    })?;

    Ok(QObject {
        ident: qt_ident,
        invokables: object_invokables,
        normal_methods: object_normal_methods,
        properties: object_properties,
        signals: object_signals,
        namespace,
        cxx_items,
        original_mod,
        original_data_struct: original_data_struct
            .unwrap_or_else(|| syn::parse_str("pub struct Data;").unwrap()),
        original_signal_enum,
        original_rust_struct: original_rust_struct
            .unwrap_or_else(|| syn::parse_str("pub struct RustObj;").unwrap()),
        original_trait_impls,
        original_passthrough_decls,
        handle_updates_impl,
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
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        // Check that it got the invokables and properties
        assert_eq!(qobject.invokables.len(), 0);
        assert_eq!(qobject.properties.len(), 1);

        assert_eq!(qobject.original_trait_impls.len(), 2);

        // Check that impl Default was found for Data
        let trait_impl = &qobject.original_trait_impls[0];
        if let Type::Path(TypePath { path, .. }) = &*trait_impl.self_ty {
            assert_eq!(path.segments.len(), 1);
            assert_eq!(path.segments[0].ident.to_string(), "Data");
        } else {
            panic!("Trait impl was not a TypePath");
        }

        // Check that impl Default was found for RustObj
        let trait_impl = &qobject.original_trait_impls[1];
        if let Type::Path(TypePath { path, .. }) = &*trait_impl.self_ty {
            assert_eq!(path.segments.len(), 1);
            assert_eq!(path.segments[0].ident.to_string(), "RustObj");
        } else {
            panic!("Trait impl was not a TypePath");
        }
    }

    #[test]
    fn parses_invokables() {
        let source = include_str!("../test_inputs/invokables.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        // Check that it got the names right
        assert_eq!(qobject.ident.to_string(), "MyObject");
        assert_eq!(qobject.original_mod.ident.to_string(), "my_object");
        assert_eq!(qobject.original_rust_struct.ident.to_string(), "RustObj");

        // Check that it got the invokables
        assert_eq!(qobject.invokables.len(), 10);

        // Check empty invokable ident
        let invokable = &qobject.invokables[0];
        assert_eq!(invokable.ident.cpp_ident.to_string(), "invokable");
        assert_eq!(invokable.ident.rust_ident.to_string(), "invokable");
        assert_eq!(invokable.parameters.len(), 0);
        assert!(invokable.return_type.is_none());
        assert!(!invokable.mutable);

        // Check CppObj invokable
        let invokable = &qobject.invokables[1];
        assert_eq!(invokable.ident.cpp_ident.to_string(), "invokableCppObj");
        assert_eq!(invokable.ident.rust_ident.to_string(), "invokable_cpp_obj");
        assert_eq!(invokable.parameters.len(), 1);
        assert!(invokable.return_type.is_none());
        assert!(!invokable.mutable);
        let parameter = &invokable.parameters[0];
        assert_eq!(parameter.ident.to_string(), "cpp");
        assert_eq!(parameter.type_ident.idents.len(), 1);
        assert_eq!(parameter.type_ident.idents[0].to_string(), "CppObj");
        if let QtTypes::CppObj { external, .. } = &parameter.type_ident.qt_type {
            assert_eq!(external, &false);
        } else {
            panic!();
        }
        assert!(parameter.type_ident.is_ref);
        assert!(parameter.type_ident.is_mut);

        // Check the mutable invokable
        let invokable = &qobject.invokables[2];
        assert_eq!(invokable.ident.cpp_ident.to_string(), "invokableMutable");
        assert_eq!(invokable.ident.rust_ident.to_string(), "invokable_mutable");
        assert_eq!(invokable.parameters.len(), 0);
        assert!(invokable.return_type.is_none());
        assert!(invokable.mutable);

        // Check the mutable CppObj invokable
        let invokable = &qobject.invokables[3];
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
        assert_eq!(parameter.type_ident.idents.len(), 1);
        assert_eq!(parameter.type_ident.idents[0].to_string(), "CppObj");
        if let QtTypes::CppObj { external, .. } = &parameter.type_ident.qt_type {
            assert_eq!(external, &false);
        } else {
            panic!();
        }
        assert!(parameter.type_ident.is_ref);
        assert!(parameter.type_ident.is_mut);

        // Check nested parameter invokable
        let invokable = &qobject.invokables[4];
        assert_eq!(
            invokable.ident.cpp_ident.to_string(),
            "invokableNestedParameter"
        );
        assert_eq!(
            invokable.ident.rust_ident.to_string(),
            "invokable_nested_parameter"
        );
        assert_eq!(invokable.parameters.len(), 1);
        assert!(invokable.return_type.is_none());
        assert!(!invokable.mutable);
        let parameter = &invokable.parameters[0];
        assert_eq!(parameter.ident.to_string(), "nested");
        assert_eq!(parameter.type_ident.idents.len(), 3);
        assert_eq!(parameter.type_ident.idents[0].to_string(), "crate");
        assert_eq!(
            parameter.type_ident.idents[1].to_string(),
            "cxx_qt_nested_object"
        );
        assert_eq!(parameter.type_ident.idents[2].to_string(), "CppObj");
        if let QtTypes::CppObj { external, .. } = &parameter.type_ident.qt_type {
            assert_eq!(external, &true);
        } else {
            panic!();
        }
        assert!(parameter.type_ident.is_ref);
        assert!(parameter.type_ident.is_mut);
        assert!(parameter.type_ident.qt_type.is_opaque());

        // Check Parameters invokable
        let invokable = &qobject.invokables[5];
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
        assert_eq!(parameter.type_ident.idents.len(), 1);
        assert_eq!(parameter.type_ident.idents[0].to_string(), "QColor");
        assert!(parameter.type_ident.is_ref);
        assert!(!parameter.type_ident.is_mut);
        let parameter = &invokable.parameters[1];
        assert_eq!(parameter.ident.to_string(), "trivial");
        assert_eq!(parameter.type_ident.idents.len(), 1);
        assert_eq!(parameter.type_ident.idents[0].to_string(), "QPoint");
        assert!(parameter.type_ident.is_ref);
        assert!(!parameter.type_ident.is_mut);
        let parameter = &invokable.parameters[2];
        assert_eq!(parameter.ident.to_string(), "primitive");
        assert_eq!(parameter.type_ident.idents.len(), 1);
        assert_eq!(parameter.type_ident.idents[0].to_string(), "i32");
        assert!(!parameter.type_ident.is_ref);
        assert!(!parameter.type_ident.is_mut);

        // Check Parameters CppObj invokable
        let invokable = &qobject.invokables[6];
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
        assert_eq!(parameter.type_ident.idents.len(), 1);
        assert_eq!(parameter.type_ident.idents[0].to_string(), "i32");
        assert!(!parameter.type_ident.is_ref);
        assert!(!parameter.type_ident.is_mut);
        let parameter = &invokable.parameters[1];
        assert_eq!(parameter.ident.to_string(), "cpp");
        assert_eq!(parameter.type_ident.idents.len(), 1);
        assert_eq!(parameter.type_ident.idents[0].to_string(), "CppObj");
        if let QtTypes::CppObj { external, .. } = &parameter.type_ident.qt_type {
            assert_eq!(external, &false);
        } else {
            panic!();
        }
        assert!(parameter.type_ident.is_ref);
        assert!(parameter.type_ident.is_mut);

        // Check return opaque invokable
        let invokable = &qobject.invokables[7];
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
        let invokable = &qobject.invokables[8];
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
        let invokable = &qobject.invokables[9];
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
        assert_eq!(qobject.normal_methods.len(), 1);
    }

    #[test]
    fn parsing_naming() {
        let source = include_str!("../test_inputs/naming.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        // Check that it got the properties and that the idents are correct
        assert_eq!(qobject.properties.len(), 1);

        // Check first property
        let prop_first = &qobject.properties[0];
        assert_eq!(prop_first.ident.cpp_ident.to_string(), "propertyName");
        assert_eq!(prop_first.ident.rust_ident.to_string(), "property_name");
        assert_eq!(prop_first.type_ident.idents.len(), 1);
        assert_eq!(prop_first.type_ident.idents[0].to_string(), "i32");
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
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        // Check that it got the names right
        assert_eq!(qobject.ident.to_string(), "MyObject");
        assert_eq!(qobject.original_mod.ident.to_string(), "my_object");
        assert_eq!(qobject.original_rust_struct.ident.to_string(), "RustObj");

        // Check that it got the inovkables and properties
        assert_eq!(qobject.invokables.len(), 0);
        assert_eq!(qobject.properties.len(), 1);
        assert_eq!(qobject.normal_methods.len(), 2);

        // Check that there is a use, enum and fn declaration
        assert_eq!(qobject.original_passthrough_decls.len(), 13);

        // Check that we have a CXX passthrough item
        assert_eq!(qobject.cxx_items.len(), 6);
    }

    #[test]
    fn parses_properties() {
        let source = include_str!("../test_inputs/properties.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

        // Check that it got the properties and that the idents are correct
        assert_eq!(qobject.properties.len(), 3);
        assert_eq!(qobject.original_data_struct.ident.to_string(), "Data");

        // Check first property
        let prop_first = &qobject.properties[0];
        assert_eq!(prop_first.ident.cpp_ident.to_string(), "primitive");
        assert_eq!(prop_first.ident.rust_ident.to_string(), "primitive");
        assert_eq!(prop_first.type_ident.idents.len(), 1);
        assert_eq!(prop_first.type_ident.idents[0].to_string(), "i32");
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
        assert_eq!(prop_second.type_ident.idents.len(), 1);
        assert_eq!(prop_second.type_ident.idents[0].to_string(), "QColor");
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

        // Check third property
        let prop_third = &qobject.properties[2];
        assert_eq!(prop_third.ident.cpp_ident.to_string(), "nested");
        assert_eq!(prop_third.ident.rust_ident.to_string(), "nested");
        assert_eq!(prop_third.type_ident.idents.len(), 3);
        assert_eq!(prop_third.type_ident.idents[0].to_string(), "crate");
        assert_eq!(
            prop_third.type_ident.idents[1].to_string(),
            "cxx_qt_nested_object"
        );
        assert_eq!(prop_third.type_ident.idents[2].to_string(), "CppObj");
        assert!(!prop_third.type_ident.is_ref);

        assert!(prop_third.getter.is_some());
        let getter = prop_third.getter.as_ref().unwrap();
        assert_eq!(getter.cpp_ident.to_string(), "getNested");
        assert_eq!(getter.rust_ident.to_string(), "nested");

        assert!(prop_third.setter.is_some());
        let setter = prop_third.setter.as_ref().unwrap();
        assert_eq!(setter.cpp_ident.to_string(), "setNested");
        assert_eq!(setter.rust_ident.to_string(), "set_nested");

        assert!(prop_third.notify.is_some());
        let notify = prop_third.notify.as_ref().unwrap();
        assert_eq!(notify.cpp_ident.to_string(), "nestedChanged");
        // TODO: does rust need a notify ident?
        assert_eq!(notify.rust_ident.to_string(), "nested");
    }

    #[test]
    fn parses_signals() {
        let source = include_str!("../test_inputs/signals.rs");
        let module: ItemMod = syn::parse_str(source).unwrap();
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

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
        assert_eq!(
            qobject.signals[1].parameters[0].type_ident.idents[0].to_string(),
            "i32"
        );
        assert_eq!(qobject.signals[1].parameters[1].ident.to_string(), "second");
        assert_eq!(
            qobject.signals[1].parameters[1].type_ident.idents[0].to_string(),
            "QVariant"
        );
        assert_eq!(qobject.signals[1].parameters[2].ident.to_string(), "third");
        assert_eq!(
            qobject.signals[1].parameters[2].type_ident.idents[0].to_string(),
            "QPoint"
        );
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
        let cpp_namespace_prefix = vec!["cxx_qt"];
        let qobject = extract_qobject(module, &cpp_namespace_prefix).unwrap();

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
