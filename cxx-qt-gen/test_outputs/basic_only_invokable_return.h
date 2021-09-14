#pragma once

#include "rust/cxx_qt.h"

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public CxxQObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  Q_INVOKABLE qint32 doubleNumber(qint32 number);
  Q_INVOKABLE QString helloMessage(const QString& msg);
  Q_INVOKABLE QString staticMessage();

private:
  rust::Box<RustObj> m_rustObj;
  bool m_initialised = false;
};

std::unique_ptr<MyObject>
newMyObject();

} // namespace cxx_qt::my_object
