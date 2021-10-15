#pragma once

#include <mutex>

#include "rust/cxx_qt.h"

#include "cxx-qt-gen/include/sub_object.h"

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public CxxQObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  Q_INVOKABLE void subTest(cxx_qt::sub_object::SubObject* sub);

private:
  rust::Box<RustObj> m_rustObj;
  std::mutex m_rustObjMutex;
  bool m_initialised = false;
};

std::unique_ptr<MyObject>
newCppObject();

} // namespace cxx_qt::my_object
