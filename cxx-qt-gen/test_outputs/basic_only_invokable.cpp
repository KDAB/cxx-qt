#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(create_my_object_rs())
{}

MyObject::~MyObject() = default;

void
MyObject::say_hi(const QString& string, int number) const
{
  m_rustObj->say_hi(qStringToRustString(string), number);
}

std::unique_ptr<MyObject>
new_MyObject()
{
  return std::make_unique<MyObject>();
}
