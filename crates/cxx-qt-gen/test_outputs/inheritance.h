#pragma once

#include <cxx-qt/type.h>

class MyObject;

#include "directory/file_ident.cxx.h"

class MyObject
  : public QAbstractItemModel
  , public ::rust::cxxqt1::CxxQtType<MyObjectRust>
{
  Q_OBJECT
public:
  virtual ~MyObject() = default;

public:
  Q_INVOKABLE QVariant data(QModelIndex const& _index,
                            ::std::int32_t _role) const noexcept override;
  Q_INVOKABLE bool hasChildren(
    QModelIndex const& _parent) const noexcept override;
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
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");

Q_DECLARE_METATYPE(MyObject*)
