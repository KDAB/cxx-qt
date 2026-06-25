// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <cxx-qt-lib/qhash.h>
#include <cxx-qt-lib/qlist.h>
#include <cxx-qt-lib/qmap.h>
#include <cxx-qt-lib/qset.h>
#include <cxx-qt-lib/qvector.h>

#include <QtCore/QCoreApplication>

static void
do_register_core_types()
{
  qRegisterMetaType<::QHash_i32_QByteArray>("QHash_i32_QByteArray");
  // Ensure that QHash<QString, QVariant> (aka QVariantHash) is registered
  // otherwise it cannot be used in QML
  qRegisterMetaType<::QHash_QString_QVariant>("QHash_QString_QVariant");

  qRegisterMetaType<::QList_bool>("QList_bool");
  qRegisterMetaType<::QList_f32>("QList_f32");
  qRegisterMetaType<::QList_f64>("QList_f64");
  qRegisterMetaType<::QList_i8>("QList_i8");
  qRegisterMetaType<::QList_i16>("QList_i16");
  qRegisterMetaType<::QList_i32>("QList_i32");
  qRegisterMetaType<::QList_i64>("QList_i64");
  qRegisterMetaType<::QList_QByteArray>("QList_QByteArray");
  qRegisterMetaType<::QList_QDate>("QList_QDate");
  qRegisterMetaType<::QList_QDateTime>("QList_QDateTime");
  qRegisterMetaType<::QList_QLine>("QList_QLine");
  qRegisterMetaType<::QList_QLineF>("QList_QLineF");
  qRegisterMetaType<::QList_QMargins>("QList_QMargins");
  qRegisterMetaType<::QList_QMarginsF>("QList_QMarginsF");
  qRegisterMetaType<::QList_QPersistentModelIndex>(
    "QList_QPersistentModelIndex");
  qRegisterMetaType<::QList_QPoint>("QList_QPoint");
  qRegisterMetaType<::QList_QPointF>("QList_QPointF");
  qRegisterMetaType<::QList_QRect>("QList_QRect");
  qRegisterMetaType<::QList_QRectF>("QList_QRectF");
  qRegisterMetaType<::QList_QSize>("QList_QSize");
  qRegisterMetaType<::QList_QSizeF>("QList_QSizeF");
  qRegisterMetaType<::QList_QString>("QList_QString");
  qRegisterMetaType<::QList_QTime>("QList_QTime");
  qRegisterMetaType<::QList_QUrl>("QList_QUrl");
  qRegisterMetaType<::QList_QUuid>("QList_QUuid");
  // Ensure that QList<QVariant> (aka QVariantList) is registered
  // otherwise it cannot be used in QML
  qRegisterMetaType<::QList_QVariant>("QList_QVariant");
  qRegisterMetaType<::QList_u8>("QList_u8");
  qRegisterMetaType<::QList_u16>("QList_u16");
  qRegisterMetaType<::QList_u32>("QList_u32");
  qRegisterMetaType<::QList_u64>("QList_u64");

  // Ensure that QMap<QString, QVariant> (aka QVariantMap) is registered
  // otherwise it cannot be used in QML
  qRegisterMetaType<::QMap_QString_QVariant>("QMap_QString_QVariant");

  qRegisterMetaType<::QSet_bool>("QSet_bool");
  qRegisterMetaType<::QSet_f32>("QSet_f32");
  qRegisterMetaType<::QSet_f64>("QSet_f64");
  qRegisterMetaType<::QSet_i8>("QSet_i8");
  qRegisterMetaType<::QSet_i16>("QSet_i16");
  qRegisterMetaType<::QSet_i32>("QSet_i32");
  qRegisterMetaType<::QSet_i64>("QSet_i64");
  qRegisterMetaType<::QSet_QByteArray>("QSet_QByteArray");
  qRegisterMetaType<::QSet_QDate>("QSet_QDate");
  qRegisterMetaType<::QSet_QDateTime>("QSet_QDateTime");
  qRegisterMetaType<::QSet_QPersistentModelIndex>("QSet_QPersistentModelIndex");
  qRegisterMetaType<::QSet_QString>("QSet_QString");
  qRegisterMetaType<::QSet_QTime>("QSet_QTime");
  qRegisterMetaType<::QSet_QUrl>("QSet_QUrl");
  qRegisterMetaType<::QSet_QUuid>("QSet_QUuid");
  qRegisterMetaType<::QSet_u8>("QSet_u8");
  qRegisterMetaType<::QSet_u16>("QSet_u16");
  qRegisterMetaType<::QSet_u32>("QSet_u32");
  qRegisterMetaType<::QSet_u64>("QSet_u64");

  qRegisterMetaType<::QVector_bool>("QVector_bool");
  qRegisterMetaType<::QVector_f32>("QVector_f32");
  qRegisterMetaType<::QVector_f64>("QVector_f64");
  qRegisterMetaType<::QVector_i8>("QVector_i8");
  qRegisterMetaType<::QVector_i16>("QVector_i16");
  qRegisterMetaType<::QVector_i32>("QVector_i32");
  qRegisterMetaType<::QVector_i64>("QVector_i64");
  qRegisterMetaType<::QVector_QByteArray>("QVector_QByteArray");
  qRegisterMetaType<::QVector_QDate>("QVector_QDate");
  qRegisterMetaType<::QVector_QDateTime>("QVector_QDateTime");
  qRegisterMetaType<::QVector_QLine>("QVector_QLine");
  qRegisterMetaType<::QVector_QLineF>("QVector_QLineF");
  qRegisterMetaType<::QVector_QMargins>("QVector_QMargins");
  qRegisterMetaType<::QVector_QMarginsF>("QVector_QMarginsF");
  qRegisterMetaType<::QVector_QPersistentModelIndex>(
    "QVector_QPersistentModelIndex");
  qRegisterMetaType<::QVector_QPoint>("QVector_QPoint");
  qRegisterMetaType<::QVector_QPointF>("QVector_QPointF");
  qRegisterMetaType<::QVector_QRect>("QVector_QRect");
  qRegisterMetaType<::QVector_QRectF>("QVector_QRectF");
  qRegisterMetaType<::QVector_QSize>("QVector_QSize");
  qRegisterMetaType<::QVector_QSizeF>("QVector_QSizeF");
  qRegisterMetaType<::QVector_QString>("QVector_QString");
  qRegisterMetaType<::QVector_QTime>("QVector_QTime");
  qRegisterMetaType<::QVector_QUrl>("QVector_QUrl");
  qRegisterMetaType<::QVector_QUuid>("QVector_QUuid");
  // Ensure that QVector<QVariant> (aka QVariantList) is registered
  // otherwise it cannot be used in QML
  qRegisterMetaType<::QVector_QVariant>("QVector_QVariant");
  qRegisterMetaType<::QVector_u8>("QVector_u8");
  qRegisterMetaType<::QVector_u16>("QVector_u16");
  qRegisterMetaType<::QVector_u32>("QVector_u32");
  qRegisterMetaType<::QVector_u64>("QVector_u64");
}

// Use Q_COREAPP_STARTUP_FUNCTION to defer registration until QCoreApplication
// is created. This is Qt's recommended approach for type registration and
// ensures proper initialization order on all platforms.
Q_COREAPP_STARTUP_FUNCTION(do_register_core_types)

extern "C" bool
init_cxx_qt_lib_core()
{
  // Registration is handled automatically via Q_COREAPP_STARTUP_FUNCTION
  // when QCoreApplication is constructed.
  return true;
}
