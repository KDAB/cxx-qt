// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx_qt::bridge(cxx_file_stem = "multiple_qobjects")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }

    #[cxx_qt::qobject]
    pub struct FirstObject {
        #[qproperty]
        counter: i32,
        #[qproperty]
        color: QColor,
    }

    impl Default for FirstObject {
        fn default() -> Self {
            Self {
                counter: 10,
                color: QColor::from_rgba(0, 0, 255, 255),
            }
        }
    }

    #[cxx_qt::qsignals(FirstObject)]
    pub enum FirstSignals {
        Accepted,
        Rejected,
    }

    impl qobject::FirstObject {
        #[qinvokable]
        pub fn increment(mut self: Pin<&mut Self>) {
            let new_value = self.as_ref().counter() + 1;
            self.as_mut().set_counter(new_value);

            if new_value % 2 == 0 {
                self.as_mut().set_color(QColor::from_rgba(0, 0, 255, 255));
                self.emit(FirstSignals::Accepted);
            } else {
                self.as_mut().set_color(QColor::from_rgba(255, 0, 0, 255));
                self.emit(FirstSignals::Rejected);
            }
        }
    }

    #[cxx_qt::qobject]
    pub struct SecondObject {
        #[qproperty]
        counter: i32,
        #[qproperty]
        url: QUrl,
    }

    impl Default for SecondObject {
        fn default() -> Self {
            Self {
                counter: 100,
                url: QUrl::from("https://github.com/kdab/cxx-qt"),
            }
        }
    }

    #[cxx_qt::qsignals(SecondObject)]
    pub enum SecondSignals {
        Accepted,
        Rejected,
    }

    impl qobject::SecondObject {
        #[qinvokable]
        pub fn increment(mut self: Pin<&mut Self>) {
            let new_value = self.as_ref().counter() + 1;
            self.as_mut().set_counter(new_value);

            if new_value % 5 == 0 {
                self.as_mut()
                    .set_url(QUrl::from("https://github.com/kdab/cxx-qt"));
                self.emit(SecondSignals::Accepted);
            } else {
                self.as_mut().set_url(QUrl::from("https://kdab.com"));
                self.emit(SecondSignals::Rejected);
            }
        }
    }
}
