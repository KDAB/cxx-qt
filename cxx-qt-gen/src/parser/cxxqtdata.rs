// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::parser::{qobject::ParsedQObject, signals::ParsedSignalsEnum};
use crate::syntax::{
    attribute::{attribute_find_path, attribute_tokens_to_ident},
    path::{path_angled_args_to_type_path, path_compare_str, path_to_single_ident},
};
use std::collections::HashMap;
use syn::{
    spanned::Spanned, Error, Ident, Item, ItemEnum, ItemImpl, ItemStruct, Result, Type, TypePath,
};

#[derive(Default)]
pub struct ParsedCxxQtData {
    /// Map of the QObjects defined in the module that will be used for code generation
    pub qobjects: HashMap<Ident, ParsedQObject>,
    /// The namespace of the CXX-Qt module
    pub namespace: String,
    /// Any `use` statements end up in the CXX-Qt generated module
    pub uses: Vec<Item>,
}

impl ParsedCxxQtData {
    /// Find the QObjects within the module and adds the keys into the qobjects HashMap
    pub fn find_qobject_keys(&mut self, items: &[Item]) -> Result<()> {
        for item in items {
            if let Item::Struct(s) = item {
                if attribute_find_path(&s.attrs, &["cxx_qt", "qobject"]).is_some() {
                    // TODO: for now we only support one qobject per block
                    if !self.qobjects.is_empty() {
                        return Err(Error::new(
                            s.span(),
                            "Only one #[cxx_qt::qobject] struct is supported per mod",
                        ));
                    }

                    // Note that we assume a compiler error will occur later
                    // if you had two structs with the same name
                    self.qobjects
                        .insert(s.ident.clone(), ParsedQObject::default());
                }
            }
        }

        Ok(())
    }

    /// Determine if the given [syn::Item] is a CXX-Qt related item
    /// If it is then add the [syn::Item] into qobjects HashMap
    /// Otherwise return the [syn::Item] to pass through to CXX
    pub fn parse_cxx_qt_item(&mut self, item: Item) -> Result<Option<Item>> {
        match item {
            Item::Enum(item_enum) => self.parse_enum(item_enum),
            Item::Struct(item_struct) => self.parse_struct(item_struct),
            Item::Impl(imp) => self.parse_impl(imp),
            Item::Use(_) => {
                // Any use statements go into the CXX-Qt generated block
                self.uses.push(item);
                Ok(None)
            }
            _ => Ok(Some(item)),
        }
    }

    /// Parse a [syn::ItemEnum] into the qobjects if it's a CXX-Qt signal
    /// otherwise return as a [syn::Item] to pass through.
    fn parse_enum(&mut self, item_enum: ItemEnum) -> Result<Option<Item>> {
        // Check if the enum has cxx_qt::signals(T)
        if let Some(index) = attribute_find_path(&item_enum.attrs, &["cxx_qt", "signals"]) {
            let ident = attribute_tokens_to_ident(&item_enum.attrs[index])?;
            // Find the matching QObject for the enum
            if let Some(qobject) = self.qobjects.get_mut(&ident) {
                qobject.signals = Some(ParsedSignalsEnum::from(&item_enum, index)?);
                return Ok(None);
            } else {
                return Err(Error::new(
                    item_enum.span(),
                    "No matching QObject found for the given cxx_qt::signals<T> enum.",
                ));
            }
        }

        // Passthrough this unknown enum
        Ok(Some(Item::Enum(item_enum)))
    }

