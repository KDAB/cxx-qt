// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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
        document
            .on_contents_changed(move |document| {
                let qt_thread = qt_thread.clone();
                let text = document.to_plain_text().to_string();
                std::thread::spawn(move || {
                    let Ok(file) =
                        syn::parse_file(&text).map_err(|err| eprintln!("Parsing error: {err}"))
                    else {
                        return;
                    };
                    let output = QString::from(prettyplease::unparse(&file));
                    qt_thread
                        .queue(move |this| {
                            unsafe { this.output_document() }.set_plain_text(&output)
                        })
                        .ok();
                });
            })
            .release();
    }
}
