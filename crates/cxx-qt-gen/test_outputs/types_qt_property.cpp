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

const QColor&
MyObject::getColor() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QColor&,
                                         const ::std::unique_ptr<QColor>&>{}(
    m_rustObj->getColor(*this));
}

void
MyObject::emitColorChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "colorChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QDate&
MyObject::getDate() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QDate&, const QDate&>{}(
    m_rustObj->getDate(*this));
}

void
MyObject::emitDateChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "dateChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QDateTime&
MyObject::getDateTime() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QDateTime&,
                                         const ::std::unique_ptr<QDateTime>&>{}(
    m_rustObj->getDateTime(*this));
}

void
MyObject::emitDateTimeChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "dateTimeChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QPoint&
MyObject::getPoint() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QPoint&, const QPoint&>{}(
    m_rustObj->getPoint(*this));
}

void
MyObject::emitPointChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "pointChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QPointF&
MyObject::getPointf() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QPointF&, const QPointF&>{}(
    m_rustObj->getPointf(*this));
}

void
MyObject::emitPointfChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "pointfChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QRect&
MyObject::getRect() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QRect&, const QRect&>{}(
    m_rustObj->getRect(*this));
}

void
MyObject::emitRectChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "rectChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QRectF&
MyObject::getRectf() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QRectF&, const QRectF&>{}(
    m_rustObj->getRectf(*this));
}

void
MyObject::emitRectfChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "rectfChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QSize&
MyObject::getSize() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QSize&, const QSize&>{}(
    m_rustObj->getSize(*this));
}

void
MyObject::emitSizeChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "sizeChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QSizeF&
MyObject::getSizef() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QSizeF&, const QSizeF&>{}(
    m_rustObj->getSizef(*this));
}

void
MyObject::emitSizefChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "sizefChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QString&
MyObject::getString() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QString&,
                                         const ::std::unique_ptr<QString>&>{}(
    m_rustObj->getString(*this));
}

void
MyObject::emitStringChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "stringChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QTime&
MyObject::getTime() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QTime&, const QTime&>{}(
    m_rustObj->getTime(*this));
}

void
MyObject::emitTimeChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "timeChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QUrl&
MyObject::getUrl() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QUrl&,
                                         const ::std::unique_ptr<QUrl>&>{}(
    m_rustObj->getUrl(*this));
}

void
MyObject::emitUrlChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "urlChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QVariant&
MyObject::getVariant() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QVariant&,
                                         const ::std::unique_ptr<QVariant>&>{}(
    m_rustObj->getVariant(*this));
}

void
MyObject::emitVariantChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "variantChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

void
MyObject::setColor(const QColor& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setColor(
    *this,
    rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<QColor>, const QColor&>{}(
      value));
}

void
MyObject::setDate(const QDate& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setDate(
    *this, rust::cxxqtlib1::cxx_qt_convert<QDate, const QDate&>{}(value));
}

void
MyObject::setDateTime(const QDateTime& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setDateTime(
    *this,
    rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<QDateTime>,
                                    const QDateTime&>{}(value));
}

void
MyObject::setPoint(const QPoint& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setPoint(
    *this, rust::cxxqtlib1::cxx_qt_convert<QPoint, const QPoint&>{}(value));
}

void
MyObject::setPointf(const QPointF& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setPointf(
    *this, rust::cxxqtlib1::cxx_qt_convert<QPointF, const QPointF&>{}(value));
}

void
MyObject::setRect(const QRect& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setRect(
    *this, rust::cxxqtlib1::cxx_qt_convert<QRect, const QRect&>{}(value));
}

void
MyObject::setRectf(const QRectF& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setRectf(
    *this, rust::cxxqtlib1::cxx_qt_convert<QRectF, const QRectF&>{}(value));
}

void
MyObject::setSize(const QSize& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setSize(
    *this, rust::cxxqtlib1::cxx_qt_convert<QSize, const QSize&>{}(value));
}

void
MyObject::setSizef(const QSizeF& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setSizef(
    *this, rust::cxxqtlib1::cxx_qt_convert<QSizeF, const QSizeF&>{}(value));
}

void
MyObject::setString(const QString& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setString(
    *this,
    rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<QString>,
                                    const QString&>{}(value));
}

void
MyObject::setTime(const QTime& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setTime(
    *this, rust::cxxqtlib1::cxx_qt_convert<QTime, const QTime&>{}(value));
}

void
MyObject::setUrl(const QUrl& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setUrl(
    *this,
    rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<QUrl>, const QUrl&>{}(
      value));
}

void
MyObject::setVariant(const QVariant& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setVariant(
    *this,
    rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<QVariant>,
                                    const QVariant&>{}(value));
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object
