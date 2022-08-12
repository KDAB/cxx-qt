#pragma once

#include <memory>
#include <mutex>

namespace rust::cxxqtlib1 {
template<typename T>
class CxxQtThread;
}

namespace cxx_qt::my_object {
class MyObject;
using MyObjectCxxQtThread = rust::cxxqtlib1::CxxQtThread<MyObject>;
} // namespace cxx_qt::my_object

#include "cxx-qt-gen/include/my_object.cxx.h"

namespace cxx_qt::my_object {
class MyObject : public QObject
{
  Q_OBJECT

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  const MyObjectRust& unsafeRust() const;
  MyObjectRust& unsafeRustMut();
  std::unique_ptr<MyObjectCxxQtThread> qtThread() const;

public:
  Q_INVOKABLE QColor testColor(const QColor& color);
  Q_INVOKABLE QDate testDate(const QDate& date);
  Q_INVOKABLE QDateTime testDateTime(const QDateTime& dateTime);
  Q_INVOKABLE QPoint testPoint(const QPoint& point);
  Q_INVOKABLE QPointF testPointf(const QPointF& pointf);
  Q_INVOKABLE QRect testRect(const QRect& rect);
  Q_INVOKABLE QRectF testRectf(const QRectF& rectf);
  Q_INVOKABLE QSize testSize(const QSize& size);
  Q_INVOKABLE QSizeF testSizef(const QSizeF& sizef);
  Q_INVOKABLE QString testString(const QString& string);
  Q_INVOKABLE QTime testTime(const QTime& time);
  Q_INVOKABLE QUrl testUrl(const QUrl& url);
  Q_INVOKABLE QVariant testVariant(const QVariant& variant);

private:
  rust::Box<MyObjectRust> m_rustObj;
  std::shared_ptr<std::mutex> m_rustObjMutex;
  bool m_initialised = false;
  std::shared_ptr<rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>
    m_cxxQtThreadObj;
};

static_assert(std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
std::unique_ptr<MyObject>
newCppObject();
} // namespace cxx_qt::my_object::cxx_qt_my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
