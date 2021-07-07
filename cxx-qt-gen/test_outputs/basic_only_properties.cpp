#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(createMyObjectRs())
{}

MyObject::~MyObject() = default;

int
MyObject::getNumber() const
{
  return m_rustObj->getNumber();
}

void
MyObject::setNumber(int value)
{
  if (value != m_rustObj->getNumber()) {
    m_rustObj->setNumber(value);

    Q_EMIT numberChanged();
  }
}

QString
MyObject::getString() const
{
  return rustStringToQString(m_rustObj->getString());
}

void
MyObject::setString(const QString& value)
{
  auto rustValue = qStringToRustString(value);
  if (rustValue != m_rustObj->getString()) {
    m_rustObj->setString(std::move(rustValue));

    Q_EMIT stringChanged();
  }
}

std::unique_ptr<MyObject>
newMyObject()
{
  return std::make_unique<MyObject>();
}
