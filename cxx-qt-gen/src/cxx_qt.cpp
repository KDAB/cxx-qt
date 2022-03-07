// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#ifndef NO_QT

#include "rust/cxx_qt.h"

#include <QDate>
#include <QDateTime>
#include <QMetaObject>
#include <QPointF>
#include <QPointer>
#include <QRect>
#include <QRectF>
#include <QSize>
#include <QSizeF>
#include <QTime>
#include <QUrl>
#include <QVariant>
#include <QtGui/QColor>

// Define macros which allow us to disable warnings from the compiler
// this is used for disabling -Wreturn-type-c-linkage
#if defined(_MSC_VER)
#define DISABLE_WARNING_PUSH __pragma(warning(push))
#define DISABLE_WARNING_POP __pragma(warning(pop))
#define DISABLE_WARNING(warningNumber)                                         \
  __pragma(warning(disable : warningNumber))

#define DISABLE_RETURN_TYPE_C_LINKAGE DISABLE_WARNING(4190)
#elif defined(__GNUC__) || defined(__clang__)
#define DO_PRAGMA(X) _Pragma(#X)
#define DISABLE_WARNING_PUSH DO_PRAGMA(GCC diagnostic push)
#define DISABLE_WARNING_POP DO_PRAGMA(GCC diagnostic pop)
#define DISABLE_WARNING(warningName)                                           \
  DO_PRAGMA(GCC diagnostic ignored #warningName)

#if defined(__clang__)
// clang-format off
#define DISABLE_RETURN_TYPE_C_LINKAGE                                          \
  DISABLE_WARNING(-Wreturn-type-c-linkage)
// clang-format on
#else
#define DISABLE_RETURN_TYPE_C_LINKAGE
#endif

#else
#define DISABLE_WARNING_PUSH
#define DISABLE_WARNING_POP
#define DISABLE_RETURN_TYPE_C_LINKAGE
#endif

// UpdateRequester is simply a wrapper around QPtr which allows for Rust code to
// post an event to a specific CxxQObject.
//
// We use QPointer as this allows us to detect when our pointer no longer points
// to a valid QObject as a result of the object having been deleted by C++ code
// for whatever reason.
//
// As Rust does not understand how QPointer works, we give it a QPtr* and
// provide the below C functions to operate on it. This QPtr is intended to be
// owned by a Rust object and is not supposed to ever be shared elsewhere as the
// Rust object takes control of deleting the memory behind the pointer.
//
// The reason that a QPtr* is used instead of constructing a QPtr directly into
// Rust allocated memory of the correct size is to prevent the perils that can
// result from Rust trying to move said memory. If we only give a pointer to
// Rust though, it is free to move that in memory as much as it likes.

using QPtr = QPointer<CxxQObject>;

extern "C"
{
  QPtr* cxxqt1$update_requester$new(CxxQObject* qobject_ptr) noexcept
  {
    Q_ASSERT(qobject_ptr != nullptr);
    return new QPtr(qobject_ptr);
  }

  void cxxqt1$update_requester$drop(QPtr* self) noexcept
  {
    Q_ASSERT(self != nullptr);
    delete self;
  }

  bool cxxqt1$update_requester$request_update(const QPtr* self) noexcept
  {
    Q_ASSERT(self != nullptr);

    if (self->isNull())
      return false;

    const auto ret = QMetaObject::invokeMethod(
      self->data(), "requestUpdate", Qt::DirectConnection);

    if (!ret) {
      qWarning() << Q_FUNC_INFO
                 << "Tried to call requestUpdate on object without "
                    "UpdateRequestHandler trait.";
    }

    return ret;
  }

  QPtr* cxxqt1$update_requester$clone(const QPtr* self) noexcept
  {
    Q_ASSERT(self != nullptr);
    return new QPtr(*self);
  }
}

namespace {

// We assume that C++ char and Rust u8 are the same
static_assert(sizeof(char) == sizeof(std::uint8_t));

} // namespace

extern "C"
{
  void cxxqt1$qstring$init$from$rust$string(std::unique_ptr<QString>* ptr,
                                            const char* data,
                                            std::size_t len) noexcept
  {
    new (ptr)
      std::unique_ptr<QString>(new QString(QString::fromUtf8(data, len)));
  }

  void cxxqt1$qstring$to$rust$string(const QString& qt,
                                     rust::String& rust) noexcept
  {
    static_assert(sizeof(char16_t) == sizeof(QChar));
    rust = rust::String(reinterpret_cast<const char16_t*>(qt.constData()),
                        qt.size());
  }

  void cxxqt1$unique_ptr$qstring$null(std::unique_ptr<QString>* ptr) noexcept
  {
    new (ptr) std::unique_ptr<QString>();
  }

  void cxxqt1$unique_ptr$qstring$raw(std::unique_ptr<QString>* ptr,
                                     QString* raw) noexcept
  {
    new (ptr) std::unique_ptr<QString>(raw);
  }

  const QString* cxxqt1$unique_ptr$qstring$get(
    const std::unique_ptr<QString>& ptr) noexcept
  {
    return ptr.get();
  }

  QString* cxxqt1$unique_ptr$qstring$release(
    std::unique_ptr<QString>& ptr) noexcept
  {
    return ptr.release();
  }

  void cxxqt1$unique_ptr$qstring$drop(std::unique_ptr<QString>* ptr) noexcept
  {
    ptr->~unique_ptr();
  }
}

namespace {

// We do these checks to ensure that we can safely store a QDate
// inside a block of memory that Rust thinks contains one i64.
// We also make sure that i64 and qint64 are equivalent.

static_assert(sizeof(qint64) == 8);
static_assert(alignof(qint64) <= 8);

static_assert(sizeof(QDate) == 8);
static_assert(alignof(QDate) <= 8);

// Our Rust code assumes that QDate is trivial. Because it is trivial to move,
// we don't need to use Pin. Because it is trivial to destruct we do not
// need a special C++ function to destruct the object.

static_assert(std::is_trivially_move_assignable<QDate>::value);
static_assert(std::is_trivially_copy_assignable<QDate>::value);
static_assert(std::is_trivially_destructible<QDate>::value);

} // namespace

extern "C"
{
  void cxxqt1$qdate$init(QDate* self, qint32 y, qint32 m, qint32 d) noexcept
  {
    new (self) QDate(y, m, d);
  }

  qint32 cxxqt1$qdate$year(const QDate& self) noexcept { return self.year(); }

  qint32 cxxqt1$qdate$month(const QDate& self) noexcept { return self.month(); }

  qint32 cxxqt1$qdate$day(const QDate& self) noexcept { return self.day(); }

  bool cxxqt1$qdate$set$date(QDate& self, qint32 y, qint32 m, qint32 d) noexcept
  {
    return self.setDate(y, m, d);
  }
}

namespace {

// We do these checks to ensure that we can safely store a QPointF
// inside a block of memory that Rust thinks contains two f64-s.
// We also make sure that f64 and qreal are equivalent.

static_assert(sizeof(qreal) == 8);
static_assert(alignof(qreal) <= 8);

static_assert(sizeof(QPointF) == 16);
static_assert(alignof(QPointF) <= 16);

// Our Rust code assumes that QPointF is trivial. Because it is trivial to move,
// we don't need to use Pin. Because it is trivial to destruct we do not
// need a special C++ function to destruct the object.

static_assert(std::is_trivially_move_assignable<QPointF>::value);
static_assert(std::is_trivially_copy_assignable<QPointF>::value);
static_assert(std::is_trivially_destructible<QPointF>::value);

} // namespace

extern "C"
{
  void cxxqt1$qpointf$init(QPointF* self, qreal x, qreal y) noexcept
  {
    new (self) QPointF(x, y);
  }
}

namespace {
// We do these checks to ensure that we can safely store a QRectF
// inside a block of memory that Rust thinks contains four f64-s.
// We also make sure that f64 and qreal are equivalent.

static_assert(sizeof(qreal) == 8);
static_assert(alignof(qreal) <= 8);

static_assert(sizeof(QRectF) == 32);
static_assert(alignof(QRectF) <= 32);

// Our Rust code assumes that QRectF is trivial. Because it is trivial to move,
// we don't need to use Pin. Because it is trivial to destruct we do not
// need a special C++ function to destruct the object.

static_assert(std::is_trivially_move_assignable<QRectF>::value);
static_assert(std::is_trivially_copy_assignable<QRectF>::value);
static_assert(std::is_trivially_destructible<QRectF>::value);

} // namespace

extern "C"
{
  void cxxqt1$qrectf$init(QRectF* self,
                          qreal xp,
                          qreal yp,
                          qreal w,
                          qreal h) noexcept
  {
    new (self) QRectF(xp, yp, w, h);
  }
}

namespace {

// We do these checks to ensure that we can safely store a QSize
// inside a block of memory that Rust thinks contains two i32-s.
// We also make sure that i32 and int are equivalent.

static_assert(sizeof(int) == 4);
static_assert(alignof(int) <= 4);

static_assert(sizeof(QSize) == 8);
static_assert(alignof(QSize) <= 8);

// Our Rust code assumes that QSize is trivial. Because it is trivial to move,
// we don't need to use Pin. Because it is trivial to destruct we do not
// need a special C++ function to destruct the object.

static_assert(std::is_trivially_move_assignable<QSize>::value);
static_assert(std::is_trivially_copy_assignable<QSize>::value);
static_assert(std::is_trivially_destructible<QSize>::value);

} // namespace

extern "C"
{
  void cxxqt1$qsize$init(QSize* self, int w, int h) noexcept
  {
    new (self) QSize(w, h);
  }
}

namespace {

// We do these checks to ensure that we can safely store a QRect
// inside a block of memory that Rust thinks contains four i32-s.
// We also make sure that i32 and int are equivalent.

static_assert(sizeof(int) == 4);
static_assert(alignof(int) <= 4);

static_assert(sizeof(QRect) == 16);
static_assert(alignof(QRect) <= 16);

// Our Rust code assumes that QRect is trivial. Because it is trivial to move,
// we don't need to use Pin. Because it is trivial to destruct we do not
// need a special C++ function to destruct the object.

static_assert(std::is_trivially_move_assignable<QRect>::value);
static_assert(std::is_trivially_copy_assignable<QRect>::value);
static_assert(std::is_trivially_destructible<QRect>::value);

} // namespace

extern "C"
{
  void cxxqt1$qrect$init(QRect* self, int xp, int yp, int w, int h) noexcept
  {
    new (self) QRect(xp, yp, w, h);
  }
}

namespace {

// We do these checks to ensure that we can safely store a QSizeF
// inside a block of memory that Rust thinks contains two f64-s.
// We also make sure that f64 and qreal are equivalent.

static_assert(sizeof(qreal) == 8);
static_assert(alignof(qreal) <= 8);

static_assert(sizeof(QSizeF) == 16);
static_assert(alignof(QSizeF) <= 16);

// Our Rust code assumes that QSizeF is trivial. Because it is trivial to move,
// we don't need to use Pin. Because it is trivial to destruct we do not
// need a special C++ function to destruct the object.

static_assert(std::is_trivially_move_assignable<QSizeF>::value);
static_assert(std::is_trivially_copy_assignable<QSizeF>::value);
static_assert(std::is_trivially_destructible<QSizeF>::value);

} // namespace

extern "C"
{
  void cxxqt1$qsizef$init(QSizeF* self, qreal w, qreal h) noexcept
  {
    new (self) QSizeF(w, h);
  }
}

namespace {

// We do these checks to ensure that we can safely store a QTime
// inside a block of memory that Rust thinks contains one i32.
// We also make sure that i32 and qint32 are equivalent.

static_assert(sizeof(qint32) == 4);
static_assert(alignof(qint32) <= 4);

static_assert(sizeof(QTime) == 4);
static_assert(alignof(QTime) <= 4);

// Our Rust code assumes that QTime is trivial. Because it is trivial to move,
// we don't need to use Pin. Because it is trivial to destruct we do not
// need a special C++ function to destruct the object.

static_assert(std::is_trivially_move_assignable<QTime>::value);
static_assert(std::is_trivially_copy_assignable<QTime>::value);
static_assert(std::is_trivially_destructible<QTime>::value);

} // namespace

extern "C"
{
  void cxxqt1$qtime$init(QTime* self,
                         qint32 h,
                         qint32 m,
                         qint32 s,
                         qint32 ms) noexcept
  {
    new (self) QTime(h, m, s, ms);
  }

  qint32 cxxqt1$qtime$hour(const QTime& self) noexcept { return self.hour(); }

  qint32 cxxqt1$qtime$minute(const QTime& self) noexcept
  {
    return self.minute();
  }

  qint32 cxxqt1$qtime$second(const QTime& self) noexcept
  {
    return self.second();
  }

  qint32 cxxqt1$qtime$msec(const QTime& self) noexcept { return self.msec(); }

  qint32 cxxqt1$qtime$set$hms(QTime& self,
                              qint32 h,
                              qint32 m,
                              qint32 s,
                              qint32 ms) noexcept
  {
    return self.setHMS(h, m, s, ms);
  }
}

extern "C"
{
  void cxxqt1$qdatetime$init$from$qdatetime(std::unique_ptr<QDateTime>* ptr,
                                            const QDateTime& qdatetime) noexcept
  {
    new (ptr) std::unique_ptr<QDateTime>(new QDateTime(qdatetime));
  }

  void cxxqt1$qdatetime$init$from$date$and$time(std::unique_ptr<QDateTime>* ptr,
                                                const QDate& date,
                                                const QTime& time) noexcept
  {
    new (ptr) std::unique_ptr<QDateTime>(new QDateTime(date, time));
  }

  // We know that QDate and QTime are C++ types
  // but they have a trivial move constructor so this is fine
  DISABLE_WARNING_PUSH
  DISABLE_RETURN_TYPE_C_LINKAGE
  QDate cxxqt1$qdatetime$get$date(const QDateTime& dateTime) noexcept
  {
    return dateTime.date();
  }

  QTime cxxqt1$qdatetime$get$time(const QDateTime& dateTime) noexcept
  {
    return dateTime.time();
  }
  DISABLE_WARNING_POP

  void cxxqt1$qdatetime$set$date(QDateTime& dateTime,
                                 const QDate& date) noexcept
  {
    dateTime.setDate(date);
  }

  void cxxqt1$qdatetime$set$time(QDateTime& dateTime,
                                 const QTime& time) noexcept
  {
    dateTime.setTime(time);
  }

  void cxxqt1$unique_ptr$qdatetime$null(
    std::unique_ptr<QDateTime>* ptr) noexcept
  {
    new (ptr) std::unique_ptr<QDateTime>();
  }

  void cxxqt1$unique_ptr$qdatetime$raw(std::unique_ptr<QDateTime>* ptr,
                                       QDateTime* raw) noexcept
  {
    new (ptr) std::unique_ptr<QDateTime>(raw);
  }

  const QDateTime* cxxqt1$unique_ptr$qdatetime$get(
    const std::unique_ptr<QDateTime>& ptr) noexcept
  {
    return ptr.get();
  }

  QDateTime* cxxqt1$unique_ptr$qdatetime$release(
    std::unique_ptr<QDateTime>& ptr) noexcept
  {
    return ptr.release();
  }

  void cxxqt1$unique_ptr$qdatetime$drop(
    std::unique_ptr<QDateTime>* ptr) noexcept
  {
    ptr->~unique_ptr();
  }
}

extern "C"
{
  void cxxqt1$qurl$init$from$qurl(std::unique_ptr<QUrl>* ptr,
                                  const QUrl& qurl) noexcept
  {
    new (ptr) std::unique_ptr<QUrl>(new QUrl(qurl));
  }

  void cxxqt1$qurl$init$from$string(std::unique_ptr<QUrl>* ptr,
                                    const char* data,
                                    std::size_t len) noexcept
  {
    new (ptr) std::unique_ptr<QUrl>(new QUrl(QString::fromUtf8(data, len)));
  }

  void cxxqt1$qurl$to$rust$string(const QUrl& qt, rust::String& rust) noexcept
  {
    static_assert(sizeof(char16_t) == sizeof(QChar));
    const auto data = qt.url();
    rust = rust::String(reinterpret_cast<const char16_t*>(data.constData()),
                        data.size());
  }

  void cxxqt1$unique_ptr$qurl$null(std::unique_ptr<QUrl>* ptr) noexcept
  {
    new (ptr) std::unique_ptr<QUrl>();
  }

  void cxxqt1$unique_ptr$qurl$raw(std::unique_ptr<QUrl>* ptr,
                                  QUrl* raw) noexcept
  {
    new (ptr) std::unique_ptr<QUrl>(raw);
  }

  const QUrl* cxxqt1$unique_ptr$qurl$get(
    const std::unique_ptr<QUrl>& ptr) noexcept
  {
    return ptr.get();
  }

  QUrl* cxxqt1$unique_ptr$qurl$release(std::unique_ptr<QUrl>& ptr) noexcept
  {
    return ptr.release();
  }

  void cxxqt1$unique_ptr$qurl$drop(std::unique_ptr<QUrl>* ptr) noexcept
  {
    ptr->~unique_ptr();
  }
}

namespace {

enum class QVariantType : uint8_t
{
  Unsupported = 0,
  Bool = 1,
  F32 = 2,
  F64 = 3,
  I8 = 4,
  I16 = 5,
  I32 = 6,
  QColor = 7,
  QDate = 8,
  QDateTime = 9,
  QPoint = 10,
  QPointF = 11,
  QRect = 12,
  QRectF = 13,
  QSize = 14,
  QSizeF = 15,
  String = 16,
  QTime = 17,
  QUrl = 18,
  U8 = 19,
  U16 = 20,
  U32 = 21,
};

} // namespace

#define CXX_QT_VARIANT_INIT(typeName, name)                                    \
  void cxxqt1$qvariant$init$from$##name(std::unique_ptr<QVariant>* ptr,        \
                                        typeName value) noexcept               \
  {                                                                            \
    new (ptr) std::unique_ptr<QVariant>(new QVariant(value));                  \
  }

