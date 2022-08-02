#pragma once

#include <mutex>

#include "cxx-qt-lib/include/qt_types.h"

#include <QtCore/QDate>
#include <QtCore/QDateTime>
#include <QtCore/QPoint>
#include <QtCore/QPointF>
#include <QtCore/QRect>
#include <QtCore/QRectF>
#include <QtCore/QSize>
#include <QtCore/QSizeF>
#include <QtCore/QString>
#include <QtCore/QTime>
#include <QtCore/QUrl>
#include <QtCore/QVariant>
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
  std::mutex m_rustObjMutex;
  bool m_initialised = false;
};

typedef MyObject CppObj;

std::unique_ptr<CppObj>
newCppObject();

} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::CppObj*)
