// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use syn::{
    spanned::Spanned, AngleBracketedGenericArguments, Error, GenericArgument, GenericParam,
    Generics, ItemImpl, Lifetime, Path, PathArguments, PathSegment, Result, Type,
};

#[derive(Default)]
struct ConstructorArguments {
    /// Arguments to the new function.
    /// The `new` function needs to return the inner Rust struct for the QObject.
    new: Option<Vec<Type>>,
    /// Arguments to be passed to the base class constructor.
    base: Option<Vec<Type>>,
    /// Arguments to the initialize function.
    /// The `initialize` function is run after the QObject is created.
    initialize: Option<Vec<Type>>,
}

/// A parsed cxx_qt::Constructor trait impl.
#[derive(Debug, PartialEq, Eq)]
pub struct Constructor {
    /// The arguments to the constructor defined by this trait impl.
    pub arguments: Vec<Type>,

    /// Arguments to the new function.
    /// The `new` function needs to return the inner Rust struct for the QObject.
    pub new_arguments: Vec<Type>,
    /// Arguments to be passed to the base class constructor.
    pub base_arguments: Vec<Type>,
    /// Arguments to the initialize function.
    /// The `initialize` function is run after the QObject is created.
    pub initialize_arguments: Vec<Type>,

    // The lifetime argument of the impl block.
    pub lifetime: Option<Lifetime>,

    /// The original impl that this constructor was parse from.
    // TODO: This has moved into MarkerTrait
    pub imp: ItemImpl,
}

impl Constructor {
    fn parse_argument_list(ty: Type) -> Result<Vec<Type>> {
        Ok(match ty {
            Type::Tuple(tuple) => tuple.elems.into_iter().collect(),
            _ => return Err(Error::new(ty.span(), "Expected a tuple as argument list!\nNote that a tuple of a single type needs to use a trailing comma, e.g. (i32,)"))
        })
    }

    fn parse_associated_types(generics: &[&GenericArgument]) -> Result<ConstructorArguments> {
        let mut arguments = ConstructorArguments::default();
        for generic in generics {
            if let GenericArgument::AssocType(associated) = generic {
                let argument_list = match &*associated.ident.to_string() {
                    "NewArguments" => &mut arguments.new,
                    "InitializeArguments" => &mut arguments.initialize,
                    "BaseArguments" => &mut arguments.base,
                    _ => return Err(Error::new_spanned(generic, "Unknown associated type!")),
                };
                if argument_list.is_some() {
                    return Err(Error::new_spanned(
                        generic,
                        "Duplicate associated type definition!",
                    ));
                }

                *argument_list = Some(Self::parse_argument_list(associated.ty.clone())?);
            } else {
                return Err(Error::new_spanned(
                    generic,
                    "Expected associated type as a generic argument!",
                ));
            }
        }
        Ok(arguments)
    }

    fn parse_generics(
        trait_path: &Path,
        generics: &[&GenericArgument],
    ) -> Result<(Vec<Type>, ConstructorArguments)> {
        if let Some((GenericArgument::Type(arguments_tuple), generics)) = generics.split_first() {
            let argument_types = Self::parse_argument_list(arguments_tuple.clone())?;

            let arguments = Self::parse_associated_types(generics)?;

            Ok((argument_types, arguments))
        } else {
            let span = if generics.is_empty() {
                trait_path.span()
            } else {
                generics[0].span()
            };

            Err(Error::new(
                span,
                "cxx_qt::Constructor expects a tuple as the first generic argument!",
            ))
        }
    }

