#include "cxx-qt-gen/multi_object.cxxqt.h"

namespace rust::cxxqtgen1::externcxxqt {
::QMetaObject::Connection
QPushButton_clickedConnect(QPushButton& self,
                           ::rust::Fn<void(QPushButton&, bool checked)> func,
                           ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &QPushButton::clicked,
    &self,
    [&, func = ::std::move(func)](bool checked) {
      const ::rust::cxxqtlib1::MaybeLockGuard<QPushButton> guard(self);
      func(self, ::std::move(checked));
    },
    type);
}

} // namespace rust::cxxqtgen1::externcxxqt

namespace rust::cxxqtgen1::externcxxqt::mynamespace {
::QMetaObject::Connection
ExternObject_dataReadyConnect(
  ::mynamespace::ExternObjectCpp& self,
  ::rust::Fn<void(::mynamespace::ExternObjectCpp&)> func,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::mynamespace::ExternObjectCpp::dataReady,
    &self,
    [&, func = ::std::move(func)]() {
      const ::rust::cxxqtlib1::MaybeLockGuard<::mynamespace::ExternObjectCpp>
        guard(self);
      func(self);
    },
    type);
}

} // namespace rust::cxxqtgen1::externcxxqt::mynamespace

namespace rust::cxxqtgen1::externcxxqt::mynamespace {
::QMetaObject::Connection
ExternObject_errorOccurredConnect(
  ::mynamespace::ExternObjectCpp& self,
  ::rust::Fn<void(::mynamespace::ExternObjectCpp&)> func,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::mynamespace::ExternObjectCpp::errorOccurred,
    &self,
    [&, func = ::std::move(func)]() {
      const ::rust::cxxqtlib1::MaybeLockGuard<::mynamespace::ExternObjectCpp>
        guard(self);
      func(self);
    },
    type);
}

} // namespace rust::cxxqtgen1::externcxxqt::mynamespace

namespace cxx_qt::multi_object {
::std::int32_t const&
MyObject::getPropertyName() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  return getPropertyNameWrapper();
}

void
MyObject::setPropertyName(::std::int32_t const& value)
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
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
      const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
      func(*this);
    },
    type);
}

void
MyObject::invokableName()
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
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
      const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
      func(*this);
    },
    type);
}

MyObject::MyObject(QObject* parent)
  : QStringListModel(parent)
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(
      ::cxx_qt::multi_object::cxx_qt_my_object::createRs())
  , ::rust::cxxqtlib1::CxxQtLocking()
{
}

} // namespace cxx_qt::multi_object

namespace second_object {
::std::int32_t const&
SecondObject::getPropertyName() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<SecondObject> guard(*this);
  return getPropertyNameWrapper();
}

void
SecondObject::setPropertyName(::std::int32_t const& value)
{
  const ::rust::cxxqtlib1::MaybeLockGuard<SecondObject> guard(*this);
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
    [&, func = ::std::move(func)]() {
      const ::rust::cxxqtlib1::MaybeLockGuard<SecondObject> guard(*this);
      func(*this);
    },
    type);
}

void
SecondObject::invokableName()
{
  const ::rust::cxxqtlib1::MaybeLockGuard<SecondObject> guard(*this);
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
    [&, func = ::std::move(func)]() {
      const ::rust::cxxqtlib1::MaybeLockGuard<SecondObject> guard(*this);
      func(*this);
    },
    type);
}

SecondObject::SecondObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqtlib1::CxxQtType<SecondObjectRust>(
      ::second_object::cxx_qt_second_object::createRs())
{
}

} // namespace second_object
