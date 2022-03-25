#pragma once

#include <mutex>

#include "cxx-qt-lib/include/qt_types.h"

#include "cxx-qt-gen/include/nested_object.h"
#include <QtGui/QColor>

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public QObject
{
  Q_OBJECT
  Q_PROPERTY(qint32 primitive READ getPrimitive WRITE setPrimitive NOTIFY
               primitiveChanged)
  Q_PROPERTY(QColor opaque READ getOpaque WRITE setOpaque NOTIFY opaqueChanged)
  Q_PROPERTY(cxx_qt::nested_object::CppObj* nested READ getNested WRITE
               setNested NOTIFY nestedChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  qint32 getPrimitive() const;
  const QColor& getOpaque() const;
  cxx_qt::nested_object::CppObj* getNested() const;
  std::unique_ptr<cxx_qt::nested_object::CppObj> takeNested();
  void giveNested(std::unique_ptr<cxx_qt::nested_object::CppObj> value);

public Q_SLOTS:
  void setPrimitive(qint32 value);
  void setOpaque(const QColor& value);
  void setNested(cxx_qt::nested_object::CppObj* value);

Q_SIGNALS:
  void primitiveChanged();
  void opaqueChanged();
  void nestedChanged();

private:
  rust::Box<RustObj> m_rustObj;
  std::mutex m_rustObjMutex;
  bool m_initialised = false;

  qint32 m_primitive;
  QColor m_opaque;
  cxx_qt::nested_object::CppObj* m_nested = nullptr;
  std::unique_ptr<cxx_qt::nested_object::CppObj> m_ownedNested;
};

typedef MyObject CppObj;

std::unique_ptr<CppObj>
newCppObject();

} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::CppObj*)
