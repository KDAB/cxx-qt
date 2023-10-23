#include "cxx-qt-gen/inheritance.cxxqt.h"

QVariant
MyObject::data(QModelIndex const& _index, ::std::int32_t _role) const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  return dataWrapper(_index, _role);
}

bool
MyObject::hasChildren(QModelIndex const& _parent) const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  return hasChildrenWrapper(_parent);
}

MyObject::MyObject(QObject* parent)
  : QAbstractItemModel(parent)
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(::cxx_qt_my_object::createRs())
  , ::rust::cxxqtlib1::CxxQtLocking()
{
}

#include "moc_inheritance.cpp"
