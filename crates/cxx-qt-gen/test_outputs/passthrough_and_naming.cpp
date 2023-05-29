#include "cxx-qt-gen/multi_object.cxxqt.h"

namespace cxx_qt::multi_object {

MyObject::MyObject(QObject* parent)
  : QStringListModel(parent)
  , m_rustObj(cxx_qt::multi_object::cxx_qt_my_object::createRs())
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
  , m_cxxQtThreadObj(
      ::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(
        this))
{
}

MyObject::~MyObject()
{
  const auto guard = ::std::unique_lock(m_cxxQtThreadObj->mutex);
  m_cxxQtThreadObj->ptr = nullptr;
}

MyObjectRust const&
MyObject::unsafeRust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafeRustMut()
{
  return *m_rustObj;
}

::std::unique_ptr<MyObjectCxxQtThread>
MyObject::qtThread() const
{
  return ::std::make_unique<MyObjectCxxQtThread>(m_cxxQtThreadObj,
                                                 m_rustObjMutex);
}

::std::int32_t const&
MyObject::getPropertyName() const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return m_rustObj->getPropertyName(*this);
}

void
MyObject::setPropertyName(::std::int32_t const& value)
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setPropertyName(*this, value);
}

void
MyObject::invokableName()
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableNameWrapper(*this);
}

void
MyObject::emitReady()
{
  Q_EMIT ready();
}

::QMetaObject::Connection
MyObject::readyConnect(::rust::Fn<void(MyObject&)> func,
                       ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::ready,
    this,
    [&, func = ::std::move(func)]() {
      const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
      func(*this);
    },
    type);
}

} // namespace cxx_qt::multi_object

namespace cxx_qt::multi_object::cxx_qt_my_object {
::std::unique_ptr<MyObject>
newCppObject()
{
  return ::std::make_unique<MyObject>();
}
} // namespace cxx_qt::multi_object::cxx_qt_my_object

namespace cxx_qt::multi_object {

SecondObject::SecondObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(cxx_qt::multi_object::cxx_qt_second_object::createRs())
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
  , m_cxxQtThreadObj(
      ::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<SecondObject>>(
        this))
{
}

SecondObject::~SecondObject()
{
  const auto guard = ::std::unique_lock(m_cxxQtThreadObj->mutex);
  m_cxxQtThreadObj->ptr = nullptr;
}

SecondObjectRust const&
SecondObject::unsafeRust() const
{
  return *m_rustObj;
}

SecondObjectRust&
SecondObject::unsafeRustMut()
{
  return *m_rustObj;
}

::std::unique_ptr<SecondObjectCxxQtThread>
SecondObject::qtThread() const
{
  return ::std::make_unique<SecondObjectCxxQtThread>(m_cxxQtThreadObj,
                                                     m_rustObjMutex);
}

::std::int32_t const&
SecondObject::getPropertyName() const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return m_rustObj->getPropertyName(*this);
}

void
SecondObject::setPropertyName(::std::int32_t const& value)
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setPropertyName(*this, value);
}

void
SecondObject::invokableName()
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableNameWrapper(*this);
}

void
SecondObject::emitReady()
{
  Q_EMIT ready();
}

::QMetaObject::Connection
SecondObject::readyConnect(::rust::Fn<void(SecondObject&)> func,
                           ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &SecondObject::ready,
    this,
    [&, func = ::std::move(func)]() {
      const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
      func(*this);
    },
    type);
}

} // namespace cxx_qt::multi_object

namespace cxx_qt::multi_object::cxx_qt_second_object {
::std::unique_ptr<SecondObject>
newCppObject()
{
  return ::std::make_unique<SecondObject>();
}
} // namespace cxx_qt::multi_object::cxx_qt_second_object
