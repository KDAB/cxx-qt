// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <QtCore/QDebug>

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "bridge.h"
#include "ffi.cxx.h"

TEST_CASE("Can construct a QString on the Rust side")
{
  // From a slice
  CHECK(can_construct_qstring(true));

  // From a rust::String
  CHECK(can_construct_qstring(false));
}

TEST_CASE("Can read a QString on the Rust side")
{
  const auto s = QStringLiteral("String constructed by C++");
  CHECK(can_read_qstring(s));
}

TEST_CASE("Can modify a QString on the Rust side")
{
  auto s = QStringLiteral("String constructed by C++");
  modify_qstring(s);
  CHECK_EQ(s, QStringLiteral("Updated string value"));
}

TEST_CASE("Can handle a QString modified on the Rust side")
{
  CHECK(can_handle_qstring_change());
}

bool
test_constructed_qstring(const QString& s)
{
  return s == QStringLiteral("String constructed by Rust");
}

void
assign_to_qstring(QString& s, const QString& v)
{
  s = v;

  // Force some more activity for valgrind to inspect
  s.detach();
}

TEST_CASE("Can construct a QColor on the Rust side")
{
  CHECK(can_construct_qcolor(ColorTest::Rgb_Red));
  CHECK(can_construct_qcolor(ColorTest::Rgb_Green));
  CHECK(can_construct_qcolor(ColorTest::Rgb_Blue));
  CHECK(can_construct_qcolor(ColorTest::Rgb_Transparent));
}

bool
test_constructed_qcolor(const QColor& c, ColorTest test)
{
  switch (test) {
    case ColorTest::Rgb_Red:
      return c.alpha() == 255 && c.red() == 255 && c.green() == 0 &&
             c.blue() == 0;
    case ColorTest::Rgb_Green:
      return c.alpha() == 255 && c.red() == 0 && c.green() == 255 &&
             c.blue() == 0;
    case ColorTest::Rgb_Blue:
      return c.alpha() == 255 && c.red() == 0 && c.green() == 0 &&
             c.blue() == 255;
    case ColorTest::Rgb_Transparent:
      return c.alpha() == 0 && c.red() == 0 && c.green() == 0 && c.blue() == 0;
    default:
      return false;
  }
}

TEST_CASE("Can convert Rust Color to QColor")
{
  const auto runTest = [](auto test) {
    return test_constructed_qcolor(std::move(*make_color(test)), test);
  };

  CHECK(runTest(ColorTest::Rgb_Red));
  CHECK(runTest(ColorTest::Rgb_Green));
  CHECK(runTest(ColorTest::Rgb_Blue));
  CHECK(runTest(ColorTest::Rgb_Transparent));
}

TEST_CASE("Can read a QColor on the Rust side")
{
  CHECK(can_read_qcolor(QColor(255, 0, 0, 255), ColorTest::Rgb_Red));
  CHECK(can_read_qcolor(QColor(0, 255, 0, 255), ColorTest::Rgb_Green));
  CHECK(can_read_qcolor(QColor(0, 0, 255, 255), ColorTest::Rgb_Blue));
  CHECK(can_read_qcolor(QColor(0, 0, 0, 0), ColorTest::Rgb_Transparent));

  CHECK(can_read_qcolor(QColor(Qt::red), ColorTest::Rgb_Red));
  CHECK(can_read_qcolor(QColor(Qt::green), ColorTest::Rgb_Green));
  CHECK(can_read_qcolor(QColor(Qt::blue), ColorTest::Rgb_Blue));
  CHECK(can_read_qcolor(QColor(Qt::transparent), ColorTest::Rgb_Transparent));
}

TEST_CASE("Can construct a QDateTime on the Rust side")
{
  CHECK(can_construct_qdatetime(QDate(2022, 1, 1), QTime(1, 2, 3, 4)));
}

