#pragma once

#include <QtCore/QObject>
#include <QtQml/QQmlEngine>
#include <cstdint>
#include <cxx-qt-common/cxxqt_locking.h>
#include <cxx-qt-common/cxxqt_maybelockguard.h>
#include <cxx-qt-common/cxxqt_type.h>

namespace cxx_qt::my_object {
class MyObject;
enum class MyEnum : ::std::int32_t
{
  A
};

enum class MyOtherEnum : ::std::int32_t
{
  X,
  Y,
  Z
};

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object {
Q_NAMESPACE
QML_ELEMENT
} // namespace cxx_qt::my_object

namespace other_namespace {
Q_NAMESPACE
} // namespace other_namespace

namespace cxx_qt::my_object {
Q_NAMESPACE
enum class MyNamespacedEnum : ::std::int32_t
{
  A,
  B,
  C
};
Q_ENUM_NS(MyNamespacedEnum)
} // namespace cxx_qt::my_object

namespace other_namespace {
Q_NAMESPACE
enum class MyOtherNamespacedEnum : ::std::int32_t
{
  Variant1,
  Variant2
};
Q_ENUM_NS(MyOtherNamespacedEnum)
} // namespace other_namespace

#include "cxx-qt-gen/ffi.cxx.h"

namespace cxx_qt::my_object {
class MyObject
  : public QObject
  , public ::rust::cxxqt1::CxxQtType<MyObjectRust>
  , public ::rust::cxxqt1::CxxQtLocking
{
  Q_OBJECT
public:
#ifdef Q_MOC_RUN
  enum class MyEnum : ::std::int32_t{ A };
  Q_ENUM(MyEnum)
#else
  using MyEnum = ::cxx_qt::my_object::MyEnum;
  Q_ENUM(MyEnum)
#endif

#ifdef Q_MOC_RUN
  enum class MyOtherEnum : ::std::int32_t{ X, Y, Z };
  Q_ENUM(MyOtherEnum)
#else
  using MyOtherEnum = ::cxx_qt::my_object::MyOtherEnum;
  Q_ENUM(MyOtherEnum)
#endif

  virtual ~MyObject() = default;

public:
  Q_INVOKABLE void myInvokable(
    ::cxx_qt::my_object::MyEnum qenum,
    ::cxx_qt::my_object::MyOtherEnum other_qenum) const;
  explicit MyObject(QObject* parent = nullptr);

private:
  void myInvokableWrapper(
    ::cxx_qt::my_object::MyEnum qenum,
    ::cxx_qt::my_object::MyOtherEnum other_qenum) const noexcept;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
