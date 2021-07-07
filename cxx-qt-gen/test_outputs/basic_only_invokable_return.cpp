#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(createMyObjectRs())
{}

MyObject::~MyObject() = default;

int
MyObject::double_number(int number) const
{
  return m_rustObj->double_number(number);
}

QString
MyObject::hello_message(const QString& msg) const
{
  return rustStringToQString(m_rustObj->hello_message(qStringToRustStr(msg)));
}

QString
MyObject::static_message() const
{
  return rustStrToQString(m_rustObj->static_message());
}

std::unique_ptr<MyObject>
newMyObject()
{
  return std::make_unique<MyObject>();
}
