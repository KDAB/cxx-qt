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
  ~MyObject();

public:
  MyObjectRust const& unsafeRust() const;
  MyObjectRust& unsafeRustMut();

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
  [[nodiscard]] ::std::lock_guard<::std::recursive_mutex> unsafeRustLock()
    const;

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
  ~SecondObject();

public:
  SecondObjectRust const& unsafeRust() const;
  SecondObjectRust& unsafeRustMut();

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

private:
  ::rust::Box<SecondObjectRust> m_rustObj;
};

static_assert(::std::is_base_of<QObject, SecondObject>::value,
              "SecondObject must inherit from QObject");
} // namespace cxx_qt::multi_object

Q_DECLARE_METATYPE(cxx_qt::multi_object::SecondObject*)