bool
test_constructed_qdatetime(const QDateTime& dt,
                           const QDate& date,
                           const QTime& time)
{
  return dt.date().year() == 2022 && dt.date().month() == 1 &&
         dt.date().day() == 1 && dt.time().hour() == 1 &&
         dt.time().minute() == 2 && dt.time().second() == 3 &&
         dt.time().msec() == 4;
}

TEST_CASE("Can read a QDateTime on the Rust side")
{
  CHECK(can_read_qdatetime(QDateTime(QDate(2022, 1, 1), QTime(1, 2, 3, 4)),
                           QDate(2022, 1, 1),
                           QTime(1, 2, 3, 4)));
}

TEST_CASE("Can construct a QUrl on the Rust side")
{
  CHECK(can_construct_qurl(QStringLiteral("https://kdab.com/")));
  CHECK(can_construct_qurl(QStringLiteral("https://github.com/KDAB/cxx-qt/")));
}

bool
test_constructed_qurl(const QUrl& c, const QString& test)
{
  return c.url() == test;
}

TEST_CASE("Can read a QUrl on the Rust side")
{
  CHECK(can_read_qurl(QUrl(QStringLiteral("https://github.com/KDAB/cxx-qt/")),
                      QStringLiteral("https://github.com/KDAB/cxx-qt/")));
  CHECK(can_read_qurl(QUrl(QStringLiteral("https://kdab.com")),
                      QStringLiteral("https://kdab.com")));
}

TEST_CASE("Can construct a QVariant on the Rust side")
{
  CHECK(can_construct_qvariant(VariantTest::Bool));
  CHECK(can_construct_qvariant(VariantTest::F32));
  CHECK(can_construct_qvariant(VariantTest::F64));
  CHECK(can_construct_qvariant(VariantTest::I8));
  CHECK(can_construct_qvariant(VariantTest::I16));
  CHECK(can_construct_qvariant(VariantTest::I32));
  CHECK(can_construct_qvariant(VariantTest::QColor));
  CHECK(can_construct_qvariant(VariantTest::QDate));
  CHECK(can_construct_qvariant(VariantTest::QDateTime));
  CHECK(can_construct_qvariant(VariantTest::QPoint));
  CHECK(can_construct_qvariant(VariantTest::QPointF));
  CHECK(can_construct_qvariant(VariantTest::QRect));
  CHECK(can_construct_qvariant(VariantTest::QRectF));
  CHECK(can_construct_qvariant(VariantTest::QSize));
  CHECK(can_construct_qvariant(VariantTest::QSizeF));
  CHECK(can_construct_qvariant(VariantTest::QTime));
  CHECK(can_construct_qvariant(VariantTest::QUrl));
  CHECK(can_construct_qvariant(VariantTest::String));
  CHECK(can_construct_qvariant(VariantTest::U8));
  CHECK(can_construct_qvariant(VariantTest::U16));
  CHECK(can_construct_qvariant(VariantTest::U32));
}

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
    case VariantTest::QTime:
      return v.value<QTime>().hour() == 1 && v.value<QTime>().minute() == 2 &&
             v.value<QTime>().second() == 3 && v.value<QTime>().msec() == 4;
    case VariantTest::QUrl:
      return v.value<QUrl>().toString() ==
             QStringLiteral("https://github.com/KDAB");
    case VariantTest::String:
      return v.toString() == QStringLiteral("Rust string");
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

TEST_CASE("Can convert Rust Variant to QVariant")
{
  const auto runTest = [](auto test) {
    return test_constructed_qvariant(std::move(*make_variant(test)), test);
  };

  CHECK(runTest(VariantTest::Bool));
  CHECK(runTest(VariantTest::F32));
  CHECK(runTest(VariantTest::F64));
  CHECK(runTest(VariantTest::I8));
  CHECK(runTest(VariantTest::I16));
  CHECK(runTest(VariantTest::I32));
  CHECK(runTest(VariantTest::QColor));
  CHECK(runTest(VariantTest::QDate));
  CHECK(runTest(VariantTest::QDateTime));
  CHECK(runTest(VariantTest::QPoint));
  CHECK(runTest(VariantTest::QPointF));
  CHECK(runTest(VariantTest::QRect));
  CHECK(runTest(VariantTest::QRectF));
  CHECK(runTest(VariantTest::QSize));
  CHECK(runTest(VariantTest::QSizeF));
  CHECK(runTest(VariantTest::QTime));
  CHECK(runTest(VariantTest::QUrl));
  CHECK(runTest(VariantTest::String));
  CHECK(runTest(VariantTest::U8));
  CHECK(runTest(VariantTest::U16));
  CHECK(runTest(VariantTest::U32));
}

