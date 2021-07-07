#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(createMyObjectRs())
{}

MyObject::~MyObject() = default;

void
MyObject::say_hi(const QString& string, int number) const
{
  m_rustObj->say_hi(qStringToRustStr(string), number);
}

void
MyObject::say_bye() const
{
  m_rustObj->say_bye();
}

std::unique_ptr<MyObject>
newMyObject()
{
  return std::make_unique<MyObject>();
}
