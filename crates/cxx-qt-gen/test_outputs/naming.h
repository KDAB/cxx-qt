#pragma once

#include <memory>
#include <mutex>

namespace rust::cxxqtlib1 {
template<typename T>
class CxxQtThread;
}

class MyObject;
using MyObjectCxxQtThread = rust::cxxqtlib1::CxxQtThread<MyObject>;

#include "cxx-qt-gen/include/my_object.cxx.h"

class MyObject : public QStringListModel
{
  Q_OBJECT
  Q_PROPERTY(qint32 propertyName READ getPropertyName WRITE setPropertyName
               NOTIFY propertyNameChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  const MyObjectRust& unsafeRust() const;
  MyObjectRust& unsafeRustMut();
  std::unique_ptr<MyObjectCxxQtThread> qtThread() const;

public:
  qint32 getPropertyName() const;
  void emitPropertyNameChanged();
  Q_INVOKABLE void invokableName();

public Q_SLOTS:
  void setPropertyName(qint32 value);

Q_SIGNALS:
  void propertyNameChanged();

private:
  rust::Box<MyObjectRust> m_rustObj;
  std::shared_ptr<std::mutex> m_rustObjMutex;
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
