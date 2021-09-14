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

  Q_INVOKABLE void sayHi(const QString& string, qint32 number);
  Q_INVOKABLE void sayBye();

private:
  rust::Box<RustObj> m_rustObj;
  bool m_initialised = false;

  void updateState() override;
};

std::unique_ptr<MyObject>
newMyObject();

} // namespace cxx_qt::my_object
