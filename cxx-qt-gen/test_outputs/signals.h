#pragma once

#include <mutex>

#include "qt_types.h"

#include <QtCore/QPoint>
#include <QtCore/QVariant>

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public QObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  Q_INVOKABLE void invokable();

  void emitReady();
  void emitDataChanged(qint32 first,
                       std::unique_ptr<QVariant> second,
                       QPoint third);

Q_SIGNALS:
  void ready();
  void dataChanged(qint32 first, const QVariant& second, const QPoint& third);

private:
  rust::Box<RustObj> m_rustObj;
  std::mutex m_rustObjMutex;
  bool m_initialised = false;
};

typedef MyObject CppObj;

std::unique_ptr<CppObj>
newCppObject();

} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::CppObj*)
