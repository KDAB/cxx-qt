#pragma once

#include <mutex>

#include "cxx-qt-lib/include/qt_types.h"

#include "cxx-qt-gen/include/nested_object.cxxqt.h"
#include <QtCore/QPoint>
#include <QtGui/QColor>

namespace cxx_qt::my_object {

class MyObjectRust;

class MyObject : public QObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  const MyObjectRust& unsafe_rust() const;
  MyObjectRust& unsafe_rust_mut();

  Q_INVOKABLE void invokable();
  Q_INVOKABLE void invokableCppObj();
  Q_INVOKABLE void invokableMutable();
  Q_INVOKABLE void invokableMutableCppObj();
  Q_INVOKABLE void invokableNestedParameter(
    cxx_qt::nested_object::CppObj* nested);
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
