#pragma once

#include <cxx-qt-common/cxxqt_locking.h>
#include <cxx-qt-common/cxxqt_type.h>

namespace cxx_qt::multi_object {
class MyObject;

} // namespace cxx_qt::multi_object

namespace cxx_qt::multi_object {
class SecondObject;

} // namespace cxx_qt::multi_object

#include "cxx-qt-gen/multi_object.cxx.h"

namespace cxx_qt::multi_object {
class MyObject
  : public QStringListModel
  , public ::rust::cxxqtlib1::CxxQtType<MyObjectRust>
  , public ::rust::cxxqtlib1::CxxQtLocking
{
  Q_OBJECT
  Q_PROPERTY(::std::int32_t propertyName READ getPropertyName WRITE
               setPropertyName NOTIFY propertyNameChanged)

public:
  virtual ~MyObject() = default;

public:
  ::std::int32_t const& getPropertyName() const;
  Q_SLOT void setPropertyName(::std::int32_t const& value);
  Q_SIGNAL void propertyNameChanged();
  ::QMetaObject::Connection propertyNameChangedConnect(
    ::rust::Fn<void(MyObject&)> func,
    ::Qt::ConnectionType type);
  Q_INVOKABLE void invokableName();
  Q_SIGNAL void ready();
  ::QMetaObject::Connection readyConnect(::rust::Fn<void(MyObject&)> func,
                                         ::Qt::ConnectionType type);
  explicit MyObject(QObject* parent = nullptr);

private:
  ::std::int32_t const& getPropertyNameWrapper() const noexcept;
  void setPropertyNameWrapper(::std::int32_t value) noexcept;
  void invokableNameWrapper() noexcept;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::multi_object

Q_DECLARE_METATYPE(cxx_qt::multi_object::MyObject*)

namespace cxx_qt::multi_object {
class SecondObject
  : public QObject
  , public ::rust::cxxqtlib1::CxxQtType<SecondObjectRust>
{
  Q_OBJECT
  Q_PROPERTY(::std::int32_t propertyName READ getPropertyName WRITE
               setPropertyName NOTIFY propertyNameChanged)

public:
  virtual ~SecondObject() = default;

public:
  ::std::int32_t const& getPropertyName() const;
  Q_SLOT void setPropertyName(::std::int32_t const& value);
  Q_SIGNAL void propertyNameChanged();
  ::QMetaObject::Connection propertyNameChangedConnect(
    ::rust::Fn<void(SecondObject&)> func,
    ::Qt::ConnectionType type);
  Q_INVOKABLE void invokableName();
  Q_SIGNAL void ready();
  ::QMetaObject::Connection readyConnect(::rust::Fn<void(SecondObject&)> func,
                                         ::Qt::ConnectionType type);
  explicit SecondObject(QObject* parent = nullptr);

private:
  ::std::int32_t const& getPropertyNameWrapper() const noexcept;
  void setPropertyNameWrapper(::std::int32_t value) noexcept;
  void invokableNameWrapper() noexcept;
};

static_assert(::std::is_base_of<QObject, SecondObject>::value,
              "SecondObject must inherit from QObject");
} // namespace cxx_qt::multi_object

Q_DECLARE_METATYPE(cxx_qt::multi_object::SecondObject*)
