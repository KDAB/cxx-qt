// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
// SPDX-FileContributor: Quentin Weber <quentin.weber@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::syntax_highlighter::SyntaxHighlighterRust;
use cxx_qt_gen::{write_rust, Parser};
use proc_macro2::{TokenStream, TokenTree};
use std::default::Default;
use std::ops::Range;
use std::str::FromStr;
use syn::{parse2, ItemMod};

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "make_q_brush"]
        #[namespace = "rust::cxxqtlib1"]
        fn make_unique(color: &QColor) -> UniquePtr<QBrush>;

        #[rust_name = "make_q_text_char_format"]
        #[namespace = "rust::cxxqtlib1"]
        fn make_unique() -> UniquePtr<QTextCharFormat>;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;

        include!("helper.h");
        type QSyntaxHighlighterCXX;

        include!(<QTextCharFormat>);
        type QTextCharFormat;

        include!(<QBrush>);
        type QBrush;

        include!(<QTextBlock>);
        type QTextBlock;

        #[cxx_name = "length"]
        fn length(self: &QTextBlock) -> i32;

        #[cxx_name = "position"]
        fn position(self: &QTextBlock) -> i32;

        #[cxx_name = "setForeground"]
        fn set_foreground(self: Pin<&mut QTextCharFormat>, brush: &QBrush);

        #[cxx_name = "setBackground"]
        fn set_background(self: Pin<&mut QTextCharFormat>, brush: &QBrush);

        #[cxx_name = "setColor"]
        fn set_color(self: Pin<&mut QBrush>, color: &QColor);
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

        #[qinvokable]
        #[cxx_name = "updateCursor"]
        fn update_cursor(self: Pin<&mut SpanInspector>, cursor_position: i32);
    }

    unsafe extern "C++Qt" {
        include!(<QSyntaxHighlighter>);
        #[qobject]
        type QSyntaxHighlighter;

        /// Creates a unique syntax highlighter instance.
        ///
        /// # Safety
        /// - `text_document` must be a valid, non-null pointer to a `QTextDocument`.
        /// - The caller must ensure the document outlives the returned `UniquePtr`.
        #[rust_name = "make_q_syntax_highlighter"]
        #[namespace = "rust::cxxqtlib1"]
        unsafe fn make_unique(text_document: *mut QTextDocument) -> UniquePtr<SyntaxHighlighter>;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[base = QSyntaxHighlighterCXX]
        type SyntaxHighlighter = super::SyntaxHighlighterRust;

        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "highlightBlock"]
        fn highlight_block(self: Pin<&mut SyntaxHighlighter>, text: &QString);

        #[inherit]
        #[cxx_name = "setFormat"]
        fn set_format(
            self: Pin<&mut SyntaxHighlighter>,
            start: i32,
            end: i32,
            format: &QTextCharFormat,
        );

        #[inherit]
        #[cxx_name = "setCurrentBlockState"]
        fn set_current_block_state(self: Pin<&mut SyntaxHighlighter>, new_state: i32);

        #[inherit]
        #[cxx_name = "previousBlockState"]
        fn previous_block_state(self: &SyntaxHighlighter) -> i32;

        #[inherit]
        #[cxx_name = "currentBlockCXX"]
        fn current_block(self: Pin<&mut SyntaxHighlighter>) -> UniquePtr<QTextBlock>;

        #[inherit]
        #[cxx_name = "rehighlight"]
        fn rehighlight(self: Pin<&mut SyntaxHighlighter>);
    }

    impl
        cxx_qt::Constructor<
            (*mut QTextDocument,),
            BaseArguments = (*mut QTextDocument,),
            NewArguments = (),
        > for SyntaxHighlighter
    {
    }

    impl UniquePtr<QTextDocument> {}
    impl cxx_qt::Threading for SpanInspector {}
}

use crate::inspector::qobject::{make_q_syntax_highlighter, SyntaxHighlighter};
use cxx::UniquePtr;
use cxx_qt::{CxxQtType, Threading};
use qobject::{QQuickTextDocument, QString, QTextDocument};
use std::{pin::Pin, ptr};

pub struct SpanInspectorRust {
    input: *mut QQuickTextDocument,
    output: *mut QQuickTextDocument,
    input_highlighter: UniquePtr<SyntaxHighlighter>,
    output_highlighter: UniquePtr<SyntaxHighlighter>,
    thread_count: u32,
    last_expansion: Result<Expansion, String>,
}

impl Default for SpanInspectorRust {
    fn default() -> Self {
        Self {
            input: ptr::null_mut(),
            output: ptr::null_mut(),
            input_highlighter: UniquePtr::null(),
            output_highlighter: UniquePtr::null(),
            thread_count: 0,
            last_expansion: Err("".into()),
        }
    }
}

impl cxx_qt::Constructor<(*mut QTextDocument,)> for qobject::SyntaxHighlighter {
    type BaseArguments = (*mut QTextDocument,);
    type InitializeArguments = ();
    type NewArguments = ();

