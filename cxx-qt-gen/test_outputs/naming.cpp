#include "cxx-qt-gen/include/my_object.cxxqt.h"

MyObject::MyObject(QObject* parent)
  : QStringListModel(parent)
  , m_rustObj(cxx_qt_my_object::createRs())
{
  cxx_qt_my_object::initialiseCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

const MyObjectRust&
MyObject::unsafeRust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafeRustMut()
{
  return *m_rustObj;
}

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
  m_rustObj->invokableNameWrapper(*this);
}

namespace cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}
} // namespace cxx_qt_my_object
