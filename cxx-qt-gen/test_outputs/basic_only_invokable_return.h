#pragma once

#include "rust/cxx_qt.h"

class MyObjectRs;

class MyObject : public QObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  Q_INVOKABLE int double_number(int number) const;
  Q_INVOKABLE QString hello_message(const QString& msg) const;
  Q_INVOKABLE QString static_message() const;

private:
  rust::Box<MyObjectRs> m_rustObj;
};

std::unique_ptr<MyObject>
newMyObject();
