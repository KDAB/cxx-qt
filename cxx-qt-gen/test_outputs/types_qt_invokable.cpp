#include "cxx-qt-gen/include/my_object.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(createRs())
{
  initialiseCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

const MyObjectRust&
MyObject::unsafe_rust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafe_rust_mut()
{
  return *m_rustObj;
}

QColor
MyObject::testColor(const QColor& color)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QColor, std::unique_ptr<QColor>>{}(
    m_rustObj->testColorWrapper(*this, color));
}

QDate
MyObject::testDate(const QDate& date)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QDate, QDate>{}(
    m_rustObj->testDateWrapper(*this, date));
}

QDateTime
MyObject::testDateTime(const QDateTime& dateTime)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QDateTime,
                                         std::unique_ptr<QDateTime>>{}(
    m_rustObj->testDateTimeWrapper(*this, dateTime));
}

QPoint
MyObject::testPoint(const QPoint& point)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QPoint, QPoint>{}(
    m_rustObj->testPointWrapper(*this, point));
}

QPointF
MyObject::testPointf(const QPointF& pointf)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QPointF, QPointF>{}(
    m_rustObj->testPointfWrapper(*this, pointf));
}

QRect
MyObject::testRect(const QRect& rect)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QRect, QRect>{}(
    m_rustObj->testRectWrapper(*this, rect));
}

QRectF
MyObject::testRectf(const QRectF& rectf)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QRectF, QRectF>{}(
    m_rustObj->testRectfWrapper(*this, rectf));
}

QSize
MyObject::testSize(const QSize& size)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QSize, QSize>{}(
    m_rustObj->testSizeWrapper(*this, size));
}

QSizeF
MyObject::testSizef(const QSizeF& sizef)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QSizeF, QSizeF>{}(
    m_rustObj->testSizefWrapper(*this, sizef));
}

QString
MyObject::testString(const QString& string)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QString, std::unique_ptr<QString>>{}(
    m_rustObj->testStringWrapper(*this, string));
}

QTime
MyObject::testTime(const QTime& time)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QTime, QTime>{}(
    m_rustObj->testTimeWrapper(*this, time));
}

QUrl
MyObject::testUrl(const QUrl& url)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QUrl, std::unique_ptr<QUrl>>{}(
    m_rustObj->testUrlWrapper(*this, url));
}

QVariant
MyObject::testVariant(const QVariant& variant)
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  return rust::cxxqtlib1::cxx_qt_convert<QVariant, std::unique_ptr<QVariant>>{}(
    m_rustObj->testVariantWrapper(*this, variant));
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