    /// Parse a [syn::ItemImpl] into the qobjects if it's a CXX-Qt implementation
    /// otherwise return as a [syn::Item] to pass through.
    fn parse_impl(&mut self, imp: ItemImpl) -> Result<Option<Item>> {
        // If the implementation has a cxx_qt::QObject
        // then this is the block of methods to be implemented on the C++ object
        if let Type::Path(TypePath { path, .. }) = imp.self_ty.as_ref() {
            if path_compare_str(path, &["cxx_qt", "QObject"]) {
                // Read the T from cxx_qt::QObject<T> and error if it's missing
                let qobject_path = path_angled_args_to_type_path(path)?;
                if let Some(qobject) = self
                    .qobjects
                    // Convert the path to a single ident, and error if it isn't
                    .get_mut(&path_to_single_ident(&qobject_path)?)
                {
                    // Extract the ImplItem's from each Impl block
                    qobject.parse_impl_items(&imp.items)?;
                } else {
                    return Err(Error::new(
                        imp.span(),
                        "No matching QObject found for the given cxx_qt::QObject<T> impl block.",
                    ));
                }
                return Ok(None);
            } else {
                // TODO: Once Data and "RustObj" have been merged this can be removed
                if !self.qobjects.is_empty() && path_compare_str(path, &["Data"]) {
                    // TODO: for now we assume that Data is related to the only struct
                    let qobject_ident = self.qobjects.keys().next().unwrap().clone();
                    self.qobjects
                        .entry(qobject_ident)
                        .and_modify(|qobject| qobject.others.push(Item::Impl(imp)));
                    return Ok(None);
                // Find if we are an impl block for a qobject
                } else if let Some(qobject) = self.qobjects.get_mut(&path_to_single_ident(path)?) {
                    // If we are the UpdateRequestHandler, then we need to store in list
                    //
                    // TODO: once impl UpdateRequestHandler is removed this block can go
                    if let Some(trait_) = &imp.trait_ {
                        if let Some(first) = trait_.1.segments.first() {
                            if first.ident == "UpdateRequestHandler" {
                                // We assume that there is only one impl block from the compiler
                                qobject.update_requester_handler = Some(imp.clone());
                                return Ok(None);
                            }
                        }
                    }

                    qobject.others.push(Item::Impl(imp));
                    return Ok(None);
                }
            }
        }

        Ok(Some(Item::Impl(imp)))
    }

