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

const bool&
MyObject::getBoolean() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const bool&, const bool&>{}(
    m_rustObj->getBoolean(*this));
}

void
MyObject::emitBooleanChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "booleanChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const float&
MyObject::getFloat32() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const float&, const float&>{}(
    m_rustObj->getFloat32(*this));
}

void
MyObject::emitFloat32Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "float32Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const double&
MyObject::getFloat64() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const double&, const double&>{}(
    m_rustObj->getFloat64(*this));
}

void
MyObject::emitFloat64Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "float64Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const qint8&
MyObject::getInt8() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const qint8&, const qint8&>{}(
    m_rustObj->getInt8(*this));
}

void
MyObject::emitInt8Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "int8Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const qint16&
MyObject::getInt16() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const qint16&, const qint16&>{}(
    m_rustObj->getInt16(*this));
}

void
MyObject::emitInt16Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "int16Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const qint32&
MyObject::getInt32() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const qint32&, const qint32&>{}(
    m_rustObj->getInt32(*this));
}

void
MyObject::emitInt32Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "int32Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const quint8&
MyObject::getUint8() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const quint8&, const quint8&>{}(
    m_rustObj->getUint8(*this));
}

void
MyObject::emitUint8Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "uint8Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const quint16&
MyObject::getUint16() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const quint16&, const quint16&>{}(
    m_rustObj->getUint16(*this));
}

void
MyObject::emitUint16Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "uint16Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const quint32&
MyObject::getUint32() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const quint32&, const quint32&>{}(
    m_rustObj->getUint32(*this));
}

void
MyObject::emitUint32Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "uint32Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

void
MyObject::setBoolean(const bool& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setBoolean(
    *this, rust::cxxqtlib1::cxx_qt_convert<bool, const bool&>{}(value));
}

void
MyObject::setFloat32(const float& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setFloat32(
    *this, rust::cxxqtlib1::cxx_qt_convert<float, const float&>{}(value));
}

void
MyObject::setFloat64(const double& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setFloat64(
    *this, rust::cxxqtlib1::cxx_qt_convert<double, const double&>{}(value));
}

void
MyObject::setInt8(const qint8& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setInt8(
    *this, rust::cxxqtlib1::cxx_qt_convert<qint8, const qint8&>{}(value));
}

void
MyObject::setInt16(const qint16& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setInt16(
    *this, rust::cxxqtlib1::cxx_qt_convert<qint16, const qint16&>{}(value));
}

void
MyObject::setInt32(const qint32& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setInt32(
    *this, rust::cxxqtlib1::cxx_qt_convert<qint32, const qint32&>{}(value));
}

void
MyObject::setUint8(const quint8& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setUint8(
    *this, rust::cxxqtlib1::cxx_qt_convert<quint8, const quint8&>{}(value));
}

void
MyObject::setUint16(const quint16& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setUint16(
    *this, rust::cxxqtlib1::cxx_qt_convert<quint16, const quint16&>{}(value));
}

void
MyObject::setUint32(const quint32& value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setUint32(
    *this, rust::cxxqtlib1::cxx_qt_convert<quint32, const quint32&>{}(value));
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object
