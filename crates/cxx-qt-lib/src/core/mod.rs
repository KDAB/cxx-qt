// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod qbytearray;
pub use qbytearray::QByteArray;

mod qcoreapplication;
pub use qcoreapplication::QCoreApplication;

mod qdate;
pub use qdate::QDate;

#[cfg(not(target_os = "emscripten"))]
mod qdatetime;
#[cfg(not(target_os = "emscripten"))]
pub use qdatetime::QDateTime;

mod qhash;
pub use qhash::{QHash, QHashPair, QHashPair_QString_QVariant, QHashPair_i32_QByteArray};

mod qline;
pub use qline::QLine;

mod qlinef;
pub use qlinef::QLineF;

mod qlist;
pub use qlist::{QList, QListElement};

mod qmap;
pub use qmap::{QMap, QMapPair, QMapPair_QString_QVariant};

mod qmargins;
pub use qmargins::QMargins;

mod qmarginsf;
pub use qmarginsf::QMarginsF;

mod qmetaobjectconnection;
pub use qmetaobjectconnection::QMetaObjectConnection;

mod qmodelindex;
pub use qmodelindex::QModelIndex;

mod qpersistentmodelindex;
pub use qpersistentmodelindex::QPersistentModelIndex;

mod qrect;
pub use qrect::QRect;

mod qrectf;
pub use qrectf::QRectF;

mod qset;
pub use qset::{QSet, QSetElement};

mod qsize;
pub use qsize::QSize;

mod qsizef;
pub use qsizef::QSizeF;

mod qstring;
pub use qstring::QString;

mod qstringlist;
pub use qstringlist::QStringList;

mod qt;
pub use qt::{
    AspectRatioMode, CaseSensitivity, ConnectionType, DateFormat, PenCapStyle, PenJoinStyle,
    PenStyle, SplitBehaviorFlags, TimeSpec, TransformationMode,
};

mod qtime;
pub use qtime::QTime;

#[cfg(not(target_os = "emscripten"))]
mod qtimezone;
#[cfg(not(target_os = "emscripten"))]
pub use qtimezone::QTimeZone;

mod qpoint;
pub use qpoint::QPoint;

mod qpointf;
pub use qpointf::QPointF;

mod qurl;
pub use qurl::QUrl;

mod qvariant;
pub use qvariant::{QVariant, QVariantValue};

mod qvector;
pub use qvector::{QVector, QVectorElement};

#[cxx::bridge]
mod ffi {
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        type c_void;
    }
}

/// This is a workaround for CXX missing support for `*mut c_void`/`*const c_void` types.
///
/// To use this type add this to your bridge:
/// ```rust
/// # #[cxx::bridge]
/// # mod ffi {
/// #
/// #[namespace = "rust::cxxqtlib1"]
/// unsafe extern "C++" {
///     include!("cxx-qt-lib/common.h");
///     type c_void = cxx_qt_lib::c_void;
/// }
/// #
/// # }
/// #
/// # fn main() {}
/// ```
///
/// See: <https://github.com/dtolnay/cxx/issues/1049>
pub use ffi::c_void;
