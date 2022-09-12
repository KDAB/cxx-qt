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

bool
MyObject::getBoolean() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<bool, bool>{}(
    m_rustObj->getBoolean(*this));
}

void
MyObject::setBoolean(bool value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setBoolean(*this, value);
}

void
MyObject::emitBooleanChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "booleanChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

float
MyObject::getFloat32() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<float, float>{}(
    m_rustObj->getFloat32(*this));
}

void
MyObject::setFloat32(float value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setFloat32(*this, value);
}

void
MyObject::emitFloat32Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "float32Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

double
MyObject::getFloat64() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<double, double>{}(
    m_rustObj->getFloat64(*this));
}

void
MyObject::setFloat64(double value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setFloat64(*this, value);
}

void
MyObject::emitFloat64Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "float64Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

qint8
MyObject::getInt8() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<qint8, qint8>{}(
    m_rustObj->getInt8(*this));
}

void
MyObject::setInt8(qint8 value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setInt8(*this, value);
}

void
MyObject::emitInt8Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "int8Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

qint16
MyObject::getInt16() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<qint16, qint16>{}(
    m_rustObj->getInt16(*this));
}

void
MyObject::setInt16(qint16 value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setInt16(*this, value);
}

void
MyObject::emitInt16Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "int16Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

qint32
MyObject::getInt32() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<qint32, qint32>{}(
    m_rustObj->getInt32(*this));
}

void
MyObject::setInt32(qint32 value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setInt32(*this, value);
}

void
MyObject::emitInt32Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "int32Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

quint8
MyObject::getUint8() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<quint8, quint8>{}(
    m_rustObj->getUint8(*this));
}

void
MyObject::setUint8(quint8 value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setUint8(*this, value);
}

void
MyObject::emitUint8Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "uint8Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

quint16
MyObject::getUint16() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<quint16, quint16>{}(
    m_rustObj->getUint16(*this));
}

void
MyObject::setUint16(quint16 value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setUint16(*this, value);
}

void
MyObject::emitUint16Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "uint16Changed", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

quint32
MyObject::getUint32() const
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<quint32, quint32>{}(
    m_rustObj->getUint32(*this));
}

void
MyObject::setUint32(quint32 value)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->setUint32(*this, value);
}

void
MyObject::emitUint32Changed()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "uint32Changed", Qt::QueuedConnection);
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
