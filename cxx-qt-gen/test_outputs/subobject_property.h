#pragma once

#include "rust/cxx_qt.h"

#include "cxx-qt-gen/include/sub_object.h"

class MyObjectRs;

class MyObject : public QObject
{
  Q_OBJECT
  Q_PROPERTY(SubObject* obj READ getObj WRITE setObj NOTIFY objChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  SubObject* getObj() const;
  std::unique_ptr<SubObject> takeObj();
  void giveObj(std::unique_ptr<SubObject> value);

public Q_SLOTS:
  void setObj(SubObject* value);

Q_SIGNALS:
  void objChanged();

private:
  rust::Box<MyObjectRs> m_rustObj;

  SubObject* m_obj = nullptr;
  std::unique_ptr<SubObject> m_ownedObj;
};

std::unique_ptr<MyObject>
newMyObject();
