#include "cxx-qt-gen/my_object.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QStringListModel(parent)
  , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
  , m_rustObjMutex(std::make_shared<std::recursive_mutex>())
  , m_cxxQtThreadObj(
      std::make_shared<rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(this))
{
}

MyObject::~MyObject()
{
  const auto guard = std::unique_lock(m_cxxQtThreadObj->mutex);
  m_cxxQtThreadObj->ptr = nullptr;
}

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

std::unique_ptr<MyObjectCxxQtThread>
MyObject::qtThread() const
{
  return std::make_unique<MyObjectCxxQtThread>(m_cxxQtThreadObj,
                                               m_rustObjMutex);
}

const qint32&
MyObject::getPropertyName() const
{
  const std::lock_guard<std::recursive_mutex> guard(*m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<const qint32&, const qint32&>{}(
    m_rustObj->getPropertyName(*this));
}

void
MyObject::invokableName()
{
  const std::lock_guard<std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableNameWrapper(*this);
}

void
MyObject::setPropertyName(const qint32& value)
{
  const std::lock_guard<std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setPropertyName(
    *this, rust::cxxqtlib1::cxx_qt_convert<qint32, const qint32&>{}(value));
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object
