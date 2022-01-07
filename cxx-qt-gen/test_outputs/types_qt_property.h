#pragma once

#include <mutex>

#include "rust/cxx_qt.h"

#include <QtCore/QPoint>
#include <QtCore/QPointF>
#include <QtCore/QSize>
#include <QtCore/QSizeF>
#include <QtCore/QVariant>

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public CxxQObject
{
  Q_OBJECT
  Q_PROPERTY(QPoint point READ getPoint WRITE setPoint NOTIFY pointChanged)
  Q_PROPERTY(QPointF pointf READ getPointf WRITE setPointf NOTIFY pointfChanged)
  Q_PROPERTY(QSize size READ getSize WRITE setSize NOTIFY sizeChanged)
  Q_PROPERTY(QSizeF sizef READ getSizef WRITE setSizef NOTIFY sizefChanged)
  Q_PROPERTY(QString string READ getString WRITE setString NOTIFY stringChanged)
  Q_PROPERTY(
    QVariant variant READ getVariant WRITE setVariant NOTIFY variantChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  const QPoint& getPoint() const;
  const QPointF& getPointf() const;
  const QSize& getSize() const;
  const QSizeF& getSizef() const;
  const QString& getString() const;
  const QVariant& getVariant() const;

public Q_SLOTS:
  void setPoint(const QPoint& value);
  void setPointf(const QPointF& value);
  void setSize(const QSize& value);
  void setSizef(const QSizeF& value);
  void setString(const QString& value);
  void setVariant(const QVariant& value);

Q_SIGNALS:
  void pointChanged();
  void pointfChanged();
  void sizeChanged();
  void sizefChanged();
  void stringChanged();
  void variantChanged();

private:
  rust::Box<RustObj> m_rustObj;
  std::mutex m_rustObjMutex;
  bool m_initialised = false;

  QPoint m_point;
  QPointF m_pointf;
  QSize m_size;
  QSizeF m_sizef;
  QString m_string;
  QVariant m_variant;
};

typedef MyObject CppObj;

std::unique_ptr<CppObj>
newCppObject();

} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::CppObj*)
