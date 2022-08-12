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

qint32
MyObject::getPrimitive() const
{
  return m_primitive;
}

void
MyObject::setPrimitive(qint32 value)
{
  if (!m_initialised) {
    m_primitive = value;
    return;
  }

  if (value != m_primitive) {
    m_primitive = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "primitiveChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QColor&
MyObject::getOpaque() const
{
  return m_opaque;
}

void
MyObject::setOpaque(const QColor& value)
{
  if (!m_initialised) {
    m_opaque = value;
    return;
  }

  if (value != m_opaque) {
    m_opaque = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "opaqueChanged", Qt::QueuedConnection);
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
