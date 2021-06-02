#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(create_my_object_rs())
{}

MyObject::~MyObject() = default;

int
MyObject::getNumber() const
{
  return m_rustObj->number();
}

void
MyObject::setNumber(int value)
{
  if (value != m_rustObj->number()) {
    m_rustObj->set_number(value);

    Q_EMIT numberChanged();
  }
}

QString
MyObject::getString() const
{
  return rustStringToQString(m_rustObj->string());
}

void
MyObject::setString(const QString& value)
{
  auto rustValue = qStringToRustString(value);
  if (rustValue != m_rustObj->string()) {
    m_rustObj->set_string(std::move(rustValue));

    Q_EMIT stringChanged();
  }
}

std::unique_ptr<MyObject>
new_MyObject()
{
  return std::make_unique<MyObject>();
}