    /// Parse a [syn::ItemStruct] into the qobjects if it's a CXX-Qt struct
    /// otherwise return as a [syn::Item] to pass through.
    fn parse_struct(&mut self, s: ItemStruct) -> Result<Option<Item>> {
        // If the attribute is #[cxx_qt::qobject] then this the struct defining a qobject
        if let Some(index) = attribute_find_path(&s.attrs, &["cxx_qt", "qobject"]) {
            // Remove the macro from the struct
            //
            // TODO: we need to read the base class from the macro
            let mut s = s.clone();
            s.attrs.remove(index);

            if let Some(qobject) = self.qobjects.get_mut(&s.ident) {
                // Parse any properties in the struct
                // and remove the #[qproperty] attribute
                qobject.parse_struct_fields(&mut s.fields)?;

                qobject.qobject_struct = Some(s);
                return Ok(None);
            } else {
                return Err(Error::new(
                    s.span(),
                    "cxx_qt::qobject struct was not found by find_qobject_keys",
                ));
            }
        }

        // TODO: for now we assume that Data is related to the only struct in the qobjects
        //
        // Once Data and "RustObj" have been merged this can be removed
        match s.ident.to_string().as_str() {
            "Data" if !self.qobjects.is_empty() => {
                let qobject_ident = self.qobjects.keys().next().unwrap().clone();
                self.qobjects
                    .entry(qobject_ident)
                    .and_modify(|qobject| qobject.data_struct = Some(s.clone()));
                Ok(None)
            }
            _others => Ok(Some(Item::Struct(s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::{format_ident, quote};
    use syn::ItemMod;

    /// The QObject ident used in these tests as the ident that already
    /// has been found.
    fn qobject_ident() -> Ident {
        format_ident!("MyObject")
    }

    /// Creates a ParsedCxxQtData with a QObject definition already found
    fn create_parsed_cxx_qt_data() -> ParsedCxxQtData {
        let mut cxx_qt_data = ParsedCxxQtData::default();
        cxx_qt_data.qobjects.entry(qobject_ident()).or_default();
        cxx_qt_data
    }

    #[test]
    fn test_find_qobjects_one_qobject() {
        let mut cxx_qt_data = ParsedCxxQtData::default();

        let module: ItemMod = tokens_to_syn(quote! {
            mod module {
                struct Other;
                #[cxx_qt::qobject]
                struct MyObject;
            }
        });
        let result = cxx_qt_data.find_qobject_keys(&module.content.unwrap().1);
        assert!(result.is_ok());
        assert_eq!(cxx_qt_data.qobjects.len(), 1);
        assert!(cxx_qt_data.qobjects.contains_key(&qobject_ident()));
    }

    #[test]
    fn test_find_qobjects_duplicate_qobject() {
        let mut cxx_qt_data = ParsedCxxQtData::default();

        let module: ItemMod = tokens_to_syn(quote! {
            mod module {
                struct Other;
                #[cxx_qt::qobject]
                struct MyObject;
                #[cxx_qt::qobject]
                struct MyObject;
            }
        });
        let result = cxx_qt_data.find_qobject_keys(&module.content.unwrap().1);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_qobjects_no_macro() {
        let mut cxx_qt_data = ParsedCxxQtData::default();

        let module: ItemMod = tokens_to_syn(quote! {
            mod module {
                struct Other;
                struct MyObject;
            }
        });
        let result = cxx_qt_data.find_qobject_keys(&module.content.unwrap().1);
        assert!(result.is_ok());
        assert_eq!(cxx_qt_data.qobjects.len(), 0);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_enum_valid_signals() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            #[cxx_qt::signals(MyObject)]
            enum MySignals {
                Ready,
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert!(cxx_qt_data.qobjects[&qobject_ident()].signals.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_enum_unknown_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        // Valid signals enum but missing QObject
        let item: Item = tokens_to_syn(quote! {
            #[cxx_qt::signals(UnknownObj)]
            enum MySignals {
                Ready,
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_enum_passthrough() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            enum MySignals {
                Ready,
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_enum_error() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            #[cxx_qt::signals]
            enum MySignals {
                Ready,
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_valid_data() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            struct Data;
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert!(cxx_qt_data.qobjects[&qobject_ident()].data_struct.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_valid_rustobj() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            #[cxx_qt::qobject]
            struct MyObject;
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert!(cxx_qt_data.qobjects[&qobject_ident()]
            .qobject_struct
            .is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_valid_properties() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            #[cxx_qt::qobject]
            struct MyObject {
                #[qproperty]
                int_property: i32,

                #[qproperty]
                pub public_property: i32,
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].properties.len(), 2);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_valid_properties_and_fields() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            #[cxx_qt::qobject]
            struct MyObject {
                #[qproperty]
                int_property: i32,

                #[qproperty]
                pub public_property: i32,

                field: i32,
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].properties.len(), 2);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_valid_fields() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            #[cxx_qt::qobject]
            struct MyObject {
                field: i32,
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].properties.len(), 0);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_passthrough() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            struct Unknown;
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_valid_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            impl cxx_qt::QObject<MyObject> {
                #[invokable]
                fn invokable() {}

                fn cpp_context() {}
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].invokables.len(), 1);
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].methods.len(), 1);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_invalid_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            impl cxx_qt::QObject {
                #[invokable]
                fn invokable() {}
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_unknown_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            impl cxx_qt::QObject<UnknownObj> {
                #[invokable]
                fn invokable() {}
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_valid_data() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            impl Data {
                fn method() {}
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].others.len(), 1);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_valid_rustobj() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            impl MyObject {
                fn method() {}
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].others.len(), 1);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_valid_update_request_handler() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            impl UpdateRequestHandler for MyObject {
                fn method() {}
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert!(cxx_qt_data.qobjects[&qobject_ident()]
            .update_requester_handler
            .is_some(),);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_uses() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            use std::collections::HashMap;
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_none());
        assert_eq!(cxx_qt_data.uses.len(), 1);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_passthrough() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            extern "Rust" {
                fn test();
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item).unwrap();
        assert!(result.is_some());
    }
}
