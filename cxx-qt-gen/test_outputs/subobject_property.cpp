#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(createMyObjectRs())
{}

MyObject::~MyObject() = default;

SubObject*
MyObject::getObj() const
{
  return m_obj;
}

void
MyObject::setObj(SubObject* value)
{
  if (value != m_obj) {
    if (m_ownedObj) {
      m_ownedObj.reset();
    }

    m_obj = value;

    Q_EMIT objChanged();
  }
}

std::unique_ptr<SubObject>
MyObject::takeObj()
{
  auto value = std::move(m_ownedObj);
  setObj(nullptr);
  return value;
}

void
MyObject::giveObj(std::unique_ptr<SubObject> value)
{
  Q_ASSERT(value.get() != m_obj);

  m_ownedObj = std::move(value);
  m_obj = m_ownedObj.get();

  Q_EMIT objChanged();
}

std::unique_ptr<MyObject>
newMyObject()
{
  return std::make_unique<MyObject>();
}
