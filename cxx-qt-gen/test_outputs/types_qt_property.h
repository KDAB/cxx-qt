#pragma once

#include <mutex>

#include "rust/cxx_qt.h"

#include <QtCore/QDate>
#include <QtCore/QPoint>
#include <QtCore/QPointF>
#include <QtCore/QRect>
#include <QtCore/QRectF>
#include <QtCore/QSize>
#include <QtCore/QSizeF>
#include <QtCore/QTime>
#include <QtCore/QUrl>
#include <QtCore/QVariant>
#include <QtGui/QColor>

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public CxxQObject
{
  Q_OBJECT
  Q_PROPERTY(QColor color READ getColor WRITE setColor NOTIFY colorChanged)
  Q_PROPERTY(QDate date READ getDate WRITE setDate NOTIFY dateChanged)
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

  const QColor& getColor() const;
  const QDate& getDate() const;
  const QPoint& getPoint() const;
  const QPointF& getPointf() const;
  const QRect& getRect() const;
  const QRectF& getRectf() const;
  const QSize& getSize() const;
  const QSizeF& getSizef() const;
  const QString& getString() const;
  const QTime& getTime() const;
  const QUrl& getUrl() const;
  const QVariant& getVariant() const;

public Q_SLOTS:
  void setColor(const QColor& value);
  void setDate(const QDate& value);
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
  rust::Box<RustObj> m_rustObj;
  std::mutex m_rustObjMutex;
  bool m_initialised = false;

  QColor m_color;
  QDate m_date;
  QPoint m_point;
  QPointF m_pointf;
  QRect m_rect;
  QRectF m_rectf;
  QSize m_size;
  QSizeF m_sizef;
  QString m_string;
  QTime m_time;
  QUrl m_url;
  QVariant m_variant;
};

typedef MyObject CppObj;

std::unique_ptr<CppObj>
newCppObject();

} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::CppObj*)
