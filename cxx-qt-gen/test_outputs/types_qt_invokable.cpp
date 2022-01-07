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

QPoint
MyObject::testPoint(const QPoint& point)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return m_rustObj->testPointWrapper(*this, point);
}

QPointF
MyObject::testPointf(const QPointF& pointf)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return m_rustObj->testPointfWrapper(*this, pointf);
}

QRect
MyObject::testRect(const QRect& rect)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return m_rustObj->testRectWrapper(*this, rect);
}

QRectF
MyObject::testRectf(const QRectF& rectf)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return m_rustObj->testRectfWrapper(*this, rectf);
}

QSize
MyObject::testSize(const QSize& size)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return m_rustObj->testSizeWrapper(*this, size);
}

QSizeF
MyObject::testSizef(const QSizeF& sizef)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return m_rustObj->testSizefWrapper(*this, sizef);
}

QString
MyObject::testString(const QString& string)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rustStringToQString(m_rustObj->testStringWrapper(*this, string));
}

QVariant
MyObject::testVariant(const QVariant& variant)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return ::CxxQt::rustVariantToQVariant(
    m_rustObj->testVariantWrapper(*this, variant));
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
