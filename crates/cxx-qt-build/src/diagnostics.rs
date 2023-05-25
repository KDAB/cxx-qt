// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module takes care of displaying errors emitted by CXX-Qt as nicely-printed diagnostics
//! using codespan-reporting.

use std::{fmt::Display, ops::Range, path::PathBuf};

use proc_macro2::{LineColumn, Span};

/// We need to wrap the CXX and CXX-Qt errors in a single error type so that they
/// can be returned from the generation phases. This then allows for us to not unwrap
/// the CXX failures meaning that we can display the macro expansion correctly.
#[derive(Debug)]
pub(crate) enum GeneratedError {
    Cxx(cxx_gen::Error),
    CxxQt(cxx_qt_gen::Error),
}

impl Display for GeneratedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratedError::Cxx(err) => write!(f, "{}", err),
            GeneratedError::CxxQt(err) => write!(f, "{}", err),
        }
    }
}

impl From<cxx_gen::Error> for GeneratedError {
    fn from(err: cxx_gen::Error) -> Self {
        Self::Cxx(err)
    }
}

impl From<cxx_qt_gen::Error> for GeneratedError {
    fn from(err: cxx_qt_gen::Error) -> Self {
        Self::CxxQt(err)
    }
}

impl GeneratedError {
    fn context(&self) -> &str {
        match self {
            GeneratedError::Cxx(_) => "cxx",
            GeneratedError::CxxQt(_) => "cxxqt",
        }
    }

    fn span(&self) -> Option<Span> {
        match self {
            GeneratedError::Cxx(cxx_err) => cxx_err.span(),
            GeneratedError::CxxQt(syn_err) => Some(syn_err.span()),
        }
    }
}

impl std::iter::IntoIterator for GeneratedError {
    type Item = GeneratedError;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            GeneratedError::Cxx(cxx_err) => IntoIter::Cxx(cxx_err.into_iter()),
            GeneratedError::CxxQt(_) => IntoIter::CxxQt(std::iter::once(self)),
        }
    }
}

pub(crate) enum IntoIter {
    Cxx(<cxx_gen::Error as std::iter::IntoIterator>::IntoIter),
    CxxQt(std::iter::Once<GeneratedError>),
}

impl Iterator for IntoIter {
    type Item = GeneratedError;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IntoIter::Cxx(cxx_iter) => cxx_iter.next().map(GeneratedError::from),
            IntoIter::CxxQt(cxxqt_iter) => cxxqt_iter.next(),
        }
    }
}

pub(crate) struct Diagnostic {
    file_path: PathBuf,
    errors: Vec<GeneratedError>,
}

impl Diagnostic {
    pub(crate) fn new(file_path: PathBuf, error: GeneratedError) -> Self {
        Self {
            file_path,
            errors: error.into_iter().collect(),
        }
    }

    fn byte_span_in(error: &GeneratedError, source: &str) -> Option<Range<usize>> {
        error.span().and_then(|span| {
            let start_offset = line_column_to_byte_in(span.start(), source)?;
            let end_offset = line_column_to_byte_in(span.end(), source)?;

            Some(start_offset..end_offset)
        })
    }

    fn create_codespan_diagnostic(
        source: &str,
        error: &GeneratedError,
    ) -> codespan_reporting::diagnostic::Diagnostic<()> {
        use codespan_reporting::diagnostic::Label;

        let mut diagnostic = codespan_reporting::diagnostic::Diagnostic::error()
            .with_message(format!("{error}"))
            .with_code(error.context());

        if let Some(span) = Self::byte_span_in(error, source) {
            diagnostic = diagnostic.with_labels(vec![Label::primary((), span)]);
        }

        diagnostic
    }

    fn create_codespan_diagnostics(
        &self,
        source: &str,
    ) -> Vec<codespan_reporting::diagnostic::Diagnostic<()>> {
        self.errors
            .iter()
            .map(|error| Self::create_codespan_diagnostic(source, error))
            .collect()
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

        let diagnostics = self.create_codespan_diagnostics(source_string.as_ref());

        let file_path = self.file_path.display();
        let file = SimpleFile::new(format!("{file_path}"), source_string);

        let stderr = StandardStream::stderr(ColorChoice::Auto);
        let mut writer = stderr.lock();

        diagnostics.into_iter().try_for_each(|diagnostic| {
            term::emit(&mut writer, &Config::default(), &file, &diagnostic).map_err(|_| ())
        })
    }

    pub(crate) fn report(self) {
        // If loading the source file fails, or printing to stderr isn't
        // possible, we try panicing as a last resort.
        self.try_report().unwrap_or_else(|_| {
            panic!("{}", self.errors.first().unwrap());
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
