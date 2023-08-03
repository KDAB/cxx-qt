#pragma once

#include <cxx-qt-common/cxxqt_locking.h>
#include <cxx-qt-common/cxxqt_type.h>

namespace cxx_qt::my_object {
class MyObject;

} // namespace cxx_qt::my_object

#include "cxx-qt-gen/ffi.cxx.h"

namespace cxx_qt::my_object {
class MyObject
  : public QObject
  , public ::rust::cxxqtlib1::CxxQtType<MyObjectRust>
  , public ::rust::cxxqtlib1::CxxQtLocking
{
  Q_OBJECT
  Q_PROPERTY(::std::int32_t primitive READ getPrimitive WRITE setPrimitive
               NOTIFY primitiveChanged)
  Q_PROPERTY(
    QPoint trivial READ getTrivial WRITE setTrivial NOTIFY trivialChanged)

public:
  virtual ~MyObject() = default;

public:
  ::std::int32_t const& getPrimitive() const;
  Q_SLOT void setPrimitive(::std::int32_t const& value);
  QPoint const& getTrivial() const;
  Q_SLOT void setTrivial(QPoint const& value);
  Q_SIGNAL void primitiveChanged();
  ::QMetaObject::Connection primitiveChangedConnect(
    ::rust::Fn<void(MyObject&)> func,
    ::Qt::ConnectionType type);
  Q_SIGNAL void trivialChanged();
  ::QMetaObject::Connection trivialChangedConnect(
    ::rust::Fn<void(MyObject&)> func,
    ::Qt::ConnectionType type);
  explicit MyObject(QObject* parent = nullptr);

private:
  ::std::int32_t const& getPrimitiveWrapper() const noexcept;
  void setPrimitiveWrapper(::std::int32_t value) noexcept;
  QPoint const& getTrivialWrapper() const noexcept;
  void setTrivialWrapper(QPoint value) noexcept;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
