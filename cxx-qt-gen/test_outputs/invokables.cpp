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

void
MyObject::invokable()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->invokable();
}

void
MyObject::invokableCppObj()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->invokableCppObjWrapper(*this);
}

void
MyObject::invokableMutable()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->invokableMutable();
}

void
MyObject::invokableMutableCppObj()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->invokableMutableCppObjWrapper(*this);
}

void
MyObject::invokableNestedParameter(cxx_qt::nested_object::CppObj* nested)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->invokableNestedParameterWrapper(*nested);
}

void
MyObject::invokableParameters(const QColor& opaque, qint32 primitive)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->invokableParametersWrapper(opaque, primitive);
}

void
MyObject::invokableParametersCppObj(qint32 primitive)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->invokableParametersCppObjWrapper(primitive, *this);
}

QColor
MyObject::invokableReturnOpaque()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return std::move(*m_rustObj->invokableReturnOpaqueWrapper());
}

qint32
MyObject::invokableReturnPrimitive()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return m_rustObj->invokableReturnPrimitive();
}

QString
MyObject::invokableReturnStatic()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return std::move(*m_rustObj->invokableReturnStaticWrapper());
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
