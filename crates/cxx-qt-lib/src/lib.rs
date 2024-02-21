// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(not(any(qt_version_major = "5", qt_version_major = "6")))]
compile_error!("qt_version_major must be either \"5\" or \"6\"");

mod core;

pub use crate::core::*;

#[cfg(feature = "qt_gui")]
mod gui;
#[cfg(feature = "qt_gui")]
pub use crate::gui::*;

#[cfg(feature = "qt_qml")]
mod qml;
#[cfg(feature = "qt_qml")]
pub use crate::qml::*;

/// Retrieves the headers for cxx-qt-lib
///
/// These can be passed into [cxx_qt_build::CxxQtBuilder].
pub fn build_opts() -> cxx_qt_build::CxxQtBuildersOpts {
    let mut opts = cxx_qt_build::CxxQtBuildersOpts::default();

    for (file_contents, file_name) in [
        (include_str!("../include/core/qbytearray.h"), "qbytearray.h"),
        (
            include_str!("../include/core/qcoreapplication.h"),
            "qcoreapplication.h",
        ),
        (include_str!("../include/core/qdate.h"), "qdate.h"),
        (include_str!("../include/core/qdatetime.h"), "qdatetime.h"),
        (include_str!("../include/core/qhash.h"), "qhash.h"),
        (include_str!("../include/core/qline.h"), "qline.h"),
        (include_str!("../include/core/qlinef.h"), "qlinef.h"),
        (include_str!("../include/core/qlist.h"), "qlist.h"),
        (
            include_str!("../include/core/qlist_qvector.h"),
            "qlist_qvector.h",
        ),
        (include_str!("../include/core/qmap.h"), "qmap.h"),
        (include_str!("../include/core/qmargins.h"), "qmargins.h"),
        (include_str!("../include/core/qmarginsf.h"), "qmarginsf.h"),
        (
            include_str!("../include/core/qmetaobjectconnection.h"),
            "qmetaobjectconnection.h",
        ),
        (
            include_str!("../include/core/qmodelindex.h"),
            "qmodelindex.h",
        ),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qpen.h"), "qpen.h"),
        (
            include_str!("../include/core/qpersistentmodelindex.h"),
            "qpersistentmodelindex.h",
        ),
        (include_str!("../include/core/qpoint.h"), "qpoint.h"),
        (include_str!("../include/core/qpointf.h"), "qpointf.h"),
        (include_str!("../include/core/qrect.h"), "qrect.h"),
        (include_str!("../include/core/qrectf.h"), "qrectf.h"),
        (include_str!("../include/core/qset.h"), "qset.h"),
        (include_str!("../include/core/qsize.h"), "qsize.h"),
        (include_str!("../include/core/qsizef.h"), "qsizef.h"),
        (include_str!("../include/core/qstring.h"), "qstring.h"),
        (
            include_str!("../include/core/qstringlist.h"),
            "qstringlist.h",
        ),
        (include_str!("../include/core/qt.h"), "qt.h"),
        (include_str!("../include/core/qtime.h"), "qtime.h"),
        (include_str!("../include/core/qtimezone.h"), "qtimezone.h"),
        (include_str!("../include/core/qurl.h"), "qurl.h"),
        (include_str!("../include/core/qvariant.h"), "qvariant.h"),
        (include_str!("../include/core/qvector.h"), "qvector.h"),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qcolor.h"), "qcolor.h"),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qfont.h"), "qfont.h"),
        #[cfg(feature = "qt_gui")]
        (
            include_str!("../include/gui/qguiapplication.h"),
            "qguiapplication.h",
        ),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qimage.h"), "qimage.h"),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qpolygon.h"), "qpolygon.h"),
        (include_str!("../include/gui/qpolygonf.h"), "qpolygonf.h"),
        (
            include_str!("../include/gui/qpainterpath.h"),
            "qpainterpath.h",
        ),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qpainter.h"), "qpainter.h"),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qvector2d.h"), "qvector2d.h"),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qvector3d.h"), "qvector3d.h"),
        #[cfg(feature = "qt_gui")]
        (include_str!("../include/gui/qvector4d.h"), "qvector4d.h"),
        #[cfg(feature = "qt_qml")]
        (
            include_str!("../include/qml/qqmlapplicationengine.h"),
            "qqmlapplicationengine.h",
        ),
        #[cfg(feature = "qt_qml")]
        (include_str!("../include/qml/qqmlengine.h"), "qqmlengine.h"),
        (include_str!("../include/common.h"), "common.h"),
    ] {
        opts = opts.header(file_contents, "cxx-qt-lib", file_name);
    }

    #[cfg(feature = "qt_gui")]
    {
        opts = opts.define("CXX_QT_GUI_FEATURE");
    }
    #[cfg(feature = "qt_qml")]
    {
        opts = opts.define("CXX_QT_QML_FEATURE");
    }

    opts
}
