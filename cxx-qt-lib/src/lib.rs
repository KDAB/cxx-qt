// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod update_requester;
pub use update_requester::UpdateRequestHandler;

mod qcolor;
pub use qcolor::{Color, QColor};

mod qdate;
pub use qdate::QDate;

mod qdatetime;
pub use qdatetime::{DateTime, QDateTime};

mod qrect;
pub use qrect::QRect;

mod qrectf;
pub use qrectf::QRectF;

mod qsize;
pub use qsize::QSize;

mod qsizef;
pub use qsizef::QSizeF;

mod qstring;
pub use qstring::QString;

mod qtime;
pub use qtime::QTime;

mod qpoint;
pub use qpoint::QPoint;

mod qpointf;
pub use qpointf::QPointF;

mod qurl;
pub use qurl::{QUrl, Url};

mod qvariant;
pub use qvariant::{QVariant, Variant, VariantValue};

mod map_qt_value;
pub use map_qt_value::*;

pub const QT_TYPES_HEADER: &str = include_str!("../include/qt_types.h");
pub const QT_TYPES_SOURCE: &str = include_str!("qt_types.cpp");

pub trait PropertyChangeHandler<C, P> {
    fn handle_property_change(&mut self, cpp: &mut C, property: P);
}

pub trait ToUniquePtr {
    type CppType;

    fn to_unique_ptr(self) -> cxx::UniquePtr<Self::CppType>
    where
        Self::CppType: cxx::memory::UniquePtrTarget;
}
