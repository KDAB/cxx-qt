// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt_gen::{write_rust, Parser};
use proc_macro2::{TokenStream, TokenTree};
use std::str::FromStr;
use syn::{parse2, ItemMod};

#[cxx_qt::bridge]
mod qobject {
    #[repr(i32)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum MoveMode {
        MoveAnchor = 0,
        KeepAnchor = 1,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;

        include!("SyntaxHighlighter.h");
        type SyntaxHighlighter;

        include!(<QTextCharFormat>);
        type QTextCharFormat;

        include!(<QTextCursor>);
        type QTextCursor;
        type MoveMode;

        include!(<QBrush>);
        type QBrush;

        #[cxx_name = "setRenderError"]
        fn set_render_error(self: Pin<&mut SyntaxHighlighter>, b: bool);

        unsafe fn new_syntax_highlighter(
            text_document: *mut QQuickTextDocument,
        ) -> UniquePtr<SyntaxHighlighter>;

        unsafe fn new_QTextCursor(text_document: *mut QQuickTextDocument)
            -> UniquePtr<QTextCursor>;

        fn new_QTextCharFormat() -> UniquePtr<QTextCharFormat>;

        unsafe fn new_QBrush(color: &QColor) -> UniquePtr<QBrush>;

        #[cxx_name = "setPosition"]
        fn set_position(self: Pin<&mut QTextCursor>, pos: i32, m: MoveMode);

        #[cxx_name = "setCharFormat"]
        fn set_char_format(self: Pin<&mut QTextCursor>, format: &QTextCharFormat);

        #[cxx_name = "mergeCharFormat"]
        fn merge_char_format(self: Pin<&mut QTextCursor>, format: &QTextCharFormat);

        #[cxx_name = "setForeground"]
        fn set_foreground(self: Pin<&mut QTextCharFormat>, brush: &QBrush);

        #[cxx_name = "setBackground"]
        fn set_background(self: Pin<&mut QTextCharFormat>, brush: &QBrush);

        #[cxx_name = "setFontWeight"]
        fn set_font_weight(self: Pin<&mut QTextCharFormat>, weight: i32);

        #[cxx_name = "setFontItalic"]
        fn set_font_italic(self: Pin<&mut QTextCharFormat>, italic: bool);

        #[cxx_name = "setFontUnderline"]
        fn set_font_underline(self: Pin<&mut QTextCharFormat>, underline: bool);

        #[cxx_name = "setColor"]
        fn set_color(self: Pin<&mut QBrush>, color: &QColor);

        #[cxx_name = "setDocument"]
        unsafe fn set_document(self: Pin<&mut SyntaxHighlighter>, doc: *mut QTextDocument);
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
        #[qproperty(*mut QQuickTextDocument, output, READ, NOTIFY, WRITE=set_output)]
        type SpanInspector = super::SpanInspectorRust;

        unsafe fn set_input(self: Pin<&mut SpanInspector>, input: *mut QQuickTextDocument);
        unsafe fn set_output(self: Pin<&mut SpanInspector>, output: *mut QQuickTextDocument);

        #[qinvokable]
        #[cxx_name = "rebuildOutput"]
        fn rebuild_output(self: Pin<&mut SpanInspector>, cursor_position: i32);
    }

    impl UniquePtr<QTextDocument> {}
    impl cxx_qt::Threading for SpanInspector {}
}

use crate::inspector::qobject::{
    new_QBrush, new_QTextCharFormat, new_QTextCursor, MoveMode, QColor,
};
use cxx::UniquePtr;
use cxx_qt::{CxxQtType, Threading};
use qobject::{QQuickTextDocument, QString, QTextDocument, SyntaxHighlighter};
use std::{pin::Pin, ptr};

pub struct SpanInspectorRust {
    input: *mut QQuickTextDocument,
    output: *mut QQuickTextDocument,
    input_highlighter: UniquePtr<SyntaxHighlighter>,
    output_highlighter: UniquePtr<SyntaxHighlighter>,
    thread_count: u32,
}

impl Default for SpanInspectorRust {
    fn default() -> Self {
        Self {
            input: ptr::null_mut(),
            output: ptr::null_mut(),
            input_highlighter: UniquePtr::null(),
            output_highlighter: UniquePtr::null(),
            thread_count: 0,
        }
    }
}

// This Trait is necessary because `prettyplease` seems to add certain characters in some situations.
// By simply ignoring these characters, we can work around the problem.
impl<I> FilterPrettyPlease for I where I: Iterator<Item = TokenTree> + Sized {}
trait FilterPrettyPlease: Iterator<Item = TokenTree> + Sized {
    fn filter_pretty_please(self) -> impl Iterator<Item = TokenTree> {
        self.filter(|token| {
            let string = token.to_string();
            !matches!(string.as_str(), "," | "}" | "{")
        })
    }
}