    fn parse_arguments(trait_path: &Path) -> Result<(Vec<Type>, ConstructorArguments)> {
        let constructor_path: Vec<_> = trait_path.segments.iter().collect();
        if let [
            // cxx_qt::
            _cxx_qt,
            // Constructor
            PathSegment {
            arguments:
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args: generics_punct, .. }),
            ..
        }] = *constructor_path
        {
            let generics: Vec<_> = generics_punct.iter().collect();

            Self::parse_generics(trait_path, &generics)
        } else {
            Err(Error::new_spanned(
                trait_path,
                "Missing generic argument for cxx_qt::Constructor!",
            ))
        }
    }

    pub fn parse_impl_generics(generics: &Generics) -> Result<Option<Lifetime>> {
        if generics.where_clause.is_some() {
            return Err(Error::new_spanned(
                &generics.where_clause,
                "Where clauses are not allowed on cxx_qt::Constructor impls!",
            ));
        }

        let parameters: Vec<_> = generics.params.iter().collect();
        match *parameters {
            [] => Ok(None),
            [GenericParam::Lifetime(lifetime)] => Ok(Some(lifetime.lifetime.clone())),
            _ => Err(Error::new_spanned(
                generics,
                "Only a single lifetime parameter is allowed on cxx_qt::Constructor impls!",
            )),
        }
    }

    pub fn parse(imp: ItemImpl) -> Result<Self> {
        if let Some(unsafety) = imp.unsafety {
            return Err(Error::new_spanned(
                unsafety,
                "Unnecessary unsafe around constructor impl!",
            ));
        }

        let (not, trait_path, _) = &imp
            .trait_
            .as_ref()
            .ok_or_else(|| Error::new_spanned(imp.clone(), "Expected trait impl!"))?;

        if not.is_some() {
            return Err(Error::new_spanned(
                trait_path,
                "Negative impls for cxx_qt::Constructor are not allowed",
            ));
        }

        let lifetime = Self::parse_impl_generics(&imp.generics)?;

        let (argument_list, arguments) = Self::parse_arguments(trait_path)?;
        Ok(Constructor {
            arguments: argument_list,
            new_arguments: arguments.new.unwrap_or_default(),
            base_arguments: arguments.base.unwrap_or_default(),
            initialize_arguments: arguments.initialize.unwrap_or_default(),
            lifetime,
            imp,
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    // CODECOV_EXCLUDE_START
    fn assert_parse_error(item: ItemImpl, message: &str) {
        assert!(
            Constructor::parse(item).is_err(),
            // Excluded as this is just a custom error message for if asserts fail
            "Constructor shouldn't have parsed because '{message}'."
        );
    }
    // CODECOV_EXCLUDE_STOP

    #[test]
    fn parse_invalid_constructors() {
        assert_parse_error(
            parse_quote! {
                impl cxx_qt::Constructor for X {}
            },
            "missing type arguments",
        );
        assert_parse_error(
            parse_quote! {
                impl cxx_qt::Constructor<NewArguments=()> for X {}
            },
            "missing main argument list",
        );

        // Hard to tell if this actually hits the error as the rest of the project isn't compiling
        assert_parse_error(
            parse_quote! {
                impl !cxx_qt::Constructor<(i32, i32)> for T {}
            },
            "Negative impls for cxx_qt::Constructor are not allowed",
        );

        assert_parse_error(
            parse_quote! {
                impl<T> cxx_qt::Constructor<()> for T {}
            },
            "generics on impl block",
        );
        // TODO This should be allowed at some point if the lifetime is actually used.
        assert_parse_error(
            parse_quote! {
                impl<'a, 'b> cxx_qt::Constructor<()> for T {}
            },
            "multiple lifetimes on impl block",
        );

        assert_parse_error(
            parse_quote! {
                impl cxx_qt::Constructor<(), UnknownArguments=()> for X {}
            },
            "unknown named type argument",
        );
        assert_parse_error(
            parse_quote! {
                impl cxx_qt::Constructor<(), NewArguments=(), NewArguments=()> for X {}
            },
            "duplicate named type argument",
        );

        // Not a tuple, missing `,`
        assert_parse_error(
            parse_quote! {
                impl cxx_qt::Constructor<> for X {}
            },
            "cxx_qt::Constructor expects a tuple as the first generic argument",
        );

        assert_parse_error(
            parse_quote! {
                impl cxx_qt::Constructor<(i32)> for X {}
            },
            "type argument is not a tuple",
        );

        assert_parse_error(
            parse_quote! {
                impl cxx_qt::Constructor<(i32,String),'a> for X {}
            },
            "Expected associated type as a generic argument!",
        );

        assert_parse_error(
            parse_quote! {
                impl cxx_qt::Constructor<(T,S)> for X where S: Debug, T: Clone {}
            },
            "Where clauses are not allowed on cxx_qt::Constructor impls!",
        );

        assert_parse_error(
            parse_quote! {
                unsafe impl cxx_qt::Constructor<(i32,String)> for X {}
            },
            "Unnecessary unsafe around constructor impl.",
        );
    }

    #[test]
    fn parse_arguments_with_default_associated_types() {
        let constructor = Constructor::parse(parse_quote! {
            impl cxx_qt::Constructor<(i32, QString)> for X {}
        })
        .unwrap();

        assert_eq!(
            constructor.arguments,
            vec![parse_quote!(i32), parse_quote!(QString)]
        );
        assert!(constructor.new_arguments.is_empty());
        assert!(constructor.base_arguments.is_empty());
        assert!(constructor.initialize_arguments.is_empty());
        assert!(constructor.lifetime.is_none());
    }

    #[test]
    fn parse_full_argument_list() {
        let constructor = Constructor::parse(parse_quote! {
            impl cxx_qt::Constructor<
                (i32,),
                NewArguments=(i8,QString),
                InitializeArguments=(),
                BaseArguments=(i64,)
            > for X {}
        })
        .unwrap();

        assert_eq!(constructor.arguments, vec![parse_quote!(i32)]);
        assert_eq!(
            constructor.new_arguments,
            vec![parse_quote!(i8), parse_quote!(QString)]
        );
        assert!(constructor.initialize_arguments.is_empty());
        assert_eq!(constructor.base_arguments, vec![parse_quote!(i64)]);
        assert!(constructor.lifetime.is_none());
    }

    #[test]
    fn parse_generic_lifetime() {
        let constructor = Constructor::parse(parse_quote! {
            impl<'my_lifetime> cxx_qt::Constructor<()> for X {}
        })
        .unwrap();

        assert!(constructor.arguments.is_empty());
        assert!(constructor.base_arguments.is_empty());
        assert!(constructor.initialize_arguments.is_empty());
        assert!(constructor.new_arguments.is_empty());
        assert_eq!(constructor.lifetime, Some(parse_quote! { 'my_lifetime }));
    }
}
