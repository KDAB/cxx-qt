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

private:
  rust::Box<RustObj> m_rustObj;
  bool m_initialised = false;
};

std::unique_ptr<MyObject>
newCppObject();

} // namespace cxx_qt::my_object
