#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(createRs())
{
  initialiseCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "colorChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "dateChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "dateTimeChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "pointChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "pointfChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "rectChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "rectfChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "sizeChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "sizefChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "stringChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "timeChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "urlChanged", Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "variantChanged", Qt::QueuedConnection));
  }
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
