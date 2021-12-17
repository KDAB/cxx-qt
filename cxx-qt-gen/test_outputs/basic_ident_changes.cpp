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
MyObject::getMyNumber() const
{
  return m_myNumber;
}

void
MyObject::setMyNumber(qint32 value)
{
  if (!m_initialised) {
    m_myNumber = value;
    return;
  }

  if (value != m_myNumber) {
    m_myNumber = value;

    runOnGUIThread([&]() { Q_EMIT myNumberChanged(); });
  }
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
