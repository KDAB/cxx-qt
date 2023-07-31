#pragma once

#include <memory>
#include <mutex>

namespace rust::cxxqtlib1 {
template<typename T>
class CxxQtThread;
}

class MyObject;

#include "cxx-qt-gen/inheritance.cxx.h"

class MyObject : public QAbstractItemModel
{
  Q_OBJECT

public:
  ~MyObject();
  MyObjectRust const& unsafeRust() const;
  MyObjectRust& unsafeRustMut();

public:
  Q_INVOKABLE QVariant data(QModelIndex const& _index,
                            ::std::int32_t _role) const override;
  Q_INVOKABLE bool hasChildren(QModelIndex const& _parent) const override;
  template<class... Args>
  bool hasChildrenCxxQtInherit(Args... args) const
  {
    return QAbstractItemModel::hasChildren(args...);
  }
  template<class... Args>
  void fetchMoreCxxQtInherit(Args... args)
  {
    return QAbstractItemModel::fetchMore(args...);
  }
  explicit MyObject(QObject* parent = nullptr);

private:
  QVariant dataWrapper(QModelIndex const& _index,
                       ::std::int32_t _role) const noexcept;
  bool hasChildrenWrapper(QModelIndex const& _parent) const noexcept;
  [[nodiscard]] ::std::lock_guard<::std::recursive_mutex> unsafeRustLock()
    const;

private:
  ::rust::Box<MyObjectRust> m_rustObj;
  ::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");

Q_DECLARE_METATYPE(MyObject*)
