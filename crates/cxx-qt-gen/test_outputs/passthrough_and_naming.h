#pragma once

#include <memory>
#include <mutex>

namespace rust::cxxqtlib1 {
template<typename T>
class CxxQtThread;
}

namespace cxx_qt::multi_object {
class MyObject;

} // namespace cxx_qt::multi_object

namespace cxx_qt::multi_object {
class SecondObject;

} // namespace cxx_qt::multi_object

#include "cxx-qt-gen/multi_object.cxx.h"

namespace cxx_qt::multi_object {
class MyObject : public QStringListModel
{
  Q_OBJECT
  Q_PROPERTY(::std::int32_t propertyName READ getPropertyName WRITE
               setPropertyName NOTIFY propertyNameChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  MyObjectRust const& unsafeRust() const;
  MyObjectRust& unsafeRustMut();

public:
  ::std::int32_t const& getPropertyName() const;
  Q_SLOT void setPropertyName(::std::int32_t const& value);
  Q_SIGNAL void propertyNameChanged();
  void emitPropertyNameChanged();
  ::QMetaObject::Connection propertyNameChangedConnect(
    ::rust::Fn<void(MyObject&)> func,
    ::Qt::ConnectionType type);
  Q_INVOKABLE void invokableName();
  Q_SIGNAL void ready();
  void emitReady();
  ::QMetaObject::Connection readyConnect(::rust::Fn<void(MyObject&)> func,
                                         ::Qt::ConnectionType type);

private:
  ::rust::Box<MyObjectRust> m_rustObj;
  ::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::multi_object

Q_DECLARE_METATYPE(cxx_qt::multi_object::MyObject*)

namespace cxx_qt::multi_object {
class SecondObject : public QObject
{
  Q_OBJECT
  Q_PROPERTY(::std::int32_t propertyName READ getPropertyName WRITE
               setPropertyName NOTIFY propertyNameChanged)

public:
  explicit SecondObject(QObject* parent = nullptr);
  ~SecondObject();
  SecondObjectRust const& unsafeRust() const;
  SecondObjectRust& unsafeRustMut();

public:
  ::std::int32_t const& getPropertyName() const;
  Q_SLOT void setPropertyName(::std::int32_t const& value);
  Q_SIGNAL void propertyNameChanged();
  void emitPropertyNameChanged();
  ::QMetaObject::Connection propertyNameChangedConnect(
    ::rust::Fn<void(SecondObject&)> func,
    ::Qt::ConnectionType type);
  Q_INVOKABLE void invokableName();
  Q_SIGNAL void ready();
  void emitReady();
  ::QMetaObject::Connection readyConnect(::rust::Fn<void(SecondObject&)> func,
                                         ::Qt::ConnectionType type);

private:
  ::rust::Box<SecondObjectRust> m_rustObj;
};

static_assert(::std::is_base_of<QObject, SecondObject>::value,
              "SecondObject must inherit from QObject");
} // namespace cxx_qt::multi_object

Q_DECLARE_METATYPE(cxx_qt::multi_object::SecondObject*)
