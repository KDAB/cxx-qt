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

    runOnGUIThread([&]() { Q_EMIT numberChanged(); });
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

    runOnGUIThread([&]() { Q_EMIT stringChanged(); });
  }
}

void
MyObject::sayHi(const QString& string, qint32 number)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->sayHiWrapper(string, number);
}

void
MyObject::sayBye()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->sayBye();
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
