// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax::{
    attribute::{attribute_find_path, attribute_tokens_to_ident},
    path::{path_angled_args_to_type_path, path_compare_str, path_to_single_ident},
};
use quote::format_ident;
use std::collections::HashMap;
use syn::{
    spanned::Spanned, Error, Ident, ImplItem, Item, ItemEnum, ItemImpl, ItemStruct, Result, Type,
    TypePath,
};

/// A representation of a QObject within a CXX-Qt [syn::ItemMod]
///
/// This has initial splitting of [syn::Item]'s into relevant blocks, other phases will
/// then mutate these [syn::Item]'s for generation purposes.
#[derive(Default)]
pub struct ParsedQObject {
    /// This is the Data struct that currently stores the properties for the QObject
    ///
    /// In the future this will be removed
    pub data_struct: Option<ItemStruct>,
    /// This is the RustObj struct that stores the invokables for the QObject
    pub rust_struct: Option<ItemStruct>,
    /// This is the Signals enum that defines the Q_SIGNALS for the QObject
    pub signal_enum: Option<ItemEnum>,
    /// This is the list of methods that need to be implemented on the C++ object in Rust
    ///
    /// TODO: in the future we will have a ParsedMethod this will have a field to say whether the
    /// method is invokable or not.
    /// Ones marked as invokable will need to be exposed to C++ as Q_INVOKABLE
    pub methods: Vec<ImplItem>,
    /// This is update request handler for the QObject
    ///
    /// In the future this may be removed
    pub update_requester_handler: Option<ItemImpl>,
    /// Items that we don't need to generate anything for CXX or C++
    /// eg impls on the Rust object or Default implementations
    pub others: Vec<Item>,
}

#[derive(Default)]
pub struct ParsedCxxQtData {
    /// Map of the QObjects defined in the module that will be used for code generation
    pub qobjects: HashMap<Ident, ParsedQObject>,
    /// Any `use` statements end up in the CXX-Qt generated module
    pub uses: Vec<Item>,
}

impl ParsedCxxQtData {
    /// Find the QObjects within the module and adds the keys into the qobjects HashMap
    pub fn find_qobject_keys(&mut self, items: &[Item]) -> Result<()> {
        for item in items {
            match item {
                Item::Struct(s) => {
                    // TODO: instead find the cxx_qt::qobject macro
                    // and support multiple QObjects in one block
                    if s.ident == "RustObj" {
                        // TODO: for now we only support one qobject per block
                        if !self.qobjects.is_empty() {
                            return Err(Error::new(
                                s.span(),
                                "Only on RustObj struct is supported per mod",
                            ));
                        }

                        // Note that we assume a compiler error will occur later
                        // if you had two structs with the same name
                        self.qobjects
                            .insert(s.ident.clone(), ParsedQObject::default());
                    }
                }
                _others => {}
            }
        }

        Ok(())
    }

