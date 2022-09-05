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

void
MyObject::invokable()
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableWrapper(*this);
}

void
MyObject::invokableMutable()
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableMutableWrapper(*this);
}

void
MyObject::invokableParameters(const QColor& opaque,
                              const QPoint& trivial,
                              qint32 primitive)
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableParametersWrapper(*this, opaque, trivial, primitive);
}

QColor
MyObject::invokableReturnOpaque()
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QColor, ::std::unique_ptr<QColor>>{}(
    m_rustObj->invokableReturnOpaqueWrapper(*this));
}

qint32
MyObject::invokableReturnPrimitive()
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<qint32, qint32>{}(
    m_rustObj->invokableReturnPrimitiveWrapper(*this));
}

QString
MyObject::invokableReturnStatic()
{
  const std::lock_guard<std::mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QString, ::std::unique_ptr<QString>>{}(
    m_rustObj->invokableReturnStaticWrapper(*this));
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object
