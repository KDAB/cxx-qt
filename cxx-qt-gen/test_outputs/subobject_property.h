#pragma once

#include "rust/cxx_qt.h"

#include "cxx-qt-gen/include/sub_object.h"

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public CxxQObject
{
  Q_OBJECT
  Q_PROPERTY(cxx_qt::sub_object::SubObject* obj READ getObj WRITE setObj NOTIFY
               objChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  cxx_qt::sub_object::SubObject* getObj() const;
  std::unique_ptr<cxx_qt::sub_object::SubObject> takeObj();
  void giveObj(std::unique_ptr<cxx_qt::sub_object::SubObject> value);

public Q_SLOTS:
  void setObj(cxx_qt::sub_object::SubObject* value);

Q_SIGNALS:
  void objChanged();

private:
  rust::Box<RustObj> m_rustObj;

  cxx_qt::sub_object::SubObject* m_obj = nullptr;
  std::unique_ptr<cxx_qt::sub_object::SubObject> m_ownedObj;
};

std::unique_ptr<MyObject>
newMyObject();

} // namespace cxx_qt::my_object
