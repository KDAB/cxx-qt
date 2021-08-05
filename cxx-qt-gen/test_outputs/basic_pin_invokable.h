#pragma once

#include "rust/cxx_qt.h"

class MyObjectRs;

class MyObject : public CxxQObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  Q_INVOKABLE void sayHi(const QString& string, int number);
  Q_INVOKABLE void sayBye();

private:
  rust::Box<MyObjectRs> m_rustObj;
};

std::unique_ptr<MyObject>
newMyObject();
