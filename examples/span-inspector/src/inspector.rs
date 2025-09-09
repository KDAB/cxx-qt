// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_gen::{write_rust, Parser};
use proc_macro2::{Span, TokenStream, TokenTree};
use std::str::FromStr;
use syn::{parse2, ItemMod};

#[cxx_qt::bridge]
mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++Qt" {
        include!(<QTextDocument>);
        #[qobject]
        type QTextDocument;

        #[cxx_name = "toPlainText"]
        fn to_plain_text(self: &QTextDocument) -> QString;

        #[cxx_name = "setPlainText"]
        fn set_plain_text(self: Pin<&mut QTextDocument>, content: &QString);

        #[cxx_name = "setHtml"]
        fn set_html(self: Pin<&mut QTextDocument>, content: &QString);

        #[qsignal]
        #[cxx_name = "contentsChanged"]
        fn contents_changed(self: Pin<&mut QTextDocument>);
    }

    unsafe extern "C++Qt" {
        include!(<QQuickTextDocument>);
        #[qobject]
        type QQuickTextDocument;

        #[cxx_name = "textDocument"]
        fn text_document(self: &QQuickTextDocument) -> *mut QTextDocument;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(*mut QQuickTextDocument, input, READ, NOTIFY, WRITE=set_input)]
        #[qproperty(*mut QQuickTextDocument, output)]
        type SpanInspector = super::SpanInspectorRust;

        unsafe fn set_input(self: Pin<&mut SpanInspector>, input: *mut QQuickTextDocument);

        #[qinvokable]
        #[cxx_name = "rebuildOutput"]
        fn rebuild_output(self: Pin<&mut SpanInspector>, cursor_position: i32);
    }

    impl UniquePtr<QTextDocument> {}
    impl cxx_qt::Threading for SpanInspector {}
}

use cxx_qt::{CxxQtType, Threading};
use qobject::{QQuickTextDocument, QString, QTextDocument};
use std::{pin::Pin, ptr};

pub struct SpanInspectorRust {
    input: *mut QQuickTextDocument,
    output: *mut QQuickTextDocument,
}

impl Default for SpanInspectorRust {
    fn default() -> Self {
        Self {
            input: ptr::null_mut(),
            output: ptr::null_mut(),
        }
    }
}

impl qobject::SpanInspector {
    unsafe fn output_document(&self) -> Pin<&mut QTextDocument> {
        if self.output.is_null() {
            panic!("Output document must be set!");
        }
        let output = unsafe { &*self.output };
        unsafe { Pin::new_unchecked(&mut *output.text_document()) }
    }

    fn set_input(mut self: Pin<&mut Self>, input: *mut QQuickTextDocument) {
        self.as_mut().rust_mut().input = input;
        self.as_mut().input_changed();
    }

    fn rebuild_output(self: Pin<&mut Self>, cursor_position: i32) {
        let input = unsafe { Pin::new_unchecked(&mut *self.input) };
        let qt_thread = self.qt_thread();
        unsafe { self.output_document() }.set_html(&QString::from(String::from("expanding...")));

        let text = unsafe { Pin::new_unchecked(&mut *input.text_document()) }.to_plain_text();

        std::thread::spawn(move || {
            let output = QString::from(
                match Self::expand(&text.to_string(), cursor_position as usize) {
                    Ok((expanded, span_data)) => {
                        let Ok(file) = syn::parse_file(expanded.as_str())
                            .map_err(|err| eprintln!("Parsing error: {err}"))
                        else {
                            return;
                        };
                        Self::build_html(&prettyplease::unparse(&file), span_data)
                    }
                    Err(error) => Self::build_error_html(&error),
                },
            );
            qt_thread
                .queue(move |this| unsafe { this.output_document() }.set_html(&output))
                .ok();
        });
    }

