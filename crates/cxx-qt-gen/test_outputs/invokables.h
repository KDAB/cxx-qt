#pragma once

#include <memory>
#include <mutex>

namespace rust::cxxqtlib1 {
template<typename T>
class CxxQtThread;
}

namespace cxx_qt::my_object {
class MyObject;
using MyObjectCxxQtThread = ::rust::cxxqtlib1::CxxQtThread<MyObject>;
} // namespace cxx_qt::my_object

#include "cxx-qt-gen/ffi.cxx.h"

namespace cxx_qt::my_object {
class MyObject : public QObject
{
  Q_OBJECT

public:
  ~MyObject();
  MyObjectRust const& unsafeRust() const;
  MyObjectRust& unsafeRustMut();

public:
  void cppMethod() const;
  Q_INVOKABLE void invokable() const;
  Q_INVOKABLE void invokableMutable();
  Q_INVOKABLE void invokableParameters(QColor const& opaque,
                                       QPoint const& trivial,
                                       ::std::int32_t primitive) const;
  Q_INVOKABLE ::std::unique_ptr<Opaque> invokableReturnOpaque();
  Q_INVOKABLE QPoint invokableReturnTrivial();
  Q_INVOKABLE void invokableFinal() const final;
  Q_INVOKABLE void invokableOverride() const override;
  Q_INVOKABLE virtual void invokableVirtual() const;
  Q_INVOKABLE void invokableResultTuple() const;
  Q_INVOKABLE ::rust::String invokableResultType() const;
  MyObjectCxxQtThread qtThread() const;
  explicit MyObject(::std::int32_t arg0, QObject* arg1);

private:
  void cppMethodWrapper() const noexcept;
  void invokableWrapper() const noexcept;
  void invokableMutableWrapper() noexcept;
  void invokableParametersWrapper(QColor const& opaque,
                                  QPoint const& trivial,
                                  ::std::int32_t primitive) const noexcept;
  ::std::unique_ptr<Opaque> invokableReturnOpaqueWrapper() noexcept;
  QPoint invokableReturnTrivialWrapper() noexcept;
  void invokableFinalWrapper() const noexcept;
  void invokableOverrideWrapper() const noexcept;
  void invokableVirtualWrapper() const noexcept;
  void invokableResultTupleWrapper() const;
  ::rust::String invokableResultTypeWrapper() const;
  explicit MyObject(
    ::cxx_qt::my_object::cxx_qt_my_object::CxxQtConstructorArguments0&& args);

private:
  ::rust::Box<MyObjectRust> m_rustObj;
  ::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;
  ::std::shared_ptr<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>
    m_cxxQtThreadObj;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
