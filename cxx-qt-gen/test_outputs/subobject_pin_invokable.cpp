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

void
MyObject::subTest(cxx_qt::sub_object::SubObject* sub)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->subTest(*this, *sub);
}

std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}

} // namespace cxx_qt::my_object
