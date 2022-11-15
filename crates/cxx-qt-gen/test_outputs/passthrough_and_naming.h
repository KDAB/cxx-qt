#pragma once

#include <memory>
#include <mutex>

namespace rust::cxxqtlib1 {
template<typename T>
class CxxQtThread;
}

namespace cxx_qt::multi_object {
class MyObject;
using MyObjectCxxQtThread = rust::cxxqtlib1::CxxQtThread<MyObject>;
} // namespace cxx_qt::multi_object

namespace cxx_qt::multi_object {
class SecondObject;
using SecondObjectCxxQtThread = rust::cxxqtlib1::CxxQtThread<SecondObject>;
} // namespace cxx_qt::multi_object

#include "cxx-qt-gen/multi_object.cxx.h"

namespace cxx_qt::multi_object {
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
  const qint32& getPropertyName() const;
  Q_SLOT void setPropertyName(const qint32& value);
  Q_SIGNAL void propertyNameChanged();
  Q_INVOKABLE void invokableName();
  Q_SIGNAL void ready();
  void emitReady();

private:
  rust::Box<MyObjectRust> m_rustObj;
  std::shared_ptr<std::recursive_mutex> m_rustObjMutex;
  std::shared_ptr<rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>
    m_cxxQtThreadObj;
};

static_assert(std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::multi_object

namespace cxx_qt::multi_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject();
} // namespace cxx_qt::multi_object::cxx_qt_my_object

Q_DECLARE_METATYPE(cxx_qt::multi_object::MyObject*)

namespace cxx_qt::multi_object {
class SecondObject : public QObject
{
  Q_OBJECT
  Q_PROPERTY(qint32 propertyName READ getPropertyName WRITE setPropertyName
               NOTIFY propertyNameChanged)

public:
  explicit SecondObject(QObject* parent = nullptr);
  ~SecondObject();
  const SecondObjectRust& unsafeRust() const;
  SecondObjectRust& unsafeRustMut();
  std::unique_ptr<SecondObjectCxxQtThread> qtThread() const;

public:
  const qint32& getPropertyName() const;
  Q_SLOT void setPropertyName(const qint32& value);
  Q_SIGNAL void propertyNameChanged();
  Q_INVOKABLE void invokableName();
  Q_SIGNAL void ready();
  void emitReady();

private:
  rust::Box<SecondObjectRust> m_rustObj;
  std::shared_ptr<std::recursive_mutex> m_rustObjMutex;
  std::shared_ptr<rust::cxxqtlib1::CxxQtGuardedPointer<SecondObject>>
    m_cxxQtThreadObj;
};

static_assert(std::is_base_of<QObject, SecondObject>::value,
              "SecondObject must inherit from QObject");
} // namespace cxx_qt::multi_object

namespace cxx_qt::multi_object::cxx_qt_second_object {
std::unique_ptr<SecondObject>
newCppObject();
} // namespace cxx_qt::multi_object::cxx_qt_second_object

Q_DECLARE_METATYPE(cxx_qt::multi_object::SecondObject*)
