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

cxx_qt::sub_object::SubObject*
MyObject::getObj() const
{
  return m_obj;
}

void
MyObject::setObj(cxx_qt::sub_object::SubObject* value)
{
  if (value != m_obj) {
    if (m_ownedObj) {
      m_ownedObj.reset();
    }

    m_obj = value;

    runOnGUIThread([&]() { Q_EMIT objChanged(); });
  }
}

std::unique_ptr<cxx_qt::sub_object::SubObject>
MyObject::takeObj()
{
  auto value = std::move(m_ownedObj);
  setObj(nullptr);
  return value;
}

void
MyObject::giveObj(std::unique_ptr<cxx_qt::sub_object::SubObject> value)
{
  Q_ASSERT(value.get() != m_obj);

  m_ownedObj = std::move(value);
  m_obj = m_ownedObj.get();

  runOnGUIThread([&]() { Q_EMIT objChanged(); });
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