    /// Determine if the given [syn::Item] is a CXX-Qt related item
    /// If it is then add the [syn::Item] into qobjects HashMap
    /// Otherwise return the [syn::Item] to pass through to CXX
    pub fn parse_cxx_qt_item(&mut self, item: Item) -> Result<Option<Item>> {
        Ok(match &item {
            Item::Enum(item_enum) => {
                // Check if the enum has cxx_qt::signals(T)
                if let Some(attr) = attribute_find_path(&item_enum.attrs, &["cxx_qt", "signals"]) {
                    let qobject = attribute_tokens_to_ident(attr)?;
                    // Find the matching QObject for the enum
                    if let Some(entry) = self.qobjects.get_mut(&qobject) {
                        entry.signal_enum = Some(item_enum.clone());
                        return Ok(None);
                    } else {
                        return Err(Error::new(
                            item_enum.span(),
                            "No matching QObject found for the given cxx_qt::signals<T> enum.",
                        ));
                    }
                }

                // Passthrough this unknown enum
                Some(item)
            }
            Item::Struct(s) => {
                // TODO: instead check for the cxx_qt::qobject macro
                //
                // // If the attribute is cxx_qt::qobject<T> then this the struct defining a qobject
                // if let Some(attr) = attribute_find_path(&s.attrs, &["cxx_qt", "qobject"]) {
                //     if let Some(qobject) = attribute_tokens_to_ident(attr)? {
                //         self
                //             .qobjects
                //             .entry(qobject)
                //             .and_modify(|value| value.rust_struct = Some(s.clone()));
                //         return Ok(None);
                //     }
                // }
                match s.ident.to_string().as_str() {
                    // TODO: for now we assume that Data is related to the only struct
                    // which is called "RustObj"
                    "Data" => {
                        self.qobjects
                            .entry(format_ident!("RustObj"))
                            .and_modify(|value| value.data_struct = Some(s.clone()));
                        None
                    }
                    // TODO: for now we assume that Data is related to the only struct
                    // which is called "RustObj"
                    "RustObj" => {
                        self.qobjects
                            .entry(s.ident.clone())
                            .and_modify(|value| value.rust_struct = Some(s.clone()));
                        None
                    }
                    _others => Some(item),
                }
            }
            Item::Impl(imp) => {
                // If the implementation has a cxx_qt::QObject
                // then this is the block of methods to be implemented on the C++ object
                if let Type::Path(TypePath { path, .. }) = imp.self_ty.as_ref() {
                    if path_compare_str(path, &["cxx_qt", "QObject"]) {
                        // Read the T from cxx_qt::QObject<T> and error if it's missing
                        let qobject_path = path_angled_args_to_type_path(path)?;
                        if let Some(value) = self
                            .qobjects
                            // Convert the path to a single ident, and error if it isn't
                            //
                            // TODO: we need to error if the ident isn't found?
                            .get_mut(&path_to_single_ident(&qobject_path)?)
                        {
                            // Extract the ImplItem's from each Impl block
                            // and add to the methods list
                            value.methods.extend(imp.items.iter().cloned());
                        } else {
                            return Err(Error::new(imp.span(), "No matching QObject found for the given cxx_qt::QObject<T> impl block."));
                        }
                        return Ok(None);
                    } else {
                        // TODO: once Data, RustObj, and impl UpdateRequestHandler are removed (?)
                        // other items can be ignored and just returned, so this block of code below
                        // can be removed in the future.
                        //
                        // For now we need to find the Data, RustObj, and impl UpdateRequestHandler
                        if path_compare_str(path, &["Data"]) {
                            // TODO: for now we assume that Data is related to the only struct
                            // which is called "RustObj"
                            self.qobjects
                                .entry(format_ident!("RustObj"))
                                .and_modify(|value| value.others.push(item));
                            return Ok(None);
                        } else if path_compare_str(path, &["RustObj"]) {
                            // If we are the UpdateRequestHandler, then we need to store in list
                            if let Some(trait_) = &imp.trait_ {
                                if let Some(first) = trait_.1.segments.first() {
                                    if first.ident == "UpdateRequestHandler" {
                                        self.qobjects
                                            .entry(format_ident!("RustObj"))
                                            // We assume that there is only one impl block from the compiler
                                            //
                                            // TODO: later this might be removed/changed anyway
                                            .and_modify(|value| {
                                                value.update_requester_handler = Some(imp.clone())
                                            });
                                        return Ok(None);
                                    }
                                }
                            }

                            self.qobjects
                                .entry(format_ident!("RustObj"))
                                .and_modify(|value| value.others.push(item));
                            return Ok(None);
                        }
                    }
                }

                Some(item)
            }
            Item::Use(_) => {
                // Any use statements go into the CXX-Qt generated block
                self.uses.push(item);
                None
            }
            _others => Some(item),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::tokens_to_syn;
    use quote::quote;
    use syn::ItemMod;

    /// The QObject ident used in these tests as the ident that already
    /// has been found.
    fn qobject_ident() -> Ident {
        format_ident!("RustObj")
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
                struct RustObj;
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
                struct RustObj;
                struct RustObj;
            }
        });
        let result = cxx_qt_data.find_qobject_keys(&module.content.unwrap().1);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_enum_valid_signals() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            #[cxx_qt::signals(RustObj)]
            enum MySignals {
                Ready,
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        assert!(cxx_qt_data.qobjects[&qobject_ident()].signal_enum.is_some());
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
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
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
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        assert!(cxx_qt_data.qobjects[&qobject_ident()].data_struct.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_valid_rustobj() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            struct RustObj;
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        assert!(cxx_qt_data.qobjects[&qobject_ident()].rust_struct.is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_struct_passthrough() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            struct Unknown;
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_valid_qobject() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            impl cxx_qt::QObject<RustObj> {
                #[invokable]
                fn invokable() {}
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
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
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].others.len(), 1);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_valid_rustobj() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            impl RustObj {
                fn method() {}
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        assert_eq!(cxx_qt_data.qobjects[&qobject_ident()].others.len(), 1);
    }

    #[test]
    fn test_find_and_merge_cxx_qt_item_impl_valid_update_request_handler() {
        let mut cxx_qt_data = create_parsed_cxx_qt_data();

        let item: Item = tokens_to_syn(quote! {
            impl UpdateRequestHandler for RustObj {
                fn method() {}
            }
        });
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
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
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
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
        let result = cxx_qt_data.parse_cxx_qt_item(item);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}
