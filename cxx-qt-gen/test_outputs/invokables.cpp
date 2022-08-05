#include "cxx-qt-gen/include/my_object.cxx.h"
#include "cxx-qt-gen/include/my_object.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(createRs())
{
  initialiseCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

const MyObjectRust&
MyObject::unsafe_rust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafe_rust_mut()
{
  return *m_rustObj;
}

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
MyObject::invokableParameters(const QColor& opaque,
                              const QPoint& trivial,
                              qint32 primitive)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->invokableParameters(opaque, trivial, primitive);
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
  return rust::cxxqtlib1::cxx_qt_convert<QColor, std::unique_ptr<QColor>>{}(
    m_rustObj->invokableReturnOpaqueWrapper());
}

qint32
MyObject::invokableReturnPrimitive()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<qint32, qint32>{}(
    m_rustObj->invokableReturnPrimitive());
}

QString
MyObject::invokableReturnStatic()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QString, std::unique_ptr<QString>>{}(
    m_rustObj->invokableReturnStaticWrapper());
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
