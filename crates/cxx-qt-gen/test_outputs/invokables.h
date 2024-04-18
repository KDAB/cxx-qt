#pragma once

#include <cxx-qt/threading.h>
#include <cxx-qt/type.h>

namespace cxx_qt::my_object {
class MyObject;
using MyObjectCxxQtThread = ::rust::cxxqt1::CxxQtThread<MyObject>;

} // namespace cxx_qt::my_object

#include "cxx-qt-gen/invokables.cxx.h"

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
  void cppMethod() const noexcept;
  Q_INVOKABLE void invokable() const noexcept;
  Q_INVOKABLE void invokableMutable() noexcept;
  Q_INVOKABLE void invokableParameters(QColor const& opaque,
                                       QPoint const& trivial,
                                       ::std::int32_t primitive) const noexcept;
  Q_INVOKABLE ::std::unique_ptr<Opaque> invokableReturnOpaque() noexcept;
  Q_INVOKABLE QPoint invokableReturnTrivial() noexcept;
  Q_INVOKABLE void invokableFinal() const noexcept final;
  Q_INVOKABLE void invokableOverride() const noexcept override;
  Q_INVOKABLE virtual void invokableVirtual() const noexcept;
  Q_INVOKABLE void invokableResultTuple() const;
  Q_INVOKABLE ::rust::String invokableResultType() const;
  explicit MyObject(::std::int32_t arg0, QString const& arg1);
  explicit MyObject();

private:
  explicit MyObject(
    ::cxx_qt::my_object::cxx_qt_my_object::CxxQtConstructorArguments0&& args);
  explicit MyObject(
    ::cxx_qt::my_object::cxx_qt_my_object::CxxQtConstructorArguments1&& args);
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
