#pragma once

#include <QtCore/QObject>
#include <QtQml/QQmlEngine>
#include <cstdint>
#include <cxx-qt/casting.h>
#include <cxx-qt/type.h>

namespace cxx_qt::my_object {
class MyObject;

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object {
class CxxName;

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
  A = 0,
  B = 1,
  C = 2
};
Q_ENUM_NS(MyNamespacedEnum)
} // namespace cxx_qt::my_object

namespace other_namespace {
Q_NAMESPACE
enum class MyOtherNamespacedEnum : ::std::int32_t
{
  Variant1 = 0,
  Variant2 = 1
};
Q_ENUM_NS(MyOtherNamespacedEnum)
} // namespace other_namespace

#include "directory/file_ident.cxx.h"

namespace cxx_qt::my_object {
class MyObject
  : public QObject
  , public ::rust::cxxqt1::CxxQtType<MyObjectRust>
{
  Q_OBJECT
public:
  enum class MyEnum : ::std::int32_t
  {
    A = 0
  };
  Q_ENUM(MyEnum)
  enum class MyOtherEnum : ::std::int32_t
  {
    X = 0,
    Y = 1,
    Z = 2
  };
  Q_ENUM(MyOtherEnum)

  virtual ~MyObject() = default;

public:
  Q_INVOKABLE void my_invokable(
    cxx_qt::my_object::MyEnum qenum,
    my_namespace::MyOtherEnum other_qenum) const noexcept;
  explicit MyObject(QObject* parent = nullptr);
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

namespace cxx_qt::my_object {
using MyEnum = ::cxx_qt::my_object::MyObject::MyEnum;
} // namespace cxx_qt::my_object

namespace my_namespace {
using MyOtherEnum = ::cxx_qt::my_object::MyObject::MyOtherEnum;
} // namespace my_namespace

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)

namespace cxx_qt::my_object {
class CxxName
  : public QObject
  , public ::rust::cxxqt1::CxxQtType<InternalObject>
{
  Q_OBJECT
public:
  enum class MyRenamedEnum : ::std::int32_t
  {
    A = 0,
    B = 1,
    C = 2
  };
  Q_ENUM(MyRenamedEnum)

  virtual ~CxxName() = default;

public:
  explicit CxxName(QObject* parent = nullptr);
};

static_assert(::std::is_base_of<QObject, CxxName>::value,
              "CxxName must inherit from QObject");
} // namespace cxx_qt::my_object

namespace cxx_qt::my_object {
using MyRenamedEnum = ::cxx_qt::my_object::CxxName::MyRenamedEnum;
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::CxxName*)