#[derive(Debug)]
enum TokenFlag {
    Default,
    //tokens highlighted by the cursor
    Highlighted,
    //tokens wich are complety generated by the expansion
    Generated,
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
        unsafe {
            self.as_mut().rust_mut().input_highlighter = qobject::new_syntax_highlighter(input);
        }
        self.as_mut().input_changed();
    }

    fn set_output(mut self: Pin<&mut Self>, output: *mut QQuickTextDocument) {
        self.as_mut().rust_mut().output = output;
        unsafe {
            self.as_mut().rust_mut().output_highlighter = qobject::new_syntax_highlighter(output);
        }
        self.as_mut().output_changed();
    }

    fn rebuild_output(mut self: Pin<&mut Self>, cursor_position: i32) {
        let input = unsafe { Pin::new_unchecked(&mut *self.input) };
        let qt_thread = self.qt_thread();
        unsafe { self.output_document() }.set_html(&QString::from(String::from("expanding...")));

        let text = unsafe { Pin::new_unchecked(&mut *input.text_document()) }.to_plain_text();

        self.as_mut().rust_mut().thread_count += 1;
        let thread_id = self.thread_count;

        std::thread::spawn(move || {
            let (output, token_flags) =
                match Self::expand(&text.to_string(), cursor_position as usize) {
                    Ok((expanded, token_flags)) => {
                        let Ok(file) = syn::parse_file(expanded.as_str())
                            .map_err(|err| eprintln!("Parsing error: {err}"))
                        else {
                            return;
                        };
                        (prettyplease::unparse(&file), Some(token_flags))
                    }
                    Err(error) => (error, None),
                };

            qt_thread
                .queue(move |mut this| {
                    if thread_id != this.thread_count {
                        return;
                    }
                    unsafe { this.output_document() }.set_plain_text(&QString::from(&output));
                    unsafe {
                        this.as_mut()
                            .rust_mut()
                            .output_highlighter
                            .pin_mut()
                            .set_document(std::ptr::null_mut());
                    }

                    let mut cursor = unsafe { new_QTextCursor(*this.output()) };

                    match token_flags {
                        Some(token_flags) => {
                            let flat_tokenstream =
                                Self::flatten_tokenstream(TokenStream::from_str(&output).unwrap());

                            for (token, flag) in flat_tokenstream
                                .into_iter()
                                .filter_pretty_please()
                                .zip(token_flags.into_iter())
                            {
                                let byte_range = token.span().byte_range();
                                cursor
                                    .pin_mut()
                                    .set_position(byte_range.start as i32, MoveMode::MoveAnchor);
                                cursor
                                    .pin_mut()
                                    .set_position(byte_range.end as i32, MoveMode::KeepAnchor);

                                let mut format = new_QTextCharFormat();

                                /*let brush = match flag {
                                    TokenFlag::Highlighted => unsafe {
                                        new_QBrush(&QColor::from_rgb(255, 0, 0))
                                    },
                                    TokenFlag::Generated => unsafe {
                                        new_QBrush(&QColor::from_rgb(0, 255, 0))
                                    },
                                    TokenFlag::Default => unsafe {
                                        new_QBrush(&QColor::from_rgb(0, 0, 255))
                                    },
                                };
                                //let brush = unsafe { new_QBrush(&QColor::from_rgb(0, 255, 0)) };
                                format.pin_mut().set_background(&*brush);*/
                                match flag {
                                    TokenFlag::Generated => format.pin_mut().set_font_italic(true),
                                    TokenFlag::Highlighted => {
                                        format.pin_mut().set_font_underline(true)
                                    }
                                    TokenFlag::Default => {}
                                }
                                cursor.pin_mut().merge_char_format(&*format);
                            }
                        }
                        None => {
                            let mut format = new_QTextCharFormat();
                            cursor.pin_mut().set_position(0, MoveMode::MoveAnchor);
                            cursor
                                .pin_mut()
                                .set_position(output.len() as i32, MoveMode::KeepAnchor);
                            let brush = unsafe { new_QBrush(&QColor::from_rgb(255, 0, 0)) };
                            //let color = brush.color();
                            format.pin_mut().set_foreground(&*brush);
                            cursor.pin_mut().merge_char_format(&*format);
                        }
                    }

                    unsafe {
                        let output_doc =
                            Pin::get_unchecked_mut(this.output_document()) as *mut QTextDocument;
                        this.as_mut()
                            .rust_mut()
                            .output_highlighter
                            .pin_mut()
                            .set_document(output_doc);
                    };
                })
                .ok();
        });
    }

    // Expand input code as #[cxxqt_qt::bridge] would do
    fn expand(input: &str, cursor_position: usize) -> Result<(String, Vec<TokenFlag>), String> {
        let input_stream: TokenStream = TokenStream::from_str(input).map_err(|e| e.to_string())?;

        let mut module: ItemMod = parse2(input_stream.clone()).map_err(|e| e.to_string())?;

        let args = TokenStream::default();
        let args_input = format!("#[cxx_qt::bridge({args})] mod dummy;");
        let attrs = syn::parse_str::<ItemMod>(&args_input)
            .map_err(|e| e.to_string())?
            .attrs;
        module.attrs = attrs.into_iter().chain(module.attrs).collect();

        let output_stream = Self::extract_and_generate(module);
        let target_range = Self::flatten_tokenstream(input_stream)
            .into_iter()
            .find(|token| {
                let range = token.span().byte_range();
                range.start <= cursor_position && range.end >= cursor_position
            })
            .map(|token| token.span().byte_range());

        let token_flags: Vec<TokenFlag> = Self::flatten_tokenstream(output_stream.clone())
            .into_iter()
            .filter_pretty_please()
            .map(
                |token| match (token.span().byte_range(), target_range.clone()) {
                    (range, Some(target_range)) if range == target_range => TokenFlag::Highlighted,
                    (range, _) if range.start == 0 => TokenFlag::Generated,
                    _ => TokenFlag::Default,
                },
            )
            .collect();

        Ok((format!("{}", output_stream), token_flags))
    }

    // Take the module and C++ namespace and generate the rust code
    fn extract_and_generate(module: ItemMod) -> TokenStream {
        Parser::from(module)
            .and_then(|parser| cxx_qt_gen::GeneratedRustBlocks::from(&parser))
            .map(|generated_rust| write_rust(&generated_rust, None))
            .unwrap_or_else(|err| err.to_compile_error())
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
}
