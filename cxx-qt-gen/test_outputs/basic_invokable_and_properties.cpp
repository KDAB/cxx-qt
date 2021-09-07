#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : CxxQObject(parent)
  , m_rustObj(createMyObjectRs())
{
  initialiseMyObjectCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

int
MyObject::getNumber() const
{
  return m_number;
}

void
MyObject::setNumber(int value)
{
  if (!m_initialised) {
    m_number = value;
    return;
  }

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
  if (!m_initialised) {
    m_string = value;
    return;
  }

  if (value != m_string) {
    m_string = value;

    Q_EMIT stringChanged();
  }
}

void
MyObject::sayHi(const QString& string, int number)
{
  m_rustObj->sayHi(qStringToRustStr(string), number);
}

void
MyObject::sayBye()
{
  m_rustObj->sayBye();
}

std::unique_ptr<MyObject>
newMyObject()
{
  return std::make_unique<MyObject>();
}

} // namespace cxx_qt::my_object
