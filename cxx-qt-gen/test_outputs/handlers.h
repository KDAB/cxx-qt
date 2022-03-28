#pragma once

#include <mutex>

#include "cxx-qt-lib/include/qt_types.h"

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public QObject
{
  Q_OBJECT
  Q_PROPERTY(qint32 number READ getNumber WRITE setNumber NOTIFY numberChanged)
  Q_PROPERTY(QString string READ getString WRITE setString NOTIFY stringChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

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
  rust::Box<RustObj> m_rustObj;
  std::mutex m_rustObjMutex;
  bool m_initialised = false;

  qint32 m_number;
  QString m_string;
};

typedef MyObject CppObj;

std::unique_ptr<CppObj>
newCppObject();

} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::CppObj*)
