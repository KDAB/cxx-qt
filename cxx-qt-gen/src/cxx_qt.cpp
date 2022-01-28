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

// We do these checks to ensure that we can safely store a QString
// inside a block of memory that Rust thinks contains as usize.
//
// We assume that std::size_t is the same size as Rust's usize.
// cxx.cc has some asserts to ensure that is true.

static_assert(alignof(QString) <= alignof(std::size_t),
              "unexpectedly large QString alignment");

static_assert(sizeof(QString) <= sizeof(std::size_t),
              "unexpectedly large QString size");

// We also assume that C++ char and Rust u8 are the same

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

// We do these checks to ensure that we can safely store a QPoint
// inside a block of memory that Rust thinks contains two i32-s.
// We also make sure that i32 and int are equivalent.

static_assert(sizeof(int) == 4);
static_assert(alignof(int) <= 4);

static_assert(sizeof(QPoint) == 8);
static_assert(alignof(QPoint) <= 8);

// Our Rust code assumes that QPoint is trivial. Because it is trivial to move,
// we don't need to use Pin. Because it is trivial to destruct we do not
// need a special C++ function to destruct the object.

static_assert(std::is_trivially_move_assignable<QPoint>::value);
static_assert(std::is_trivially_copy_assignable<QPoint>::value);
static_assert(std::is_trivially_destructible<QPoint>::value);

} // namespace

