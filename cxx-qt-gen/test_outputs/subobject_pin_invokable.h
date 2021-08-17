#pragma once

#include "rust/cxx_qt.h"

#include "cxx-qt-gen/include/sub_object.h"

namespace cxx_qt::my_object {

class MyObjectRs;

class MyObject : public CxxQObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  Q_INVOKABLE void subTest(cxx_qt::sub_object::SubObject* sub);

private:
  rust::Box<MyObjectRs> m_rustObj;
};

std::unique_ptr<MyObject>
newMyObject();

} // namespace cxx_qt::my_object
