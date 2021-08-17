#pragma once

#include "rust/cxx_qt.h"

namespace cxx_qt::my_object {

class MyObjectRs;

class MyObject : public CxxQObject
{
  Q_OBJECT
  Q_PROPERTY(int number READ getNumber WRITE setNumber NOTIFY numberChanged)
  Q_PROPERTY(QString string READ getString WRITE setString NOTIFY stringChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  int getNumber() const;
  const QString& getString() const;

  Q_INVOKABLE void sayHi(const QString& string, int number);
  Q_INVOKABLE void sayBye();

public Q_SLOTS:
  void setNumber(int value);
  void setString(const QString& value);

Q_SIGNALS:
  void numberChanged();
  void stringChanged();

private:
  rust::Box<MyObjectRs> m_rustObj;

  int m_number;
  QString m_string;
};

std::unique_ptr<MyObject>
newMyObject();

} // namespace cxx_qt::my_object
