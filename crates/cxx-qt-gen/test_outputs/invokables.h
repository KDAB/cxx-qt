#pragma once

#include <cxx-qt/threading.h>
#include <cxx-qt/type.h>

namespace cxx_qt::my_object {
class MyObject;
using MyObjectCxxQtThread = ::rust::cxxqt1::CxxQtThread<MyObject>;

} // namespace cxx_qt::my_object

#include "directory/file_ident.cxx.h"

namespace cxx_qt::my_object {
class MyObject
  : public QObject
  , public ::rust::cxxqt1::CxxQtType<MyObjectRust>
  , public ::rust::cxxqt1::CxxQtThreading<MyObject>
{
  Q_OBJECT
public:
  virtual ~MyObject() = default;

public:
  void cpp_method() const noexcept;
  Q_INVOKABLE void invokable() const noexcept;
  Q_INVOKABLE void invokable_mutable() noexcept;
  Q_INVOKABLE void invokable_parameters(
    QColor const& opaque,
    QPoint const& trivial,
    ::std::int32_t primitive) const noexcept;
  Q_INVOKABLE ::std::unique_ptr<Opaque> invokable_return_opaque() noexcept;
  Q_INVOKABLE QPoint invokable_return_trivial() noexcept;
  Q_INVOKABLE void invokable_final() const noexcept final;
  Q_INVOKABLE void invokable_override() const noexcept override;
  Q_INVOKABLE virtual void invokable_virtual() const noexcept;
  Q_INVOKABLE virtual void invokable_pure_virtual() const noexcept = 0;
  Q_INVOKABLE void invokable_result_tuple() const;
  Q_INVOKABLE ::rust::String invokable_result_type() const;
  explicit MyObject(::std::int32_t arg0, QString const& arg1);
  explicit MyObject();

private:
  explicit MyObject(
    ::cxx_qt::my_object::cxx_qt_MyObject::CxxQtConstructorArguments0&& args);
  explicit MyObject(
    ::cxx_qt::my_object::cxx_qt_MyObject::CxxQtConstructorArguments1&& args);
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