#define CXX_QT_VARIANT_INIT_REF(typeName, name)                                \
  void cxxqt1$qvariant$init$from$##name(std::unique_ptr<QVariant>* ptr,        \
                                        const typeName& value) noexcept        \
  {                                                                            \
    new (ptr) std::unique_ptr<QVariant>(new QVariant(value));                  \
  }

#define CXX_QT_VARIANT_OPAQUE_VALUE(typeName, name)                            \
  void cxxqt1$qvariant$to$##name(const QVariant& self,                         \
                                 std::unique_ptr<typeName>* ptr) noexcept      \
  {                                                                            \
    Q_ASSERT(self.canConvert<typeName>());                                     \
    new (ptr) std::unique_ptr<typeName>(new typeName(self.value<typeName>())); \
  }

#define CXX_QT_VARIANT_TRIVIAL_VALUE(typeName, name)                           \
  typeName cxxqt1$qvariant$to$##name(const QVariant& self) noexcept            \
  {                                                                            \
    Q_ASSERT(self.canConvert<typeName>());                                     \
    return self.value<typeName>();                                             \
  }

extern "C"
{
  CXX_QT_VARIANT_INIT(QVariant, qvariant)
  CXX_QT_VARIANT_INIT(bool, bool)
  CXX_QT_VARIANT_INIT(float, f32)
  CXX_QT_VARIANT_INIT(double, f64)
  CXX_QT_VARIANT_INIT(qint8, i8)
  CXX_QT_VARIANT_INIT(qint16, i16)
  CXX_QT_VARIANT_INIT(qint32, i32)
  CXX_QT_VARIANT_INIT_REF(QColor, qcolor)
  CXX_QT_VARIANT_INIT_REF(QDate, qdate)
  CXX_QT_VARIANT_INIT_REF(QDateTime, qdatetime)
  CXX_QT_VARIANT_INIT_REF(QPoint, qpoint)
  CXX_QT_VARIANT_INIT_REF(QPointF, qpointf)
  CXX_QT_VARIANT_INIT_REF(QRect, qrect)
  CXX_QT_VARIANT_INIT_REF(QRectF, qrectf)
  CXX_QT_VARIANT_INIT_REF(QSize, qsize)
  CXX_QT_VARIANT_INIT_REF(QSizeF, qsizef)
  CXX_QT_VARIANT_INIT_REF(QTime, qtime)
  CXX_QT_VARIANT_INIT_REF(QUrl, qurl)

  void cxxqt1$qvariant$init$from$str(std::unique_ptr<QVariant>* ptr,
                                     rust::Str s) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(
      new QVariant(QString::fromUtf8(s.data(), s.size())));
  }

  CXX_QT_VARIANT_INIT(quint8, u8)
  CXX_QT_VARIANT_INIT(quint16, u16)
  CXX_QT_VARIANT_INIT(quint32, u32)

  QVariantType cxxqt1$qvariant$get$type(const QVariant& self) noexcept
  {
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
    switch (static_cast<QMetaType::Type>(self.metaType().id())) {
#else
    // QVariant::Type is obsolete, ensure we use QMetaType::Type to avoid
    // warnings
    switch (static_cast<QMetaType::Type>(self.type())) {
#endif
      case QMetaType::Bool:
        return QVariantType::Bool;
      case QMetaType::Float:
        return QVariantType::F32;
      case QMetaType::Double:
        return QVariantType::F64;
      case QMetaType::SChar:
        return QVariantType::I8;
      case QMetaType::Short:
        return QVariantType::I16;
      case QMetaType::Int:
        return QVariantType::I32;
      case QMetaType::QColor:
        return QVariantType::QColor;
      case QMetaType::QDate:
        return QVariantType::QDate;
      case QMetaType::QDateTime:
        return QVariantType::QDateTime;
      case QMetaType::QPoint:
        return QVariantType::QPoint;
      case QMetaType::QPointF:
        return QVariantType::QPointF;
      case QMetaType::QRect:
        return QVariantType::QRect;
      case QMetaType::QRectF:
        return QVariantType::QRectF;
      case QMetaType::QSize:
        return QVariantType::QSize;
      case QMetaType::QSizeF:
        return QVariantType::QSizeF;
      case QMetaType::QString:
        return QVariantType::String;
      case QMetaType::QTime:
        return QVariantType::QTime;
      case QMetaType::QUrl:
        return QVariantType::QUrl;
      case QMetaType::UChar:
        return QVariantType::U8;
      case QMetaType::UShort:
        return QVariantType::U16;
      case QMetaType::UInt:
        return QVariantType::U32;

      default:
        return QVariantType::Unsupported;
    }
  }

  CXX_QT_VARIANT_TRIVIAL_VALUE(bool, bool)
  CXX_QT_VARIANT_TRIVIAL_VALUE(float, f32)
  CXX_QT_VARIANT_TRIVIAL_VALUE(double, f64)
  CXX_QT_VARIANT_TRIVIAL_VALUE(qint8, i8)
  CXX_QT_VARIANT_TRIVIAL_VALUE(qint16, i16)
  CXX_QT_VARIANT_TRIVIAL_VALUE(qint32, i32)
  CXX_QT_VARIANT_OPAQUE_VALUE(QColor, qcolor)
  CXX_QT_VARIANT_TRIVIAL_VALUE(QDate, qdate)
  CXX_QT_VARIANT_OPAQUE_VALUE(QDateTime, qdatetime)
  CXX_QT_VARIANT_TRIVIAL_VALUE(QPoint, qpoint)
  CXX_QT_VARIANT_TRIVIAL_VALUE(QPointF, qpointf)
  CXX_QT_VARIANT_TRIVIAL_VALUE(QRect, qrect)
  CXX_QT_VARIANT_TRIVIAL_VALUE(QRectF, qrectf)
  CXX_QT_VARIANT_TRIVIAL_VALUE(QSize, qsize)
  CXX_QT_VARIANT_TRIVIAL_VALUE(QSizeF, qsizef)
  CXX_QT_VARIANT_TRIVIAL_VALUE(QTime, qtime)
  CXX_QT_VARIANT_OPAQUE_VALUE(QUrl, qurl)

  void cxxqt1$qvariant$copy$to$string(const QVariant& self,
                                      rust::String& string) noexcept
  {
    cxxqt1$qstring$to$rust$string(self.toString(), string);
  }

  CXX_QT_VARIANT_TRIVIAL_VALUE(quint8, u8)
  CXX_QT_VARIANT_TRIVIAL_VALUE(quint16, u16)
  CXX_QT_VARIANT_TRIVIAL_VALUE(quint32, u32)

  void cxxqt1$unique_ptr$qvariant$null(std::unique_ptr<QVariant>* ptr) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>();
  }

  void cxxqt1$unique_ptr$qvariant$raw(std::unique_ptr<QVariant>* ptr,
                                      QVariant* raw) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(raw);
  }

  const QVariant* cxxqt1$unique_ptr$qvariant$get(
    const std::unique_ptr<QVariant>& ptr) noexcept
  {
    return ptr.get();
  }

  QVariant* cxxqt1$unique_ptr$qvariant$release(
    std::unique_ptr<QVariant>& ptr) noexcept
  {
    return ptr.release();
  }

  void cxxqt1$unique_ptr$qvariant$drop(std::unique_ptr<QVariant>* ptr) noexcept
  {
    ptr->~unique_ptr();
  }
}

