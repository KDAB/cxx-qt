// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_gen::{write_rust, Parser};
use proc_macro2::{Span, TokenStream, TokenTree};
use std::{str::FromStr, sync::{Arc, Mutex}, usize};
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
        #[cxx_name = "updateCursorPosition"]
        fn update_cursor_position(self: Pin<&mut SpanInspector>, pos: i32);
    }

    impl UniquePtr<QTextDocument> {}
    impl cxx_qt::Threading for SpanInspector {}
}

use cxx_qt::{CxxQtType, Threading};
use qobject::{QQuickTextDocument, QString, QTextDocument};
use std::{pin::Pin, ptr};

pub struct SpanInspectorRust {
    cursor_position: Arc<Mutex<i32>>,
    input: *mut QQuickTextDocument,
    output: *mut QQuickTextDocument,
}

impl Default for SpanInspectorRust {
    fn default() -> Self {
        Self {
            cursor_position: Arc::new(Mutex::new(3)),
            input: ptr::null_mut(),
            output: ptr::null_mut(),
        }
    }
}

impl qobject::SpanInspector {
    unsafe fn output_document(&self) -> Pin<&mut QTextDocument> {
        if self.output == ptr::null_mut() {
            panic!("Output document must be set!");
        }
        let output = unsafe { &*self.output };
        unsafe { Pin::new_unchecked(&mut *output.text_document()) }
    }

    fn set_input(mut self: Pin<&mut Self>, input: *mut QQuickTextDocument) {
        self.as_mut().rust_mut().input = input;
        self.as_mut().input_changed();

        let input = unsafe { Pin::new_unchecked(&mut *input) };
        let document = unsafe { Pin::new_unchecked(&mut *input.text_document()) };
        let qt_thread = self.qt_thread();
        let cursor_position = self.cursor_position.clone();
        document
            .on_contents_changed(move |document| {
                let qt_thread = qt_thread.clone();
                let text = document.to_plain_text().to_string();
                println!("{}", text);
                let cursor_position = cursor_position.clone();
                std::thread::spawn(move || {
                    let cursor_position = match cursor_position.lock() {
                        Ok(mutex) => mutex,
                        Err(poison_error) => poison_error.into_inner()
                    };
                    let (expanded, span_data) = Self::expand(&text, *cursor_position as usize);
                    let Ok(file) = syn::parse_file(expanded.as_str())
                        .map_err(|err| eprintln!("Parsing error: {err}"))
                    else {
                        return;
                    };
                    let output = QString::from(Self::build_html(prettyplease::unparse(&file), span_data));
                    println!("thomas: {}", *cursor_position);
                    qt_thread
                        .queue(move |this| {
                            unsafe { this.output_document() }.set_html(&QString::from(output))
                        })
                        .ok();
                });
            })
            .release();
    }

    fn update_cursor_position(self: Pin<&mut Self>, pos: i32){
        let mut cursor_position = match self.cursor_position.lock() {
            Ok(mutex) => mutex,
            Err(poison_error) => poison_error.into_inner()
        };
        *cursor_position = pos;
    }

    // Expand input code as #[cxxqt_qt::bridge] would do
    fn expand(input: &str, cursor_position: usize) -> (String, Vec<bool>){
        let input_stream: TokenStream = TokenStream::from_str(input).unwrap();
        
        let mut module: ItemMod = parse2(input_stream.clone()).expect("could not generate ItemMod");

        let args = TokenStream::default();
        let args_input = format!("#[cxx_qt::bridge({args})] mod dummy;");
        let attrs = syn::parse_str::<ItemMod>(&args_input).unwrap().attrs;
        module.attrs = attrs.into_iter().chain(module.attrs).collect();

        let output_stream = Self::extract_and_generate(module);
        let target_span= input_stream.into_iter().find( |token| {
            let range = token.span().byte_range();
            range.start <= cursor_position && range.end >= cursor_position
        }).map(|token| token.span()).unwrap();
        let span_data = Self::get_span_data(output_stream.clone(), target_span);
        
        (format!("{}", output_stream), span_data)
    }

    // Take the module and C++ namespace and generate the rust code
    fn extract_and_generate(module: ItemMod) -> TokenStream {
        Parser::from(module)
            .and_then(|parser| cxx_qt_gen::GeneratedRustBlocks::from(&parser))
            .map(|generated_rust| write_rust(&generated_rust, None))
            .unwrap_or_else(|err| err.to_compile_error())
            .into()
    }

    fn build_html(input: String, span_data: Vec<bool>) -> String{
        fn highlight(token_stream: TokenStream, mut text: String, span_data: &Vec<bool>, mut token_position: usize) -> (String, usize) {
            let token_vec: Vec<TokenTree> = token_stream.into_iter().collect();
            for token in token_vec.into_iter().rev() {
                match &token {
                    TokenTree::Group(group) => {
                    (text, token_position) = highlight(group.stream(), text, span_data, token_position);
                    },
                    _ => {
                        token_position = token_position - 1;
                        println!("debug: token: {} , token_position: {} , is_highlighted: {}", token, token_position, *span_data.get(token_position).unwrap());
                        if *span_data.get(token_position).unwrap() {
                            text.replace_range(token.span().byte_range(), format!("<span class=\"highlight\">{}</span>", token).as_str());
                        }
                    },
                }
            }
            return (text, token_position);
        }

        let token_stream: TokenStream = TokenStream::from_str(input.as_str()).unwrap();
        let (highlighted_string, _) = highlight(token_stream, input, &span_data, span_data.len());
        let style: String = String::from("
            <style> 
                .highlight {
                    background-color: #ff00ff;
                    padding: 2px 6px;
                    border-radius: 6px;
                }
            </style>
        ");
        format!("<!DOCTYPE html><html><head>{}</head><body><pre>{}</pre></body></html>", style, highlighted_string)
    }

   
    fn get_span_data(token_stream: TokenStream, target_span: Span) -> Vec<bool> {
        let mut vec: Vec<bool> = vec![];
        for token in token_stream {
            match token {
                TokenTree::Group(group) => {
                    vec.extend(Self::get_span_data(group.stream(), target_span));
                }
                _ => {
                    println!("vergleich: {} ,input: {:?} , output: {:?} , token: {} ", target_span.byte_range().eq(token.span().byte_range()), target_span.byte_range(), token.span().byte_range(), token);
                    vec.push(target_span.byte_range().eq(token.span().byte_range()));
                }
            }
        }
        println!("{:?}", vec);
        vec
    }
}
