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
  const MyObjectRust& unsafeRust() const;
  MyObjectRust& unsafeRustMut();

public:
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

static_assert(std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject();
} // namespace cxx_qt::my_object::cxx_qt_my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