    // Expand input code as #[cxxqt_qt::bridge] would do
    fn expand(input: &str, cursor_position: usize) -> Result<(String, Vec<(bool, bool)>), String> {
        let input_stream: TokenStream = TokenStream::from_str(input).map_err(|e| e.to_string())?;

        let mut module: ItemMod = parse2(input_stream.clone()).map_err(|e| e.to_string())?;

        let args = TokenStream::default();
        let args_input = format!("#[cxx_qt::bridge({args})] mod dummy;");
        let attrs = syn::parse_str::<ItemMod>(&args_input)
            .map_err(|e| e.to_string())?
            .attrs;
        module.attrs = attrs.into_iter().chain(module.attrs).collect();

        let output_stream = Self::extract_and_generate(module);

        let target_span: Option<Span> = Self::flatten_tokenstream(input_stream)
            .into_iter()
            .find(|token| {
                let range = token.span().byte_range();
                range.start <= cursor_position && range.end >= cursor_position
            })
            .map(|token| token.span());

        let span_data: Vec<(bool, bool)> = Self::flatten_tokenstream(output_stream.clone())
            .into_iter()
            // prettyplease may insert extra tokens.
            // This filter simply ignores them.
            .filter(|token| {
                let string = token.to_string();
                !matches!(string.as_str(), "," | "}" | "{")
            })
            .map(|token| {
                (
                    target_span
                        .map(|s| s.byte_range().eq(token.span().byte_range()))
                        .unwrap_or_else(|| false),
                    token.span().byte_range().start == 0,
                )
            })
            .collect();

        Ok((format!("{}", output_stream), span_data))
    }

    // Take the module and C++ namespace and generate the rust code
    fn extract_and_generate(module: ItemMod) -> TokenStream {
        Parser::from(module)
            .and_then(|parser| cxx_qt_gen::GeneratedRustBlocks::from(&parser))
            .map(|generated_rust| write_rust(&generated_rust, None))
            .unwrap_or_else(|err| err.to_compile_error())
    }

    fn build_error_html(input: &str) -> String {
        let style = String::from(
            "
            <style>
                .error {
                    whitespace = normal;
                    color: red;
                }
            </style> 
        ",
        );
        format!("<!DOCTYPE html><html><head>{style}</head><body><p class=\"error\">{input}</p></body></html>")
    }

    fn flatten_tokenstream(stream: TokenStream) -> Vec<TokenTree> {
        let mut output: Vec<TokenTree> = vec![];
        for token in stream {
            match token {
                TokenTree::Group(group) => {
                    output.extend(Self::flatten_tokenstream(group.stream()));
                }
                other => {
                    output.push(other);
                }
            }
        }
        output
    }

    fn build_html(input: &str, span_data: Vec<(bool, bool)>) -> String {
        let flat_tokenstream = Self::flatten_tokenstream(TokenStream::from_str(input).unwrap());
        let mut highlighted_string = input.to_string();

        let mut token_position = span_data.len();
        for token in flat_tokenstream.into_iter().rev() {
            // prettyplease may insert extra tokens.
            // This `if` statement simply ignores them.
            let token_string = token.to_string();
            if !matches!(token_string.as_str(), "," | "{" | "}") {
                token_position -= 1;
            }
            if span_data.get(token_position).unwrap().0 {
                highlighted_string.replace_range(
                    token.span().byte_range(),
                    format!(
                        "<span class=\"highlight\">{}</span>",
                        Self::sanitize(&token.to_string())
                    )
                    .as_str(),
                );
            } else if span_data.get(token_position).unwrap().1 {
                highlighted_string.replace_range(
                    token.span().byte_range(),
                    format!(
                        "<span class=\"generated\">{}</span>",
                        Self::sanitize(&token.to_string())
                    )
                    .as_str(),
                );
            } else {
                highlighted_string.replace_range(
                    token.span().byte_range(),
                    Self::sanitize(&token.to_string()).as_str(),
                );
            }
        }

        let style = String::from(
            "
            <style> 
                .highlight {
                    background-color: #ff00ff;
                    padding: 2px 6px;
                    border-radius: 6px;
                }
                .generated {
                    color: rgba(255, 255, 255, 100);
                    padding: 2px 6px;
                    border-radius: 6px;
                }
            </style>
        ",
        );

        format!("<!DOCTYPE html><html><head>{style}</head><body><pre>{highlighted_string}</pre></body></html>")
    }

    fn sanitize(input: &str) -> String {
        input
            .chars()
            .map(|char| match char {
                '>' => "&gt;".to_string(),
                '<' => "&lt;".to_string(),
                '&' => "&amp;".to_string(),
                '"' => "&quot;".to_string(),
                _ => char.to_string(),
            })
            .collect()
    }
}
