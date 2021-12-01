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

qint32
MyObject::getNumber() const
{
  return m_number;
}

void
MyObject::setNumber(qint32 value)
{
  if (!m_initialised) {
    m_number = value;
    return;
  }

  if (value != m_number) {
    m_number = value;

    requestEmitSignal([&]() { Q_EMIT numberChanged(); });

    requestPropertyChange(static_cast<int>(Property::Number));
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

    requestEmitSignal([&]() { Q_EMIT stringChanged(); });

    requestPropertyChange(static_cast<int>(Property::String));
  }
}

void
MyObject::updatePropertyChange(int propertyId)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->handlePropertyChange(*this, static_cast<Property>(propertyId));
}

std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}

} // namespace cxx_qt::my_object
