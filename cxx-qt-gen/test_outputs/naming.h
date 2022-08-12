#pragma once

#include <memory>
#include <mutex>

class MyObject;

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

public:
  qint32 getPropertyName() const;
  Q_INVOKABLE void invokableName();

public Q_SLOTS:
  void setPropertyName(qint32 value);

Q_SIGNALS:
  void propertyNameChanged();

private:
  rust::Box<MyObjectRust> m_rustObj;
  std::mutex m_rustObjMutex;
  bool m_initialised = false;

  qint32 m_propertyName;
};

static_assert(std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");

namespace cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject();
} // namespace cxx_qt_my_object

Q_DECLARE_METATYPE(MyObject*)
