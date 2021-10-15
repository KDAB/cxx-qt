#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : CxxQObject(parent)
  , m_rustObj(createRs())
{
  initialiseCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

qint32
MyObject::doubleNumber(qint32 number)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return m_rustObj->doubleNumber(number);
}

QString
MyObject::helloMessage(const QString& msg)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rustStringToQString(m_rustObj->helloMessage(msg));
}

QString
MyObject::staticMessage()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rustStrToQString(m_rustObj->staticMessage());
}

std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}

} // namespace cxx_qt::my_object