static const QEvent::Type
createEventType(int hint)
{
  auto eventId = QEvent::registerEventType(hint);
  Q_ASSERT(eventId > -1);
  return static_cast<QEvent::Type>(eventId);
}

const QEvent::Type CxxQObject::ProcessQueueEvent =
  createEventType(QEvent::User + 1);

CxxQObject::CxxQObject(QObject* parent)
  : QObject(parent)
{}

CxxQObject::~CxxQObject() = default;

bool
CxxQObject::event(QEvent* event)
{
  if (event->type() == ProcessQueueEvent) {
    // New Rust-side events might come in while we are processing the queue.
    //
    // If we flip this flag before takeQueue then worst case we get an
    // extra event with nothing to actually process whereas if we do it
    // afterwards then we might miss a queue item to process.
    m_waitingForUpdate.store(false, std::memory_order_relaxed);

    for (const auto& item : takeQueue()) {
      item();
    }
    return true;
  }

  return false;
}

void
CxxQObject::runOnGUIThread(std::function<void()> functor)
{
  // Lock the queue, post the event, add to the queue
  // worst case we'll push an event that does nothing if takeQueue() is
  // waiting on the lock
  const std::lock_guard<std::mutex> guard(m_queueMutex);

  if (!m_waitingForUpdate.exchange(true, std::memory_order_relaxed)) {
    QCoreApplication::postEvent(this, new QEvent(ProcessQueueEvent));
  }

  m_queue.push_back(functor);
}

std::vector<std::function<void()>>
CxxQObject::takeQueue()
{
  const std::lock_guard<std::mutex> guard(m_queueMutex);
  std::vector<std::function<void()>> queue;
  std::swap(m_queue, queue);
  return queue;
}

#endif // NO_QT
