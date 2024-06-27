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

static const int register_QHash_i32_QByteArray =
  qRegisterMetaType<::QHash_i32_QByteArray>("QHash_i32_QByteArray");
// Ensure that QHash<QString, QVariant> (aka QVariantHash) is registered
// otherwise it cannot be used in QML
static const int register_QHash_QString_QVariant =
  qRegisterMetaType<::QHash_QString_QVariant>("QHash_QString_QVariant");

static const int register_QList_bool =
  qRegisterMetaType<::QList_bool>("QList_bool");
static const int register_QList_f32 =
  qRegisterMetaType<::QList_f32>("QList_f32");
static const int register_QList_f64 =
  qRegisterMetaType<::QList_f64>("QList_f64");
static const int register_QList_i8 = qRegisterMetaType<::QList_i8>("QList_i8");
static const int register_QList_i16 =
  qRegisterMetaType<::QList_i16>("QList_i16");
static const int register_QList_i32 =
  qRegisterMetaType<::QList_i32>("QList_i32");
static const int register_QList_i64 =
  qRegisterMetaType<::QList_i64>("QList_i64");
static const int register_QList_QByteArray =
  qRegisterMetaType<::QList_QByteArray>("QList_QByteArray");
static const int register_QList_QDate =
  qRegisterMetaType<::QList_QDate>("QList_QDate");
static const int register_QList_QDateTime =
  qRegisterMetaType<::QList_QDateTime>("QList_QDateTime");
static const int register_QList_QMargins =
  qRegisterMetaType<::QList_QMargins>("QList_QMargins");
static const int register_QList_QMarginsF =
  qRegisterMetaType<::QList_QMarginsF>("QList_QMarginsF");
static const int register_QList_QPersistentModelIndex =
  qRegisterMetaType<::QList_QPersistentModelIndex>(
    "QList_QPersistentModelIndex");
static const int register_QList_QPoint =
  qRegisterMetaType<::QList_QPoint>("QList_QPoint");
static const int register_QList_QPointF =
  qRegisterMetaType<::QList_QPointF>("QList_QPointF");
static const int register_QList_QRect =
  qRegisterMetaType<::QList_QRect>("QList_QRect");
static const int register_QList_QRectF =
  qRegisterMetaType<::QList_QRectF>("QList_QRectF");
static const int register_QList_QSize =
  qRegisterMetaType<::QList_QSize>("QList_QSize");
static const int register_QList_QSizeF =
  qRegisterMetaType<::QList_QSizeF>("QList_QSizeF");
static const int register_QList_QString =
  qRegisterMetaType<::QList_QString>("QList_QString");
static const int register_QList_QTime =
  qRegisterMetaType<::QList_QTime>("QList_QTime");
static const int register_QList_QUrl =
  qRegisterMetaType<::QList_QUrl>("QList_QUrl");
// Ensure that QList<QVariant> (aka QVariantList) is registered
// otherwise it cannot be used in QML
static const int register_QList_QVariant =
  qRegisterMetaType<::QList_QVariant>("QList_QVariant");
static const int register_QList_u8 = qRegisterMetaType<::QList_u8>("QList_u8");
static const int register_QList_u16 =
  qRegisterMetaType<::QList_u16>("QList_u16");
static const int register_QList_u32 =
  qRegisterMetaType<::QList_u32>("QList_u32");
static const int register_QList_u64 =
  qRegisterMetaType<::QList_u64>("QList_u64");

// Ensure that QMap<QString, QVariant> (aka QVariantMap) is registered
// otherwise it cannot be used in QML
static const int register_QMap_QString_QVariant =
  qRegisterMetaType<::QMap_QString_QVariant>("QMap_QString_QVariant");

static const int register_QSet_bool =
  qRegisterMetaType<::QSet_bool>("QSet_bool");
static const int register_QSet_f32 = qRegisterMetaType<::QSet_f32>("QSet_f32");
static const int register_QSet_f64 = qRegisterMetaType<::QSet_f64>("QSet_f64");
static const int register_QSet_i8 = qRegisterMetaType<::QSet_i8>("QSet_i8");
static const int register_QSet_i16 = qRegisterMetaType<::QSet_i16>("QSet_i16");
static const int register_QSet_i32 = qRegisterMetaType<::QSet_i32>("QSet_i32");
static const int register_QSet_i64 = qRegisterMetaType<::QSet_i64>("QSet_i64");
static const int register_QSet_QByteArray =
  qRegisterMetaType<::QSet_QByteArray>("QSet_QByteArray");
