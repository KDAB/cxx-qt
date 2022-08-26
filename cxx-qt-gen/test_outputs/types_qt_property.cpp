#include "cxx-qt-gen/include/my_object.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
  , m_rustObjMutex(std::make_shared<std::mutex>())
  , m_cxxQtThreadObj(
      std::make_shared<rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(this))
{
  cxx_qt::my_object::cxx_qt_my_object::initialiseCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject()
{
  const auto guard = std::unique_lock(m_cxxQtThreadObj->mutex);
  m_cxxQtThreadObj->ptr = nullptr;
}

const MyObjectRust&
MyObject::unsafeRust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafeRustMut()
{
  return *m_rustObj;
}

std::unique_ptr<MyObjectCxxQtThread>
MyObject::qtThread() const
{
  return std::make_unique<MyObjectCxxQtThread>(m_cxxQtThreadObj,
                                               m_rustObjMutex);
}

const QColor&
MyObject::getColor() const
{
  return m_color;
}

void
MyObject::setColor(const QColor& value)
{
  if (!m_initialised) {
    m_color = value;
    return;
  }

  if (value != m_color) {
    m_color = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "colorChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QDate&
MyObject::getDate() const
{
  return m_date;
}

void
MyObject::setDate(const QDate& value)
{
  if (!m_initialised) {
    m_date = value;
    return;
  }

  if (value != m_date) {
    m_date = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "dateChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QDateTime&
MyObject::getDateTime() const
{
  return m_dateTime;
}

void
MyObject::setDateTime(const QDateTime& value)
{
  if (!m_initialised) {
    m_dateTime = value;
    return;
  }

  if (value != m_dateTime) {
    m_dateTime = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "dateTimeChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QPoint&
MyObject::getPoint() const
{
  return m_point;
}

void
MyObject::setPoint(const QPoint& value)
{
  if (!m_initialised) {
    m_point = value;
    return;
  }

  if (value != m_point) {
    m_point = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "pointChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QPointF&
MyObject::getPointf() const
{
  return m_pointf;
}

void
MyObject::setPointf(const QPointF& value)
{
  if (!m_initialised) {
    m_pointf = value;
    return;
  }

  if (value != m_pointf) {
    m_pointf = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "pointfChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QRect&
MyObject::getRect() const
{
  return m_rect;
}

void
MyObject::setRect(const QRect& value)
{
  if (!m_initialised) {
    m_rect = value;
    return;
  }

  if (value != m_rect) {
    m_rect = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "rectChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QRectF&
MyObject::getRectf() const
{
  return m_rectf;
}

void
MyObject::setRectf(const QRectF& value)
{
  if (!m_initialised) {
    m_rectf = value;
    return;
  }

  if (value != m_rectf) {
    m_rectf = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "rectfChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QSize&
MyObject::getSize() const
{
  return m_size;
}

void
MyObject::setSize(const QSize& value)
{
  if (!m_initialised) {
    m_size = value;
    return;
  }

  if (value != m_size) {
    m_size = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "sizeChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QSizeF&
MyObject::getSizef() const
{
  return m_sizef;
}

void
MyObject::setSizef(const QSizeF& value)
{
  if (!m_initialised) {
    m_sizef = value;
    return;
  }

  if (value != m_sizef) {
    m_sizef = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "sizefChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QString&
MyObject::getString() const
{
  return m_string;
}

void
MyObject::setString(const QString& value)
{
  if (!m_initialised) {
    m_string = value;
    return;
  }

  if (value != m_string) {
    m_string = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "stringChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QTime&
MyObject::getTime() const
{
  return m_time;
}

void
MyObject::setTime(const QTime& value)
{
  if (!m_initialised) {
    m_time = value;
    return;
  }

  if (value != m_time) {
    m_time = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "timeChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QUrl&
MyObject::getUrl() const
{
  return m_url;
}

void
MyObject::setUrl(const QUrl& value)
{
  if (!m_initialised) {
    m_url = value;
    return;
  }

  if (value != m_url) {
    m_url = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "urlChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QVariant&
MyObject::getVariant() const
{
  return m_variant;
}

void
MyObject::setVariant(const QVariant& value)
{
  if (!m_initialised) {
    m_variant = value;
    return;
  }

  if (value != m_variant) {
    m_variant = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "variantChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object