TEST_CASE("Can read a QVariant on the Rust side")
{
  CHECK(can_read_qvariant(QVariant::fromValue(false), VariantTest::Bool));
  CHECK(can_read_qvariant(QVariant::fromValue<float>(89.1), VariantTest::F32));
  CHECK(can_read_qvariant(QVariant::fromValue<double>(89.1), VariantTest::F64));
  CHECK(can_read_qvariant(QVariant::fromValue<qint8>(89), VariantTest::I8));
  CHECK(can_read_qvariant(QVariant::fromValue<qint8>(89), VariantTest::I8));
  CHECK(can_read_qvariant(QVariant::fromValue<qint16>(8910), VariantTest::I16));
  CHECK(can_read_qvariant(QVariant::fromValue(8910), VariantTest::I32));
  CHECK(can_read_qvariant(QVariant::fromValue<QColor>(QColor(0, 255, 0, 255)),
                          VariantTest::QColor));
  CHECK(can_read_qvariant(QVariant::fromValue<QDate>(QDate(2021, 12, 31)),
                          VariantTest::QDate));
  CHECK(can_read_qvariant(QVariant::fromValue<QDateTime>(
                            QDateTime(QDate(2021, 12, 31), QTime(4, 3, 2, 1))),
                          VariantTest::QDateTime));
  CHECK(can_read_qvariant(QVariant::fromValue<QPoint>(QPoint(8, 9)),
                          VariantTest::QPoint));
  CHECK(can_read_qvariant(QVariant::fromValue<QPointF>(QPointF(8.0, 9.0)),
                          VariantTest::QPointF));
  CHECK(can_read_qvariant(QVariant::fromValue<QRect>(QRect(123, 456, 246, 912)),
                          VariantTest::QRect));
  CHECK(can_read_qvariant(
    QVariant::fromValue<QRectF>(QRectF(1.23, 4.56, 2.46, 9.12)),
    VariantTest::QRectF));
  CHECK(can_read_qvariant(QVariant::fromValue<QSize>(QSize(8, 9)),
                          VariantTest::QSize));
  CHECK(can_read_qvariant(QVariant::fromValue<QSizeF>(QSizeF(8.0, 9.0)),
                          VariantTest::QSizeF));
  CHECK(can_read_qvariant(QVariant::fromValue<QTime>(QTime(4, 3, 2, 1)),
                          VariantTest::QTime));
  CHECK(can_read_qvariant(QVariant::fromValue<QUrl>(QUrl(
                            QStringLiteral("https://github.com/KDAB/cxx-qt"))),
                          VariantTest::QUrl));
  CHECK(can_read_qvariant(QVariant::fromValue(QStringLiteral("C++ string")),
                          VariantTest::String));
  CHECK(can_read_qvariant(QVariant::fromValue<quint8>(89), VariantTest::U8));
  CHECK(
    can_read_qvariant(QVariant::fromValue<quint16>(8910), VariantTest::U16));
  CHECK(
    can_read_qvariant(QVariant::fromValue<quint32>(8910), VariantTest::U32));
}

TEST_CASE("Can construct a QPoint on the Rust side")
{
  const auto p = construct_qpoint();
  CHECK(p.x() == 2);
  CHECK(p.y() == 4);
}

TEST_CASE("Can read a QPoint on the Rust side")
{
  const auto p = QPoint(2, 4);
  CHECK(read_qpoint(p));
}

