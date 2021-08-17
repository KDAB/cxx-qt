#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

namespace cxx_qt::my_object {

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

std::unique_ptr<MyObject>
newMyObject()
{
  return std::make_unique<MyObject>();
}

} // namespace cxx_qt::my_object
