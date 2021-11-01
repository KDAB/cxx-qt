// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#ifndef NO_QT

#include "rust/cxx_qt.h"

#include <QPointF>
#include <QPointer>
#include <QSizeF>
#include <QVariant>
#include <QtGui/QColor>

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

    self->data()->requestUpdate();
    return true;
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
  void cxxqt1$qstring$init(QString* self,
                           const char* ptr,
                           std::size_t len) noexcept
  {
    new (self) QString();
    *self = QString::fromUtf8(ptr, len);
  }

  void cxxqt1$qstring$to_rust_string(const QString& qt,
                                     rust::String& rust) noexcept
  {
    static_assert(sizeof(char16_t) == sizeof(QChar));
    rust = rust::String(reinterpret_cast<const char16_t*>(qt.constData()),
                        qt.size());
  }

  void cxxqt1$qstring$drop(QString* self) noexcept { self->~QString(); }
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

static_assert(std::is_trivially_move_assignable<QPointF>::value == true);
static_assert(std::is_trivially_copy_assignable<QPointF>::value == true);
static_assert(std::is_trivially_destructible<QPointF>::value == true);

} // namespace

extern "C"
{
  void cxxqt1$qpointf$init(QPointF* self, qreal x, qreal y) noexcept
  {
    new (self) QPointF(x, y);
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

static_assert(std::is_trivially_move_assignable<QSizeF>::value == true);
static_assert(std::is_trivially_copy_assignable<QSizeF>::value == true);
static_assert(std::is_trivially_destructible<QSizeF>::value == true);

} // namespace

extern "C"
{
  void cxxqt1$qsizef$init(QSizeF* self, qreal w, qreal h) noexcept
  {
    new (self) QSizeF(w, h);
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

enum class QColorSpec : uint8_t
{
  Unsupported = 0,
  Rgb = 1,
};

} // namespace

extern "C"
{
  void cxxqt1$qcolor$init$from$argb(QColor* self,
                                    int a,
                                    int r,
                                    int g,
                                    int b) noexcept
  {
    new (self) QColor();
    *self = QColor(r, g, b, a);
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

  QColorSpec cxxqt1$qcolor$get$spec(const QColor& self) noexcept
  {
    switch (self.spec()) {
      case QColor::Rgb:
        return QColorSpec::Rgb;
      default:
        return QColorSpec::Unsupported;
    }
  }

  void cxxqt1$qcolor$drop(QColor* self) noexcept { self->~QColor(); }
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
  String = 1,
  Int = 2,
  Bool = 3,
};

} // namespace

extern "C"
{
  void cxxqt1$qvariant$init(QVariant* self) noexcept { new (self) QVariant(); }

  void cxxqt1$qvariant$init$from$int(QVariant* self, int i) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(i);
  }

  void cxxqt1$qvariant$init$from$str(QVariant* self, rust::Str s) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(rustStrToQString(s));
  }

  void cxxqt1$qvariant$init$from$bool(QVariant* self, bool b) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(b);
  }

  QVariantType cxxqt1$qvariant$get$type(const QVariant& self) noexcept
  {
    switch (self.type()) {
      case QMetaType::QString:
        return QVariantType::String;
      case QMetaType::Int:
        return QVariantType::Int;
      case QMetaType::Bool:
        return QVariantType::Bool;

      default:
        return QVariantType::Unsupported;
    }
  }

  void cxxqt1$qvariant$copy$to$string(const QVariant& self,
                                      rust::String& string) noexcept
  {
    cxxqt1$qstring$to_rust_string(self.toString(), string);
  }

  int cxxqt1$qvariant$to$int(const QVariant& self) noexcept
  {
    bool ok;
    int result = self.toInt(&ok);
    Q_ASSERT(ok);
    return result;
  }

  bool cxxqt1$qvariant$to$bool(const QVariant& self) noexcept
  {
    return self.toBool();
  }

  void cxxqt1$qvariant$assign$qvariant(const QVariant& from,
                                       QVariant& to) noexcept
  {
    to = from;
  }

  void cxxqt1$qvariant$drop(QVariant* self) noexcept { self->~QVariant(); }
}

const QEvent::Type CxxQObject::UpdateStateEvent = []() {
  auto eventId = QEvent::registerEventType(QEvent::User + 1);
  Q_ASSERT(eventId > -1);
  return static_cast<QEvent::Type>(eventId);
}();

#endif // NO_QT
