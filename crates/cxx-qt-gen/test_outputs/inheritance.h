#pragma once

#include <memory>
#include <mutex>

namespace rust::cxxqtlib1 {
template<typename T>
class CxxQtThread;
}

class MyObject;
using MyObjectCxxQtThread = ::rust::cxxqtlib1::CxxQtThread<MyObject>;

#include "cxx-qt-gen/inheritance.cxx.h"

class MyObject : public QAbstractItemModel
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  MyObjectRust const& unsafeRust() const;
  MyObjectRust& unsafeRustMut();
  ::std::unique_ptr<MyObjectCxxQtThread> qtThread() const;

public:
  Q_INVOKABLE QVariant data(QModelIndex const& _index,
                            ::std::int32_t _role) const override;
  Q_INVOKABLE bool hasChildren(QModelIndex const& _parent) const override;
  template<class... Args>
  void fetchMore_cxxqt_inherit(Args... args)
  {
    return QAbstractItemModel::fetchMore(args...);
  }
  template<class... Args>
  bool hasChildren_cxxqt_inherit(Args... args) const
  {
    return QAbstractItemModel::hasChildren(args...);
  }

private:
  ::rust::Box<MyObjectRust> m_rustObj;
  ::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;
  ::std::shared_ptr<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>
    m_cxxQtThreadObj;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");

namespace cxx_qt_my_object {
::std::unique_ptr<MyObject>
newCppObject();
} // namespace cxx_qt_my_object

Q_DECLARE_METATYPE(MyObject*)
