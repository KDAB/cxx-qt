#pragma once

#include "rust/cxx_qt.h"

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public CxxQObject
{
  Q_OBJECT
  Q_PROPERTY(
    qint32 myNumber READ getMyNumber WRITE setMyNumber NOTIFY myNumberChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  qint32 getMyNumber() const;

  Q_INVOKABLE void sayBye();

public Q_SLOTS:
  void setMyNumber(qint32 value);

Q_SIGNALS:
  void myNumberChanged();

private:
  rust::Box<RustObj> m_rustObj;
  bool m_initialised = false;

  qint32 m_myNumber;
};

std::unique_ptr<MyObject>
newMyObject();

} // namespace cxx_qt::my_object
