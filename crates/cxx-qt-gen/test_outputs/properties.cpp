#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
  , m_rustObjMutex(std::make_shared<std::recursive_mutex>())
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

const qint32&
MyObject::getPrimitive() const
{
  const std::lock_guard<std::recursive_mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const qint32&, const qint32&>{}(
    m_rustObj->getPrimitive(*this));
}

void
MyObject::emitPrimitiveChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "primitiveChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const QPoint&
MyObject::getTrivial() const
{
  const std::lock_guard<std::recursive_mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const QPoint&, const QPoint&>{}(
    m_rustObj->getTrivial(*this));
}

void
MyObject::emitTrivialChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "trivialChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

const Value&
MyObject::getOpaque() const
{
  const std::lock_guard<std::recursive_mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const Value&,
                                         const ::std::unique_ptr<Opaque>&>{}(
    m_rustObj->getOpaque(*this));
}

void
MyObject::emitOpaqueChanged()
{
  const auto signalSuccess =
    QMetaObject::invokeMethod(this, "opaqueChanged", Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

void
MyObject::setPrimitive(const qint32& value)
{
  const std::lock_guard<std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setPrimitive(
    *this, rust::cxxqtlib1::cxx_qt_convert<qint32, const qint32&>{}(value));
}

void
MyObject::setTrivial(const QPoint& value)
{
  const std::lock_guard<std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setTrivial(
    *this, rust::cxxqtlib1::cxx_qt_convert<QPoint, const QPoint&>{}(value));
}

void
MyObject::setOpaque(const Value& value)
{
  const std::lock_guard<std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setOpaque(
    *this,
    rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<Opaque>, const Value&>{}(
      value));
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object