extern "C"
{
  void cxxqt1$qpoint$init(QPoint* self, int x, int y) noexcept
  {
    new (self) QPoint(x, y);
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

namespace {

// We do these checks to ensure that we can safely store a QColor
// inside a block of memory that Rust thinks contains as 2 usizes.
//
// We assume that std::size_t is the same size as Rust's usize.
// cxx.cc has some asserts to ensure that is true.
static_assert(alignof(QColor) <= alignof(std::size_t[2]),
              "unexpectedly large QColor alignment");
static_assert(sizeof(QColor) <= sizeof(std::size_t[2]),
              "unexpectedly large QColor size");

} // namespace

extern "C"
{
  void cxxqt1$qcolor$init$from$qcolor(std::unique_ptr<QColor>* ptr,
                                      const QColor& qcolor) noexcept
  {
    new (ptr) std::unique_ptr<QColor>(new QColor(qcolor));
  }

  void cxxqt1$qcolor$init$from$rgba(std::unique_ptr<QColor>* ptr,
                                    std::int32_t r,
                                    std::int32_t g,
                                    std::int32_t b,
                                    std::int32_t a) noexcept
  {
    new (ptr) std::unique_ptr<QColor>(new QColor(r, g, b, a));
  }

  int cxxqt1$qcolor$get$alpha(const QColor& self) noexcept
  {
    return self.alpha();
  }

  int cxxqt1$qcolor$get$red(const QColor& self) noexcept { return self.red(); }

  int cxxqt1$qcolor$get$green(const QColor& self) noexcept
  {
    return self.green();
  }

  int cxxqt1$qcolor$get$blue(const QColor& self) noexcept
  {
    return self.blue();
  }

  void cxxqt1$qcolor$set$alpha(QColor& self, int alpha) noexcept
  {
    self.setAlpha(alpha);
  }

  void cxxqt1$qcolor$set$red(QColor& self, int red) noexcept
  {
    self.setRed(red);
  }

  void cxxqt1$qcolor$set$green(QColor& self, int green) noexcept
  {
    self.setGreen(green);
  }

  void cxxqt1$qcolor$set$blue(QColor& self, int blue) noexcept
  {
    self.setBlue(blue);
  }

  void cxxqt1$unique_ptr$qcolor$null(std::unique_ptr<QColor>* ptr) noexcept
  {
    new (ptr) std::unique_ptr<QColor>();
  }

  void cxxqt1$unique_ptr$qcolor$raw(std::unique_ptr<QColor>* ptr,
                                    QColor* raw) noexcept
  {
    new (ptr) std::unique_ptr<QColor>(raw);
  }

  const QColor* cxxqt1$unique_ptr$qcolor$get(
    const std::unique_ptr<QColor>& ptr) noexcept
  {
    return ptr.get();
  }

  QColor* cxxqt1$unique_ptr$qcolor$release(
    std::unique_ptr<QColor>& ptr) noexcept
  {
    return ptr.release();
  }

  void cxxqt1$unique_ptr$qcolor$drop(std::unique_ptr<QColor>* ptr) noexcept
  {
    ptr->~unique_ptr();
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

// We do these checks to ensure that we can safely store a QVariant
// inside a block of memory that Rust thinks contains as 2 usizes.
//
// We assume that std::size_t is the same size as Rust's usize.
// cxx.cc has some asserts to ensure that is true.
static_assert(alignof(QVariant) <= alignof(std::size_t[2]),
              "unexpectedly large QVariant alignment");
static_assert(sizeof(QVariant) <= sizeof(std::size_t[2]),
              "unexpectedly large QVariant size");

enum class QVariantType : uint8_t
{
  Unsupported = 0,
  Bool = 1,
  F32 = 2,
  F64 = 3,
  I8 = 4,
  I16 = 5,
  I32 = 6,
  QPoint = 7,
  QPointF = 8,
  QRect = 9,
  QRectF = 10,
  QSize = 11,
  QSizeF = 12,
  String = 13,
  U8 = 14,
  U16 = 15,
  U32 = 16,
};

} // namespace

extern "C"
{
  void cxxqt1$qvariant$init$from$qvariant(std::unique_ptr<QVariant>* ptr,
                                          const QVariant& qvariant) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(qvariant));
  }

  void cxxqt1$qvariant$init$from$bool(std::unique_ptr<QVariant>* ptr,
                                      bool b) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(b));
  }

  void cxxqt1$qvariant$init$from$f32(std::unique_ptr<QVariant>* ptr,
                                     float f) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(f));
  }

  void cxxqt1$qvariant$init$from$f64(std::unique_ptr<QVariant>* ptr,
                                     double d) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(d));
  }

  void cxxqt1$qvariant$init$from$i8(std::unique_ptr<QVariant>* ptr,
                                    qint8 i) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(i));
  }

  void cxxqt1$qvariant$init$from$i16(std::unique_ptr<QVariant>* ptr,
                                     qint16 i) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(i));
  }

  void cxxqt1$qvariant$init$from$i32(std::unique_ptr<QVariant>* ptr,
                                     qint32 i) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(i));
  }

  void cxxqt1$qvariant$init$from$qpoint(std::unique_ptr<QVariant>* ptr,
                                        const QPoint& point) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(point));
  }

  void cxxqt1$qvariant$init$from$qpointf(std::unique_ptr<QVariant>* ptr,
                                         const QPointF& pointf) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(pointf));
  }

  void cxxqt1$qvariant$init$from$qrect(std::unique_ptr<QVariant>* ptr,
                                       const QRect& rect) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(rect));
  }

  void cxxqt1$qvariant$init$from$qrectf(std::unique_ptr<QVariant>* ptr,
                                        const QRectF& rectf) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(rectf));
  }

  void cxxqt1$qvariant$init$from$qsize(std::unique_ptr<QVariant>* ptr,
                                       const QSize& size) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(size));
  }

  void cxxqt1$qvariant$init$from$qsizef(std::unique_ptr<QVariant>* ptr,
                                        const QSizeF& sizef) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(sizef));
  }

  void cxxqt1$qvariant$init$from$str(std::unique_ptr<QVariant>* ptr,
                                     rust::Str s) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(
      new QVariant(QString::fromUtf8(s.data(), s.size())));
  }

  void cxxqt1$qvariant$init$from$u8(std::unique_ptr<QVariant>* ptr,
                                    quint8 i) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(i));
  }

  void cxxqt1$qvariant$init$from$u16(std::unique_ptr<QVariant>* ptr,
                                     quint16 i) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(i));
  }

  void cxxqt1$qvariant$init$from$u32(std::unique_ptr<QVariant>* ptr,
                                     quint32 i) noexcept
  {
    new (ptr) std::unique_ptr<QVariant>(new QVariant(i));
  }

  QVariantType cxxqt1$qvariant$get$type(const QVariant& self) noexcept
  {
    // QVariant::Type is obsolete, ensure we use QMetaType::Type to avoid
    // warnings
    switch (static_cast<QMetaType::Type>(self.type())) {
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

  bool cxxqt1$qvariant$to$bool(const QVariant& self) noexcept
  {
    return self.toBool();
  }

  float cxxqt1$qvariant$to$f32(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<float>());
    return self.value<float>();
  }

  double cxxqt1$qvariant$to$f64(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<double>());
    return self.value<double>();
  }

  qint8 cxxqt1$qvariant$to$i8(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<qint8>());
    return self.value<qint8>();
  }

  qint16 cxxqt1$qvariant$to$i16(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<qint16>());
    return self.value<qint16>();
  }

  qint32 cxxqt1$qvariant$to$i32(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<qint32>());
    return self.value<qint32>();
  }

  QPoint cxxqt1$qvariant$to$qpoint(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<QPoint>());
    return self.value<QPoint>();
  }

  QPointF cxxqt1$qvariant$to$qpointf(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<QPointF>());
    return self.value<QPointF>();
  }

  QRect cxxqt1$qvariant$to$qrect(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<QRect>());
    return self.value<QRect>();
  }

  QRectF cxxqt1$qvariant$to$qrectf(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<QRectF>());
    return self.value<QRectF>();
  }

  QSize cxxqt1$qvariant$to$qsize(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<QSize>());
    return self.value<QSize>();
  }

  QSizeF cxxqt1$qvariant$to$qsizef(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<QSizeF>());
    return self.value<QSizeF>();
  }

  void cxxqt1$qvariant$copy$to$string(const QVariant& self,
                                      rust::String& string) noexcept
  {
    cxxqt1$qstring$to$rust$string(self.toString(), string);
  }

  quint8 cxxqt1$qvariant$to$u8(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<quint8>());
    return self.value<quint8>();
  }

  quint16 cxxqt1$qvariant$to$u16(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<quint16>());
    return self.value<quint16>();
  }

  quint32 cxxqt1$qvariant$to$u32(const QVariant& self) noexcept
  {
    Q_ASSERT(self.canConvert<quint32>());
    return self.value<quint32>();
  }

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