    fn route_arguments(
        args: (*mut QTextDocument,),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), args, ())
    }

    fn new(_: ()) -> SyntaxHighlighterRust {
        SyntaxHighlighterRust::default()
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

#[derive(Debug, Clone)]
pub enum TokenFlag {
    //tokens from user input
    Original,
    //tokens highlighted by the cursor
    Highlighted,
    //tokens wich are complety generated by the expansion
    Generated,
}

#[derive(Clone)]
struct Expansion {
    formatted_rust: String,
    output_token_ranges: Vec<Range<usize>>,
    input_token_ranges: Vec<Range<usize>>,
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
            let input = Pin::new_unchecked(&mut *input);
            self.as_mut().rust_mut().input_highlighter =
                make_q_syntax_highlighter(input.text_document());
        }
        self.as_mut().input_changed();
    }

    fn set_output(mut self: Pin<&mut Self>, output: *mut QQuickTextDocument) {
        self.as_mut().rust_mut().output = output;
        unsafe {
            let output = Pin::new_unchecked(&mut *output);
            self.as_mut().rust_mut().output_highlighter =
                make_q_syntax_highlighter(output.text_document());

            self.as_mut()
                .rust_mut()
                .output_highlighter
                .pin_mut()
                .rust_mut()
                .is_output = true;
        }
        self.as_mut().output_changed();
    }

    fn update_cursor(mut self: Pin<&mut Self>, cursor_position: i32) {
        if let Ok(last_expansion) = &self.last_expansion.clone() {
            let mut rust_self = self.as_mut().rust_mut();
            let mut output_highlighter = rust_self.output_highlighter.pin_mut();

            output_highlighter.as_mut().rust_mut().char_flags = Some(Self::get_char_flags(
                last_expansion,
                cursor_position as usize,
            ));

            output_highlighter.as_mut().rehighlight();
        }
    }

    fn rebuild_output(mut self: Pin<&mut Self>, cursor_position: i32) {
        let input = unsafe { Pin::new_unchecked(&mut *self.input) };
        let qt_thread = self.qt_thread();
        unsafe { self.output_document() }.set_html(&QString::from(String::from("expanding...")));

        let text = unsafe { Pin::new_unchecked(&mut *input.text_document()) }.to_plain_text();

        self.as_mut().rust_mut().thread_count += 1;
        let thread_id = self.thread_count;

        std::thread::spawn(move || {
            let expand_result = Self::expand(&text.to_string());

            let (formatted_rust, char_flags) = match expand_result.clone() {
                Ok(expanded) => {
                    let char_flags = Self::get_char_flags(&expanded, cursor_position as usize);

                    (expanded.formatted_rust, Some(char_flags))
                }
                Err(error) => (error, None),
            };

            qt_thread
                .queue(move |mut this| {
                    if thread_id != this.thread_count {
                        return;
                    }

                    this.as_mut().rust_mut().last_expansion = expand_result;

                    this.as_mut()
                        .rust_mut()
                        .output_highlighter
                        .pin_mut()
                        .rust_mut()
                        .char_flags = char_flags;

                    unsafe { this.output_document() }
                        .set_plain_text(&QString::from(&formatted_rust));
                })
                .ok();
        });
    }

    // Expand input code as #[cxxqt_qt::bridge] would do
    fn expand(input: &str) -> Result<Expansion, String> {
        let input_stream: TokenStream = TokenStream::from_str(input).map_err(|e| e.to_string())?;

        let mut module: ItemMod = parse2(input_stream.clone()).map_err(|e| e.to_string())?;

        let args = TokenStream::default();
        let args_input = format!("#[cxx_qt::bridge({args})] mod dummy;");
        let attrs = syn::parse_str::<ItemMod>(&args_input)
            .map_err(|e| e.to_string())?
            .attrs;
        module.attrs = attrs.into_iter().chain(module.attrs).collect();

        let output_stream = Self::extract_and_generate(module);

        let file = syn::parse_file(&output_stream.to_string())
            .map_err(|err| eprintln!("Parsing error: {err}"))
            .unwrap();

        let formatted_rust = prettyplease::unparse(&file);

        let output_token_ranges = Self::flatten_tokenstream(output_stream)
            .into_iter()
            .filter_pretty_please()
            .map(|token| token.span().byte_range())
            .collect();

        let input_token_ranges = Self::flatten_tokenstream(input_stream)
            .into_iter()
            .filter_pretty_please()
            .map(|token| token.span().byte_range())
            .collect();

        Ok(Expansion {
            formatted_rust,
            output_token_ranges,
            input_token_ranges,
        })
    }

    fn get_char_flags(last_expansion: &Expansion, cursor_position: usize) -> Vec<TokenFlag> {
        let target_range = last_expansion
            .input_token_ranges
            .clone()
            .into_iter()
            .find(|range| range.start <= cursor_position && range.end >= cursor_position);

        let token_flags: Vec<TokenFlag> = last_expansion
            .output_token_ranges
            .clone()
            .into_iter()
            .map(|range| {
                if let Some(target_range) = &target_range {
                    if *target_range == range {
                        return TokenFlag::Highlighted;
                    }
                }
                if range.start == 0 {
                    return TokenFlag::Generated;
                }
                TokenFlag::Original
            })
            .collect();

        let flat_tokenstream = Self::flatten_tokenstream(
            TokenStream::from_str(&last_expansion.formatted_rust).unwrap(),
        );

        let mut char_flags = vec![TokenFlag::Generated; last_expansion.formatted_rust.len() + 1];

        for (token, flag) in flat_tokenstream
            .into_iter()
            .filter_pretty_please()
            .zip(token_flags.into_iter())
        {
            for i in token.span().byte_range() {
                char_flags[i] = flag.clone();
            }
        }
        char_flags
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