static const int register_QSet_QDate =
  qRegisterMetaType<::QSet_QDate>("QSet_QDate");
static const int register_QSet_QDateTime =
  qRegisterMetaType<::QSet_QDateTime>("QSet_QDateTime");
static const int register_QSet_QPersistentModelIndex =
  qRegisterMetaType<::QSet_QPersistentModelIndex>("QSet_QPersistentModelIndex");
static const int register_QSet_QString =
  qRegisterMetaType<::QSet_QString>("QSet_QString");
static const int register_QSet_QTime =
  qRegisterMetaType<::QSet_QTime>("QSet_QTime");
static const int register_QSet_QUrl =
  qRegisterMetaType<::QSet_QUrl>("QSet_QUrl");
static const int register_QSet_u8 = qRegisterMetaType<::QSet_u8>("QSet_u8");
static const int register_QSet_u16 = qRegisterMetaType<::QSet_u16>("QSet_u16");
static const int register_QSet_u32 = qRegisterMetaType<::QSet_u32>("QSet_u32");
static const int register_QSet_u64 = qRegisterMetaType<::QSet_u64>("QSet_u64");

static const int register_QVector_bool =
  qRegisterMetaType<::QVector_bool>("QVector_bool");
static const int register_QVector_f32 =
  qRegisterMetaType<::QVector_f32>("QVector_f32");
static const int register_QVector_f64 =
  qRegisterMetaType<::QVector_f64>("QVector_f64");
static const int register_QVector_i8 =
  qRegisterMetaType<::QVector_i8>("QVector_i8");
static const int register_QVector_i16 =
  qRegisterMetaType<::QVector_i16>("QVector_i16");
static const int register_QVector_i32 =
  qRegisterMetaType<::QVector_i32>("QVector_i32");
static const int register_QVector_i64 =
  qRegisterMetaType<::QVector_i64>("QVector_i64");
static const int register_QVector_QByteArray =
  qRegisterMetaType<::QVector_QByteArray>("QVector_QByteArray");
static const int register_QVector_QDate =
  qRegisterMetaType<::QVector_QDate>("QVector_QDate");
static const int register_QVector_QDateTime =
  qRegisterMetaType<::QVector_QDateTime>("QVector_QDateTime");
static const int register_QVector_QMargins =
  qRegisterMetaType<::QVector_QMargins>("QVector_QMargins");
static const int register_QVector_QMarginsF =
  qRegisterMetaType<::QVector_QMarginsF>("QVector_QMarginsF");
static const int register_QVector_QPersistentModelIndex =
  qRegisterMetaType<::QVector_QPersistentModelIndex>(
    "QVector_QPersistentModelIndex");
static const int register_QVector_QPoint =
  qRegisterMetaType<::QVector_QPoint>("QVector_QPoint");
static const int register_QVector_QPointF =
  qRegisterMetaType<::QVector_QPointF>("QVector_QPointF");
static const int register_QVector_QRect =
  qRegisterMetaType<::QVector_QRect>("QVector_QRect");
static const int register_QVector_QRectF =
  qRegisterMetaType<::QVector_QRectF>("QVector_QRectF");
static const int register_QVector_QSize =
  qRegisterMetaType<::QVector_QSize>("QVector_QSize");
static const int register_QVector_QSizeF =
  qRegisterMetaType<::QVector_QSizeF>("QVector_QSizeF");
static const int register_QVector_QString =
  qRegisterMetaType<::QVector_QString>("QVector_QString");
static const int register_QVector_QTime =
  qRegisterMetaType<::QVector_QTime>("QVector_QTime");
static const int register_QVector_QUrl =
  qRegisterMetaType<::QVector_QUrl>("QVector_QUrl");
// Ensure that QVector<QVariant> (aka QVariantList) is registered
// otherwise it cannot be used in QML
static const int register_QVector_QVariant =
  qRegisterMetaType<::QVector_QVariant>("QVector_QVariant");
static const int register_QVector_u8 =
  qRegisterMetaType<::QVector_u8>("QVector_u8");
static const int register_QVector_u16 =
  qRegisterMetaType<::QVector_u16>("QVector_u16");
static const int register_QVector_u32 =
  qRegisterMetaType<::QVector_u32>("QVector_u32");
static const int register_QVector_u64 =
  qRegisterMetaType<::QVector_u64>("QVector_u64");
