#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : CxxQObject(parent)
  , m_rustObj(createRs())
{
  initialiseCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

QPointF
MyObject::testPointf(const QPointF& pointf)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return m_rustObj->testPointf(*this, pointf);
}

QString
MyObject::testString(const QString& string)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rustStringToQString(m_rustObj->testString(*this, string));
}

QVariant
MyObject::testVariant(const QVariant& variant)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return ::CxxQt::rustVariantToQVariant(m_rustObj->testVariant(*this, variant));
}

std::unique_ptr<MyObject>
newCppObject()
{
  return std::make_unique<MyObject>();
}

} // namespace cxx_qt::my_object
