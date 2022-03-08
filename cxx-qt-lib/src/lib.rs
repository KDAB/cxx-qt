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

pub const QCOLOR_CXX_HEADER: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/include/qcolor_cxx.h"));
pub const QCOLOR_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qcolor_cxx.cpp"));
pub const QDATE_CXX_HEADER: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/include/qdate_cxx.h"));
pub const QDATE_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qdate_cxx.cpp"));
pub const QPOINT_CXX_HEADER: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/include/qpoint_cxx.h"));
pub const QPOINT_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qpoint_cxx.cpp"));
pub const QPOINTF_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    "/cxx-qt-lib/include/qpointf_cxx.h"
));
pub const QPOINTF_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qpointf_cxx.cpp"));
pub const QRECT_CXX_HEADER: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/include/qrect_cxx.h"));
pub const QRECT_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qrect_cxx.cpp"));
pub const QRECTF_CXX_HEADER: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/include/qrectf_cxx.h"));
pub const QRECTF_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qrectf_cxx.cpp"));
pub const QSIZE_CXX_HEADER: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/include/qsize_cxx.h"));
pub const QSIZE_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qsize_cxx.cpp"));
pub const QSIZEF_CXX_HEADER: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/include/qsizef_cxx.h"));
pub const QSIZEF_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qsizef_cxx.cpp"));
pub const QSTRING_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    "/cxx-qt-lib/include/qstring_cxx.h"
));
pub const QSTRING_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qstring_cxx.cpp"));
pub const QTIME_CXX_HEADER: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/include/qtime_cxx.h"));
pub const QTIME_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qtime_cxx.cpp"));
pub const QURL_CXX_HEADER: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/include/qurl_cxx.h"));
pub const QURL_CXX_SOURCE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cxx-qt-lib/src/qurl_cxx.cpp"));

pub trait PropertyChangeHandler<C, P> {
    fn handle_property_change(&mut self, cpp: &mut C, property: P);
}

pub trait ToUniquePtr {
    type CppType;

    fn to_unique_ptr(self) -> cxx::UniquePtr<Self::CppType>
    where
        Self::CppType: cxx::memory::UniquePtrTarget;
}