TEST_CASE("Can copy a QPoint on the Rust side")
{
  const auto p = QPoint(2, 4);
  const auto c = copy_qpoint(p);
  CHECK(c.x() == 2);
  CHECK(c.y() == 4);
}

TEST_CASE("Can copy a value QPoint on the Rust side")
{
  const auto p = QPoint(2, 4);
  const auto c = copy_value_qpoint(p);
  CHECK(c.x() == 2);
  CHECK(c.y() == 4);
}

TEST_CASE("Can construct a QDate on the Rust side")
{
  const auto d = construct_qdate();
  CHECK(d.year() == 2022);
  CHECK(d.month() == 1);
  CHECK(d.day() == 1);
}

TEST_CASE("Can read a QDate on the Rust side")
{
  const auto d = QDate(2022, 1, 1);
  CHECK(read_qdate(d));
}

TEST_CASE("Can copy a QDate on the Rust side")
{
  const auto d = QDate(2022, 1, 1);
  const auto c = copy_qdate(d);
  CHECK(c.year() == 2022);
  CHECK(c.month() == 1);
  CHECK(c.day() == 1);
}

TEST_CASE("Can copy a value QDate on the Rust side")
{
  const auto d = QDate(2022, 1, 1);
  const auto c = copy_value_qdate(d);
  CHECK(c.year() == 2022);
  CHECK(c.month() == 1);
  CHECK(c.day() == 1);
}

TEST_CASE("Can construct a QPointF on the Rust side")
{
  const auto p = construct_qpointf();
  CHECK(qFuzzyCompare(p.x(), 1.23));
  CHECK(qFuzzyCompare(p.y(), 4.56));
}

TEST_CASE("Can read a QPointF on the Rust side")
{
  const auto p = QPointF(1.23, 4.56);
  CHECK(read_qpointf(p));
}

TEST_CASE("Can copy a QPointF on the Rust side")
{
  const auto p = QPointF(1.23, 4.56);
  const auto c = copy_qpointf(p);
  CHECK(qFuzzyCompare(c.x(), 1.23));
  CHECK(qFuzzyCompare(c.y(), 4.56));
}

TEST_CASE("Can copy a value QPointF on the Rust side")
{
  const auto p = QPointF(1.23, 4.56);
  const auto c = copy_value_qpointf(p);
  CHECK(qFuzzyCompare(c.x(), 1.23));
  CHECK(qFuzzyCompare(c.y(), 4.56));
}

TEST_CASE("Can construct a QRect on the Rust side")
{
  const auto r = construct_qrect();
  CHECK(r.x() == 1);
  CHECK(r.y() == 4);
  CHECK(r.width() == 2);
  CHECK(r.height() == 8);
}

TEST_CASE("Can read a QRect on the Rust side")
{
  const auto r = QRect(1, 4, 2, 8);
  CHECK(read_qrect(r));
}

TEST_CASE("Can copy a QRect on the Rust side")
{
  const auto r = QRect(1, 4, 2, 8);
  const auto c = copy_qrect(r);
  CHECK(c.x() == 1);
  CHECK(c.y() == 4);
  CHECK(c.width() == 2);
  CHECK(c.height() == 8);
}

TEST_CASE("Can copy a value QRect on the Rust side")
{
  const auto r = QRect(1, 4, 2, 8);
  const auto c = copy_value_qrect(r);
  CHECK(c.x() == 1);
  CHECK(c.y() == 4);
  CHECK(c.width() == 2);
  CHECK(c.height() == 8);
}

TEST_CASE("Can construct a QRectF on the Rust side")
{
  const auto r = construct_qrectf();
  CHECK(qFuzzyCompare(r.x(), 1.23));
  CHECK(qFuzzyCompare(r.y(), 4.56));
  CHECK(qFuzzyCompare(r.width(), 2.46));
  CHECK(qFuzzyCompare(r.height(), 9.12));
}

