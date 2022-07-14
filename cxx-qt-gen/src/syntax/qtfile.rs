// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::CxxQtItem;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{AttrStyle, Attribute, Result};

pub struct CxxQtFile {
    pub shebang: Option<String>,
    pub attrs: Vec<Attribute>,
    pub items: Vec<CxxQtItem>,
}

impl Parse for CxxQtFile {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(CxxQtFile {
            shebang: None,
            attrs: input.call(Attribute::parse_inner)?,
            items: {
                let mut items = Vec::new();
                while !input.is_empty() {
                    items.push(input.parse()?);
                }
                items
            },
        })
    }
}

impl ToTokens for CxxQtFile {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            self.attrs
                .iter()
                .filter(|attr| matches!(attr.style, AttrStyle::Inner(_))),
        );
        tokens.append_all(&self.items);
    }
}

// TODO: can we contribute a generic parse_file implementation upstream?
// as the rest of this file (parse_qt_file and whitespace) is just so that
// we can read a file into a CxxQtFile rather than a File.
pub fn parse_qt_file(mut content: &str) -> Result<CxxQtFile> {
    println!("parsing file...");
    // Strip the BOM if it is present
    const BOM: &str = "\u{feff}";
    if content.starts_with(BOM) {
        content = &content[BOM.len()..];
    }

    let mut shebang = None;
    if content.starts_with("#!") {
        let rest = whitespace::skip(&content[2..]);
        if !rest.starts_with('[') {
            if let Some(idx) = content.find('\n') {
                shebang = Some(content[..idx].to_string());
                content = &content[idx..];
            } else {
                shebang = Some(content.to_string());
                content = "";
            }
        }
    }

    let mut file: CxxQtFile = syn::parse_str(content)?;
    file.shebang = shebang;
    Ok(file)
}

// Copied from src/whitespace.rs in syn crate
mod whitespace {
    pub fn skip(mut s: &str) -> &str {
        'skip: while !s.is_empty() {
            let byte = s.as_bytes()[0];
            if byte == b'/' {
                if s.starts_with("//")
                    && (!s.starts_with("///") || s.starts_with("////"))
                    && !s.starts_with("//!")
                {
                    if let Some(i) = s.find('\n') {
                        s = &s[i + 1..];
                        continue;
                    } else {
                        return "";
                    }
                } else if s.starts_with("/**/") {
                    s = &s[4..];
                    continue;
                } else if s.starts_with("/*")
                    && (!s.starts_with("/**") || s.starts_with("/***"))
                    && !s.starts_with("/*!")
                {
                    let mut depth = 0;
                    let bytes = s.as_bytes();
                    let mut i = 0;
                    let upper = bytes.len() - 1;
                    while i < upper {
                        if bytes[i] == b'/' && bytes[i + 1] == b'*' {
                            depth += 1;
                            i += 1; // eat '*'
                        } else if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                            depth -= 1;
                            if depth == 0 {
                                s = &s[i + 2..];
                                continue 'skip;
                            }
                            i += 1; // eat '/'
                        }
                        i += 1;
                    }
                    return s;
                }
            }
            match byte {
                b' ' | 0x09..=0x0d => {
                    s = &s[1..];
                    continue;
                }
                b if b <= 0x7f => {}
                _ => {
                    let ch = s.chars().next().unwrap();
                    if is_whitespace(ch) {
                        s = &s[ch.len_utf8()..];
                        continue;
                    }
                }
            }
            return s;
        }
        s
    }

    fn is_whitespace(ch: char) -> bool {
        // Rust treats left-to-right mark and right-to-left mark as whitespace
        ch.is_whitespace() || ch == '\u{200e}' || ch == '\u{200f}'
    }
}
