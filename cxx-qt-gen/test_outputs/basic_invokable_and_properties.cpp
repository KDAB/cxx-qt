#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

MyObject::MyObject(QObject* parent)
  : CxxQObject(parent)
  , m_rustObj(createMyObjectRs())
{}

MyObject::~MyObject() = default;

int
MyObject::getNumber() const
{
  return m_number;
}

void
MyObject::setNumber(int value)
{
  if (value != m_number) {
    m_number = value;

    Q_EMIT numberChanged();
  }
}

const QString&
MyObject::getString() const
{
  return m_string;
}

void
MyObject::setString(const QString& value)
{
  if (value != m_string) {
    m_string = value;

    Q_EMIT stringChanged();
  }
}

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