TEST_CASE("Can read a QRectF on the Rust side")
{
  const auto r = QRectF(1.23, 4.56, 2.46, 9.12);
  CHECK(read_qrectf(r));
}

TEST_CASE("Can copy a QRectF on the Rust side")
{
  const auto r = QRectF(1.23, 4.56, 2.46, 9.12);
  const auto c = copy_qrectf(r);
  CHECK(qFuzzyCompare(c.x(), 1.23));
  CHECK(qFuzzyCompare(c.y(), 4.56));
  CHECK(qFuzzyCompare(c.width(), 2.46));
  CHECK(qFuzzyCompare(c.height(), 9.12));
}

TEST_CASE("Can copy a value QRectF on the Rust side")
{
  const auto r = QRectF(1.23, 4.56, 2.46, 9.12);
  const auto c = copy_value_qrectf(r);
  CHECK(qFuzzyCompare(c.x(), 1.23));
  CHECK(qFuzzyCompare(c.y(), 4.56));
  CHECK(qFuzzyCompare(c.width(), 2.46));
  CHECK(qFuzzyCompare(c.height(), 9.12));
}

TEST_CASE("Can construct a QSize on the Rust side")
{
  const auto s = construct_qsize();
  CHECK(s.width() == 1);
  CHECK(s.height() == 4);
}

TEST_CASE("Can read a QSize on the Rust side")
{
  const auto s = QSize(1, 4);
  CHECK(read_qsize(s));
}

TEST_CASE("Can copy a QSize on the Rust side")
{
  const auto s = QSize(1, 4);
  const auto c = copy_qsize(s);
  CHECK(c.width() == 1);
  CHECK(c.height() == 4);
}

TEST_CASE("Can copy a value QSize on the Rust side")
{
  const auto s = QSize(1, 4);
  const auto c = copy_value_qsize(s);
  CHECK(c.width() == 1);
  CHECK(c.height() == 4);
}

TEST_CASE("Can construct a QSizeF on the Rust side")
{
  const auto s = construct_qsizef();
  CHECK(qFuzzyCompare(s.width(), 1.23));
  CHECK(qFuzzyCompare(s.height(), 4.56));
}

TEST_CASE("Can read a QSizeF on the Rust side")
{
  const auto s = QSizeF(1.23, 4.56);
  CHECK(read_qsizef(s));
}

TEST_CASE("Can copy a QSizeF on the Rust side")
{
  const auto s = QSizeF(1.23, 4.56);
  const auto c = copy_qsizef(s);
  CHECK(qFuzzyCompare(c.width(), 1.23));
  CHECK(qFuzzyCompare(c.height(), 4.56));
}

TEST_CASE("Can copy a value QSizeF on the Rust side")
{
  const auto s = QSizeF(1.23, 4.56);
  const auto c = copy_value_qsizef(s);
  CHECK(qFuzzyCompare(c.width(), 1.23));
  CHECK(qFuzzyCompare(c.height(), 4.56));
}

TEST_CASE("Can construct a QTime on the Rust side")
{
  const auto t = construct_qtime();
  CHECK(t.hour() == 1);
  CHECK(t.minute() == 2);
  CHECK(t.second() == 3);
  CHECK(t.msec() == 4);
}

TEST_CASE("Can read a QTime on the Rust side")
{
  const auto t = QTime(1, 2, 3, 4);
  CHECK(read_qtime(t));
}

TEST_CASE("Can copy a QTime on the Rust side")
{
  const auto t = QTime(1, 2, 3, 4);
  const auto c = copy_qtime(t);
  CHECK(t.hour() == 1);
  CHECK(t.minute() == 2);
  CHECK(t.second() == 3);
  CHECK(t.msec() == 4);
}

TEST_CASE("Can copy a value QTime on the Rust side")
{
  const auto t = QTime(1, 2, 3, 4);
  const auto c = copy_value_qtime(t);
  CHECK(t.hour() == 1);
  CHECK(t.minute() == 2);
  CHECK(t.second() == 3);
  CHECK(t.msec() == 4);
}
