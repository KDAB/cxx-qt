#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : CxxQObject(parent)
  , m_rustObj(createMyObjectRs())
{
  initialiseMyObjectCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

void
MyObject::sayHi(const QString& string, int number)
{
  m_rustObj->sayHi(*this, qStringToRustStr(string), number);
}

void
MyObject::sayBye()
{
  m_rustObj->sayBye(*this);
}

std::unique_ptr<MyObject>
newMyObject()
{
  return std::make_unique<MyObject>();
}

} // namespace cxx_qt::my_object
