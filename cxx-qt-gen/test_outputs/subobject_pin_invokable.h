#pragma once

#include "rust/cxx_qt.h"

#include "cxx-qt-gen/include/sub_object.h"

class MyObjectRs;

class MyObject : public CxxQObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  Q_INVOKABLE void subTest(SubObject* sub);

private:
  rust::Box<MyObjectRs> m_rustObj;
};

std::unique_ptr<MyObject>
newMyObject();
