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
  Q_PROPERTY(qint32 primitive READ getPrimitive WRITE setPrimitive NOTIFY
               primitiveChanged)
  Q_PROPERTY(QColor opaque READ getOpaque WRITE setOpaque NOTIFY opaqueChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  const MyObjectRust& unsafe_rust() const;
  MyObjectRust& unsafe_rust_mut();

  qint32 getPrimitive() const;
  const QColor& getOpaque() const;

public Q_SLOTS:
  void setPrimitive(qint32 value);
  void setOpaque(const QColor& value);

Q_SIGNALS:
  void primitiveChanged();
  void opaqueChanged();

private:
  rust::Box<MyObjectRust> m_rustObj;
  std::mutex m_rustObjMutex;
  bool m_initialised = false;

  qint32 m_primitive;
  QColor m_opaque;
};

typedef MyObject CppObj;

std::unique_ptr<CppObj>
newCppObject();

} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::CppObj*)
