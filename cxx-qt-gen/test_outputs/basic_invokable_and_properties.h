#pragma once

#include "rust/cxx_qt.h"

class MyObjectRs;

class MyObject : public QObject
{
  Q_OBJECT
  Q_PROPERTY(int number READ getNumber WRITE setNumber NOTIFY numberChanged)
  Q_PROPERTY(QString string READ getString WRITE setString NOTIFY stringChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  int getNumber() const;
  QString getString() const;

  Q_INVOKABLE void say_hi(const QString& string, int number) const;
  Q_INVOKABLE void say_bye() const;

public Q_SLOTS:
  void setNumber(int value);
  void setString(const QString& value);

Q_SIGNALS:
  void numberChanged();
  void stringChanged();

private:
  rust::Box<MyObjectRs> m_rustObj;
};

std::unique_ptr<MyObject>
newMyObject();
