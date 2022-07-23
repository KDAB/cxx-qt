#include "my_object.cxx.h"
#include "my_object.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(createRs())
{
  initialiseCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

qint32
MyObject::getPropertyName() const
{
  return m_propertyName;
}

void
MyObject::setPropertyName(qint32 value)
{
  if (!m_initialised) {
    m_propertyName = value;
    return;
  }

  if (value != m_propertyName) {
    m_propertyName = value;

    const auto signalSuccess = QMetaObject::invokeMethod(
      this, "propertyNameChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

void
MyObject::invokableName()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->invokableName();
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
