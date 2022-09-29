// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QVariant>
#include <QtTest/QTest>

#include "cxx-qt-gen/include/qvariant_cxx.cxx.h"

// We use VariantTest in data driven tests, so register to Qt metatype system
Q_DECLARE_METATYPE(VariantTest)

namespace {

bool
test_constructed_qvariant(const QVariant& v, VariantTest test)
{
  switch (test) {
    case VariantTest::Bool:
      return v.toBool() == true;
    case VariantTest::F32:
      return qFuzzyCompare(v.value<float>(), 1.23f);
    case VariantTest::F64:
      return qFuzzyCompare(v.value<double>(), 1.23);
    case VariantTest::I8:
      return v.value<qint8>() == 12;
    case VariantTest::I16:
      return v.value<qint16>() == 123;
    case VariantTest::I32:
      return v.value<qint32>() == 123;
    case VariantTest::QColor:
      return v.value<QColor>().alpha() == 255 &&
             v.value<QColor>().red() == 255 && v.value<QColor>().green() == 0 &&
             v.value<QColor>().blue() == 0;
    case VariantTest::QDate:
      return v.value<QDate>().year() == 2022 && v.value<QDate>().month() == 1 &&
             v.value<QDate>().day() == 1;
    case VariantTest::QDateTime:
      return v.value<QDateTime>().date().year() == 2022 &&
             v.value<QDateTime>().date().month() == 1 &&
             v.value<QDateTime>().date().day() == 1 &&
             v.value<QDateTime>().time().hour() == 1 &&
             v.value<QDateTime>().time().minute() == 2 &&
             v.value<QDateTime>().time().second() == 3 &&
             v.value<QDateTime>().time().msec() == 4;
    case VariantTest::QPoint:
      return v.value<QPoint>().x() == 1 && v.value<QPoint>().y() == 3;
    case VariantTest::QPointF:
      return v.value<QPointF>().x() == 1.0 && v.value<QPoint>().y() == 3.0;
    case VariantTest::QRect:
      return v.value<QRectF>().x() == 123 && v.value<QRectF>().y() == 456 &&
             v.value<QRectF>().width() == 246 &&
             v.value<QRectF>().height() == 912;
    case VariantTest::QRectF:
      return qFuzzyCompare(v.value<QRectF>().x(), 1.23) &&
             qFuzzyCompare(v.value<QRectF>().y(), 4.56) &&
             qFuzzyCompare(v.value<QRectF>().width(), 2.46) &&
             qFuzzyCompare(v.value<QRectF>().height(), 9.12);
    case VariantTest::QSize:
      return v.value<QSize>().width() == 1 && v.value<QSize>().height() == 3;
    case VariantTest::QSizeF:
      return v.value<QSizeF>().width() == 1.0 &&
             v.value<QSize>().height() == 3.0;
    case VariantTest::QString:
      return v.toString() == QStringLiteral("Rust string");
    case VariantTest::QTime:
      return v.value<QTime>().hour() == 1 && v.value<QTime>().minute() == 2 &&
             v.value<QTime>().second() == 3 && v.value<QTime>().msec() == 4;
    case VariantTest::QUrl:
      return v.value<QUrl>().toString() ==
             QStringLiteral("https://github.com/KDAB");
    case VariantTest::U8:
      return v.value<quint8>() == 12;
    case VariantTest::U16:
      return v.value<quint16>() == 123;
    case VariantTest::U32:
      return v.value<quint32>() == 123;

    default:
      return false;
  }
}

}

class QVariantTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    QFETCH(VariantTest, variantTest);
    QVERIFY(
      test_constructed_qvariant(construct_qvariant(variantTest), variantTest));
  }

  void construct_data()
  {
    QTest::addColumn<VariantTest>("variantTest");

    QTest::newRow("Bool") << VariantTest::Bool;
    QTest::newRow("F32") << VariantTest::F32;
    QTest::newRow("F64") << VariantTest::F64;
    QTest::newRow("I8") << VariantTest::I8;
    QTest::newRow("I16") << VariantTest::I16;
    QTest::newRow("I32") << VariantTest::I32;
    QTest::newRow("QColor") << VariantTest::QColor;
    QTest::newRow("QDate") << VariantTest::QDate;
    QTest::newRow("QDateTime") << VariantTest::QDateTime;
    QTest::newRow("QPoint") << VariantTest::QPoint;
    QTest::newRow("QPointF") << VariantTest::QPointF;
    QTest::newRow("QRect") << VariantTest::QRect;
    QTest::newRow("QRectF") << VariantTest::QRectF;
    QTest::newRow("QSize") << VariantTest::QSize;
    QTest::newRow("QSizeF") << VariantTest::QSizeF;
    QTest::newRow("QString") << VariantTest::QString;
    QTest::newRow("QTime") << VariantTest::QTime;
    QTest::newRow("QUrl") << VariantTest::QUrl;
    QTest::newRow("U8") << VariantTest::U8;
    QTest::newRow("U16") << VariantTest::U16;
    QTest::newRow("U32") << VariantTest::U32;
  }

  void read()
  {
    QFETCH(QVariant, variant);
    QFETCH(VariantTest, variantTest);

    QVERIFY(read_qvariant(variant, variantTest));
  }

  void read_data()
  {
    QTest::addColumn<QVariant>("variant");
    QTest::addColumn<VariantTest>("variantTest");

    QTest::newRow("Bool") << QVariant::fromValue(false) << VariantTest::Bool;
    QTest::newRow("F32") << QVariant::fromValue<float>(89.1)
                         << VariantTest::F32;
    QTest::newRow("F64") << QVariant::fromValue<double>(89.1)
                         << VariantTest::F64;
    QTest::newRow("I8") << QVariant::fromValue<qint8>(89) << VariantTest::I8;
    QTest::newRow("I16") << QVariant::fromValue<qint16>(8910)
                         << VariantTest::I16;
    QTest::newRow("I32") << QVariant::fromValue(8910) << VariantTest::I32;
    QTest::newRow("QColor")
      << QVariant::fromValue<QColor>(QColor(0, 255, 0, 255))
      << VariantTest::QColor;
    QTest::newRow("QDate") << QVariant::fromValue<QDate>(QDate(2021, 12, 31))
                           << VariantTest::QDate;
    QTest::newRow("QDateTime") << QVariant::fromValue<QDateTime>(QDateTime(
                                    QDate(2021, 12, 31), QTime(4, 3, 2, 1)))
                               << VariantTest::QDateTime;
    QTest::newRow("QPoint")
      << QVariant::fromValue<QPoint>(QPoint(8, 9)) << VariantTest::QPoint;
    QTest::newRow("QPointF") << QVariant::fromValue<QPointF>(QPointF(8.0, 9.0))
                             << VariantTest::QPointF;
    QTest::newRow("QRect") << QVariant::fromValue<QRect>(
                                QRect(123, 456, 246, 912))
                           << VariantTest::QRect;
    QTest::newRow("QRectF")
      << QVariant::fromValue<QRectF>(QRectF(1.23, 4.56, 2.46, 9.12))
      << VariantTest::QRectF;
    QTest::newRow("QSize") << QVariant::fromValue<QSize>(QSize(8, 9))
                           << VariantTest::QSize;
    QTest::newRow("QSizeF")
      << QVariant::fromValue<QSizeF>(QSizeF(8.0, 9.0)) << VariantTest::QSizeF;
    QTest::newRow("QString")
      << QVariant::fromValue(QStringLiteral("C++ string"))
      << VariantTest::QString;
    QTest::newRow("QTime") << QVariant::fromValue<QTime>(QTime(4, 3, 2, 1))
                           << VariantTest::QTime;
    QTest::newRow("QUrl") << QVariant::fromValue<QUrl>(QUrl(QStringLiteral(
                               "https://github.com/KDAB/cxx-qt")))
                          << VariantTest::QUrl;
    QTest::newRow("U8") << QVariant::fromValue<quint8>(89) << VariantTest::U8;
    QTest::newRow("U16") << QVariant::fromValue<quint16>(8910)
                         << VariantTest::U16;
    QTest::newRow("U32") << QVariant::fromValue<quint32>(8910)
                         << VariantTest::U32;
  }

  void clone()
  {
    const auto v = QVariant::fromValue<QPoint>(QPoint(8, 9));
    const auto c = clone_qvariant(v);
    QCOMPARE(c.toPoint().x(), 8);
    QCOMPARE(c.toPoint().y(), 9);
  }
};
