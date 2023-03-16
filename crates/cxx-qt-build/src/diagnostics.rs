// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module takes care of displaying errors emitted by CXX-Qt as nicely-printed diagnostics
//! using codespan-reporting.

use std::{ops::Range, path::PathBuf};

use proc_macro2::LineColumn;

pub(crate) struct Diagnostic {
    file_path: PathBuf,
    error: cxx_qt_gen::Error,
}

impl Diagnostic {
    pub(crate) fn new(file_path: PathBuf, error: cxx_qt_gen::Error) -> Self {
        Self { file_path, error }
    }

    fn byte_span_in(&self, source: &str) -> Option<Range<usize>> {
        let syn_err = &self.error;
        let span = syn_err.span();

        let start_offset = line_column_to_byte_in(span.start(), source)?;
        let end_offset = line_column_to_byte_in(span.end(), source)?;

        Some(start_offset..end_offset)
    }

    fn create_codespan_diagnostic(
        &self,
        source: &str,
    ) -> codespan_reporting::diagnostic::Diagnostic<()> {
        use codespan_reporting::diagnostic::Label;

        let syn_err = &self.error;
        let mut diagnostic = codespan_reporting::diagnostic::Diagnostic::error()
            .with_message(format!("{syn_err}"))
            .with_code("cxxqt".to_owned());

        if let Some(span) = self.byte_span_in(source) {
            diagnostic = diagnostic.with_labels(vec![Label::primary((), span)]);
        }

        diagnostic
    }

    fn try_report(&self) -> Result<(), ()> {
        use codespan_reporting::{
            files::SimpleFile,
            term::{
                self,
                termcolor::{ColorChoice, StandardStream},
                Config,
            },
        };
        let source_string = std::fs::read_to_string(&self.file_path).map_err(|_| ())?;

        let diagnostic = self.create_codespan_diagnostic(source_string.as_ref());

        let file_path = self.file_path.display();
        let file = SimpleFile::new(format!("{file_path}"), source_string);

        let stderr = StandardStream::stderr(ColorChoice::Auto);
        let mut writer = stderr.lock();

        term::emit(&mut writer, &Config::default(), &file, &diagnostic).map_err(|_| ())
    }

    pub(crate) fn report(&self) {
        // If loading the source file fails, or printing to stderr isn't
        // possible, we try panicing as a last resort.
        self.try_report().unwrap_or_else(|_| {
            panic!("{}", self.error);
        })
    }
}

fn line_column_to_byte_in(line_column: LineColumn, source: &str) -> Option<usize> {
    let mut line = 1;
    source
        .char_indices()
        .skip_while(|(_byte, char)| {
            let should_skip = line_column.line > line;
            // Regarding Windows compatibility:
            // \n in Rust references one Line feed (LF) character.
            // As new lines in Windows are marked by FIRST a Carriage return (CR), then an LF
            // character, we can simply ignore the CR character, as the new line only starts
            // after a LF character.
            // This code currently cannot handle the case where a CR character is used without
            // an LF character (old Mac OS style).
            if *char == '\n' {
                line += 1;
            }
            should_skip
        })
        .skip(line_column.column)
        .map(|(byte, _char)| byte)
        .next()
}
