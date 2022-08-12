#pragma once

#include <memory>
#include <mutex>

namespace cxx_qt::my_object {
class MyObject;
} // namespace cxx_qt::my_object

#include "cxx-qt-gen/include/my_object.cxx.h"

namespace cxx_qt::my_object {

class MyObject : public QObject
{
  Q_OBJECT
  Q_PROPERTY(qint32 number READ getNumber WRITE setNumber NOTIFY numberChanged)
  Q_PROPERTY(QString string READ getString WRITE setString NOTIFY stringChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  const MyObjectRust& unsafeRust() const;
  MyObjectRust& unsafeRustMut();

public:
  qint32 getNumber() const;
  const QString& getString() const;
  std::unique_ptr<rust::cxxqtlib1::UpdateRequester> updateRequester();
  Q_INVOKABLE void updateState();

public Q_SLOTS:
  void setNumber(qint32 value);
  void setString(const QString& value);

Q_SIGNALS:
  void numberChanged();
  void stringChanged();

private:
  rust::Box<MyObjectRust> m_rustObj;
  std::mutex m_rustObjMutex;
  bool m_initialised = false;

  qint32 m_number;
  QString m_string;
};

static_assert(std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject();
} // namespace cxx_qt::my_object::cxx_qt_my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
