#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "numberChanged", Qt::QueuedConnection));

    Q_ASSERT(QMetaObject::invokeMethod(
      this,
      [&]() {
        const std::lock_guard<std::mutex> guard(m_rustObjMutex);
        m_rustObj->handlePropertyChange(*this, Property::Number);
      },
      Qt::QueuedConnection));
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

    Q_ASSERT(
      QMetaObject::invokeMethod(this, "stringChanged", Qt::QueuedConnection));

    Q_ASSERT(QMetaObject::invokeMethod(
      this,
      [&]() {
        const std::lock_guard<std::mutex> guard(m_rustObjMutex);
        m_rustObj->handlePropertyChange(*this, Property::String);
      },
      Qt::QueuedConnection));
  }
}

rust::cxxqtlib1::DeferredCall
MyObject::updateRequester()
{
  return rust::cxxqtlib1::DeferredCall(this, "updateState");
}

void
MyObject::updateState()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->handleUpdateRequest(*this);
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
