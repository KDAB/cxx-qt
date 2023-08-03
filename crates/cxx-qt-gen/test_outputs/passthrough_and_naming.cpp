#include "cxx-qt-gen/multi_object.cxxqt.h"

namespace cxx_qt::multi_object {

MyObject::~MyObject() {}

::std::int32_t const&
MyObject::getPropertyName() const
{
  const auto guard = unsafeRustLock();
  return getPropertyNameWrapper();
}

void
MyObject::setPropertyName(::std::int32_t const& value)
{
  const auto guard = unsafeRustLock();
  setPropertyNameWrapper(value);
}

::QMetaObject::Connection
MyObject::propertyNameChangedConnect(::rust::Fn<void(MyObject&)> func,
                                     ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::propertyNameChanged,
    this,
    [&, func = ::std::move(func)]() {
      const auto guard = unsafeRustLock();
      func(*this);
    },
    type);
}

void
MyObject::invokableName()
{
  const auto guard = unsafeRustLock();
  invokableNameWrapper();
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
      const auto guard = unsafeRustLock();
      func(*this);
    },
    type);
}

MyObject::MyObject(QObject* parent)
  : QStringListModel(parent)
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(
      ::cxx_qt::multi_object::cxx_qt_my_object::createRs())
{
}

} // namespace cxx_qt::multi_object

namespace cxx_qt::multi_object {

SecondObject::~SecondObject() {}

::std::int32_t const&
SecondObject::getPropertyName() const
{

  return getPropertyNameWrapper();
}

void
SecondObject::setPropertyName(::std::int32_t const& value)
{

  setPropertyNameWrapper(value);
}

::QMetaObject::Connection
SecondObject::propertyNameChangedConnect(::rust::Fn<void(SecondObject&)> func,
                                         ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &SecondObject::propertyNameChanged,
    this,
    [&, func = ::std::move(func)]() { func(*this); },
    type);
}

void
SecondObject::invokableName()
{

  invokableNameWrapper();
}

::QMetaObject::Connection
SecondObject::readyConnect(::rust::Fn<void(SecondObject&)> func,
                           ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &SecondObject::ready,
    this,
    [&, func = ::std::move(func)]() { func(*this); },
    type);
}

SecondObject::SecondObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqtlib1::CxxQtType<SecondObjectRust>(
      ::cxx_qt::multi_object::cxx_qt_second_object::createRs())
{
}

} // namespace cxx_qt::multi_object
