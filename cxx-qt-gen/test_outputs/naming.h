#pragma once

#include <mutex>

#include "cxx-qt-lib/include/qt_types.h"

namespace cxx_qt::my_object {

class MyObjectRust;

class MyObject : public QObject
{
  Q_OBJECT
  Q_PROPERTY(qint32 propertyName READ getPropertyName WRITE setPropertyName
               NOTIFY propertyNameChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

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

typedef MyObject CppObj;

std::unique_ptr<CppObj>
newCppObject();

} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::CppObj*)
