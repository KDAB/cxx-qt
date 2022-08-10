#pragma once

#include <memory>
#include <mutex>

namespace cxx_qt::my_object {
class MyObject;
} // namespace cxx_qt::my_object

#include "cxx-qt-gen/include/my_object.cxx.h"

namespace cxx_qt::my_object {

class MyObject : public QObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  const MyObjectRust& unsafe_rust() const;
  MyObjectRust& unsafe_rust_mut();

public:
  Q_INVOKABLE void invokable();
  Q_INVOKABLE void invokableCppObj();
  Q_INVOKABLE void invokableMutable();
  Q_INVOKABLE void invokableMutableCppObj();
  Q_INVOKABLE void invokableParameters(const QColor& opaque,
                                       const QPoint& trivial,
                                       qint32 primitive);
  Q_INVOKABLE void invokableParametersCppObj(qint32 primitive);
  Q_INVOKABLE QColor invokableReturnOpaque();
  Q_INVOKABLE qint32 invokableReturnPrimitive();
  Q_INVOKABLE QString invokableReturnStatic();

private:
  rust::Box<MyObjectRust> m_rustObj;
  std::mutex m_rustObjMutex;
  bool m_initialised = false;
};

typedef MyObject CppObj;

std::unique_ptr<CppObj>
newCppObject();

} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::CppObj*)
