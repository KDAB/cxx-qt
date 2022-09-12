#include "cxx-qt-gen/include/my_object.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
  , m_rustObjMutex(std::make_shared<std::mutex>())
  , m_cxxQtThreadObj(
      std::make_shared<rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(this))
{
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

QColor
MyObject::getColor() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QColor, std::unique_ptr<QColor>>{}(
    m_rustObj->getColor(*this));
}

void
MyObject::setColor(const QColor& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setColor(*this, value);
}

void
MyObject::emitColorChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "colorChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QDate
MyObject::getDate() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QDate, QDate>{}(
    m_rustObj->getDate(*this));
}

void
MyObject::setDate(const QDate& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setDate(*this, value);
}

void
MyObject::emitDateChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "dateChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QDateTime
MyObject::getDateTime() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QDateTime,
                                         std::unique_ptr<QDateTime>>{}(
    m_rustObj->getDateTime(*this));
}

void
MyObject::setDateTime(const QDateTime& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setDateTime(*this, value);
}

void
MyObject::emitDateTimeChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "dateTimeChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QPoint
MyObject::getPoint() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QPoint, QPoint>{}(
    m_rustObj->getPoint(*this));
}

void
MyObject::setPoint(const QPoint& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setPoint(*this, value);
}

void
MyObject::emitPointChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "pointChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QPointF
MyObject::getPointf() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QPointF, QPointF>{}(
    m_rustObj->getPointf(*this));
}

void
MyObject::setPointf(const QPointF& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setPointf(*this, value);
}

void
MyObject::emitPointfChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "pointfChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QRect
MyObject::getRect() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QRect, QRect>{}(
    m_rustObj->getRect(*this));
}

void
MyObject::setRect(const QRect& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setRect(*this, value);
}

void
MyObject::emitRectChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "rectChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QRectF
MyObject::getRectf() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QRectF, QRectF>{}(
    m_rustObj->getRectf(*this));
}

void
MyObject::setRectf(const QRectF& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setRectf(*this, value);
}

void
MyObject::emitRectfChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "rectfChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QSize
MyObject::getSize() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QSize, QSize>{}(
    m_rustObj->getSize(*this));
}

void
MyObject::setSize(const QSize& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setSize(*this, value);
}

void
MyObject::emitSizeChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "sizeChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QSizeF
MyObject::getSizef() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QSizeF, QSizeF>{}(
    m_rustObj->getSizef(*this));
}

void
MyObject::setSizef(const QSizeF& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setSizef(*this, value);
}

void
MyObject::emitSizefChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "sizefChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QString
MyObject::getString() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QString, std::unique_ptr<QString>>{}(
    m_rustObj->getString(*this));
}

void
MyObject::setString(const QString& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setString(*this, value);
}

void
MyObject::emitStringChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "stringChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QTime
MyObject::getTime() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QTime, QTime>{}(
    m_rustObj->getTime(*this));
}

void
MyObject::setTime(const QTime& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setTime(*this, value);
}

void
MyObject::emitTimeChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "timeChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QUrl
MyObject::getUrl() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QUrl, std::unique_ptr<QUrl>>{}(
    m_rustObj->getUrl(*this));
}

void
MyObject::setUrl(const QUrl& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setUrl(*this, value);
}

void
MyObject::emitUrlChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "urlChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

QVariant
MyObject::getVariant() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QVariant, std::unique_ptr<QVariant>>{}(
    m_rustObj->getVariant(*this));
}

void
MyObject::setVariant(const QVariant& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setVariant(*this, value);
}

void
MyObject::emitVariantChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "variantChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object
