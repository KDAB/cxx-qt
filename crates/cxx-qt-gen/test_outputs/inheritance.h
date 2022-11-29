#pragma once

#include <memory>
#include <mutex>

namespace rust::cxxqtlib1 {
template<typename T>
class CxxQtThread;
}

class MyObject;
using MyObjectCxxQtThread = rust::cxxqtlib1::CxxQtThread<MyObject>;

#include "cxx-qt-gen/inheritance.cxx.h"

class MyObject : public QAbstractItemModel
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  const MyObjectRust& unsafeRust() const;
  MyObjectRust& unsafeRustMut();
  std::unique_ptr<MyObjectCxxQtThread> qtThread() const;

public:
  Q_INVOKABLE QVariant data(const QModelIndex& _index,
                            qint32 _role) const override;
  Q_INVOKABLE bool hasChildren(const QModelIndex& _parent) const override;

private:
  rust::Box<MyObjectRust> m_rustObj;
  std::shared_ptr<std::recursive_mutex> m_rustObjMutex;
  std::shared_ptr<rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>
    m_cxxQtThreadObj;
};

static_assert(std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");

namespace cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject();
} // namespace cxx_qt_my_object

Q_DECLARE_METATYPE(MyObject*)
