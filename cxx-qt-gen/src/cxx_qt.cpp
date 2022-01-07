// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#ifndef NO_QT

#include "rust/cxx_qt.h"

#include <QMetaObject>
#include <QPointF>
#include <QPointer>
#include <QSize>
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

static_assert(std::is_trivially_move_assignable<QSize>::value == true);
static_assert(std::is_trivially_copy_assignable<QSize>::value == true);
static_assert(std::is_trivially_destructible<QSize>::value == true);

} // namespace

extern "C"
{
  void cxxqt1$qsize$init(QSize* self, int w, int h) noexcept
  {
    new (self) QSize(w, h);
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
  Bool = 1,
  F32 = 2,
  F64 = 3,
  I8 = 4,
  I16 = 5,
  I32 = 6,
  String = 7,
  U8 = 8,
  U16 = 9,
  U32 = 10,
};

} // namespace

extern "C"
{
  void cxxqt1$qvariant$init(QVariant* self) noexcept { new (self) QVariant(); }

  void cxxqt1$qvariant$init$from$bool(QVariant* self, bool b) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(b);
  }

  void cxxqt1$qvariant$init$from$f32(QVariant* self, float f) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(f);
  }

  void cxxqt1$qvariant$init$from$f64(QVariant* self, double d) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(d);
  }

  void cxxqt1$qvariant$init$from$i8(QVariant* self, qint8 i) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(i);
  }

  void cxxqt1$qvariant$init$from$i16(QVariant* self, qint16 i) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(i);
  }

  void cxxqt1$qvariant$init$from$i32(QVariant* self, qint32 i) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(i);
  }

  void cxxqt1$qvariant$init$from$str(QVariant* self, rust::Str s) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(rustStrToQString(s));
  }

  void cxxqt1$qvariant$init$from$u8(QVariant* self, quint8 i) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(i);
  }

  void cxxqt1$qvariant$init$from$u16(QVariant* self, quint16 i) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(i);
  }

  void cxxqt1$qvariant$init$from$u32(QVariant* self, quint32 i) noexcept
  {
    new (self) QVariant();
    *self = QVariant::fromValue(i);
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

  void cxxqt1$qvariant$copy$to$string(const QVariant& self,
                                      rust::String& string) noexcept
  {
    cxxqt1$qstring$to_rust_string(self.toString(), string);
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

  void cxxqt1$qvariant$assign$qvariant(const QVariant& from,
                                       QVariant& to) noexcept
  {
    to = from;
  }

  void cxxqt1$qvariant$drop(QVariant* self) noexcept { self->~QVariant(); }
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
