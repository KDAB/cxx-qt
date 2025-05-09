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

    #[qenum(Greeter)]
    pub enum Language {
        English,
        German,
        French,
    }

    #[qenum(Greeter)]
    pub enum Greeting {
        Hello,
        Bye,
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(Greeting, greeting)]
        #[qproperty(Language, language)]
        type Greeter = super::GreeterRust;

        #[qinvokable]
        fn greet(&self) -> QString;
    }
}

use qobject::*;

impl Greeting {
    fn translate(&self, language: Language) -> String {
        match (self, language) {
            (&Greeting::Hello, Language::English) => "Hello, World!",
            (&Greeting::Hello, Language::German) => "Hallo, Welt!",
            (&Greeting::Hello, Language::French) => "Bonjour, le monde!",
            (&Greeting::Bye, Language::English) => "Bye!",
            (&Greeting::Bye, Language::German) => "Auf Wiedersehen!",
            (&Greeting::Bye, Language::French) => "Au revoir!",
            _ => "🤯",
        }
        .to_string()
    }
}

pub struct GreeterRust {
    greeting: Greeting,
    language: Language,
}

impl Default for GreeterRust {
    fn default() -> Self {
        Self {
            greeting: Greeting::Hello,
            language: Language::English,
        }
    }
}

use cxx_qt_lib::QString;

impl qobject::Greeter {
    fn greet(&self) -> QString {
        QString::from(self.greeting.translate(self.language))
    }
}

fn main() {
    use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/kdab/tutorial/qml/main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
