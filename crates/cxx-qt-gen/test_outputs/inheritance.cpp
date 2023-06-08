#include "cxx-qt-gen/inheritance.cxxqt.h"

MyObject::MyObject(QObject* parent)
  : QAbstractItemModel(parent)
  , m_rustObj(cxx_qt_my_object::createRs())
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
{
}

MyObject::~MyObject() {}

MyObjectRust const&
MyObject::unsafeRust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafeRustMut()
{
  return *m_rustObj;
}
