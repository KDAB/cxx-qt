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
  Q_PROPERTY(QColor color READ getColor WRITE setColor NOTIFY colorChanged)
  Q_PROPERTY(QDate date READ getDate WRITE setDate NOTIFY dateChanged)
  Q_PROPERTY(QDateTime dateTime READ getDateTime WRITE setDateTime NOTIFY
               dateTimeChanged)
  Q_PROPERTY(QPoint point READ getPoint WRITE setPoint NOTIFY pointChanged)
  Q_PROPERTY(QPointF pointf READ getPointf WRITE setPointf NOTIFY pointfChanged)
  Q_PROPERTY(QRect rect READ getRect WRITE setRect NOTIFY rectChanged)
  Q_PROPERTY(QRectF rectf READ getRectf WRITE setRectf NOTIFY rectfChanged)
  Q_PROPERTY(QSize size READ getSize WRITE setSize NOTIFY sizeChanged)
  Q_PROPERTY(QSizeF sizef READ getSizef WRITE setSizef NOTIFY sizefChanged)
  Q_PROPERTY(QString string READ getString WRITE setString NOTIFY stringChanged)
  Q_PROPERTY(QTime time READ getTime WRITE setTime NOTIFY timeChanged)
  Q_PROPERTY(QUrl url READ getUrl WRITE setUrl NOTIFY urlChanged)
  Q_PROPERTY(
    QVariant variant READ getVariant WRITE setVariant NOTIFY variantChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  const MyObjectRust& unsafeRust() const;
  MyObjectRust& unsafeRustMut();
  std::unique_ptr<MyObjectCxxQtThread> qtThread() const;

public:
  QColor getColor() const;
  void emitColorChanged();
  QDate getDate() const;
  void emitDateChanged();
  QDateTime getDateTime() const;
  void emitDateTimeChanged();
  QPoint getPoint() const;
  void emitPointChanged();
  QPointF getPointf() const;
  void emitPointfChanged();
  QRect getRect() const;
  void emitRectChanged();
  QRectF getRectf() const;
  void emitRectfChanged();
  QSize getSize() const;
  void emitSizeChanged();
  QSizeF getSizef() const;
  void emitSizefChanged();
  QString getString() const;
  void emitStringChanged();
  QTime getTime() const;
  void emitTimeChanged();
  QUrl getUrl() const;
  void emitUrlChanged();
  QVariant getVariant() const;
  void emitVariantChanged();

public Q_SLOTS:
  void setColor(const QColor& value);
  void setDate(const QDate& value);
  void setDateTime(const QDateTime& value);
  void setPoint(const QPoint& value);
  void setPointf(const QPointF& value);
  void setRect(const QRect& value);
  void setRectf(const QRectF& value);
  void setSize(const QSize& value);
  void setSizef(const QSizeF& value);
  void setString(const QString& value);
  void setTime(const QTime& value);
  void setUrl(const QUrl& value);
  void setVariant(const QVariant& value);

Q_SIGNALS:
  void colorChanged();
  void dateChanged();
  void dateTimeChanged();
  void pointChanged();
  void pointfChanged();
  void rectChanged();
  void rectfChanged();
  void sizeChanged();
  void sizefChanged();
  void stringChanged();
  void timeChanged();
  void urlChanged();
  void variantChanged();

private:
  rust::Box<MyObjectRust> m_rustObj;
  std::shared_ptr<std::mutex> m_rustObjMutex;
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
