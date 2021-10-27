#pragma once

#include "rust/cxx_qt.h"

#include <QtCore/QPointF>

namespace cxx_qt::my_object {

class RustObj;

class MyObject : public CxxQObject
{
  Q_OBJECT
  Q_PROPERTY(QPointF pointf READ getPointf WRITE setPointf NOTIFY pointfChanged)
  Q_PROPERTY(QString string READ getString WRITE setString NOTIFY stringChanged)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();

  const QPointF& getPointf() const;
  const QString& getString() const;

public Q_SLOTS:
  void setPointf(const QPointF& value);
  void setString(const QString& value);

Q_SIGNALS:
  void pointfChanged();
  void stringChanged();

private:
  rust::Box<RustObj> m_rustObj;
  bool m_initialised = false;

  QPointF m_pointf;
  QString m_string;
};

std::unique_ptr<MyObject>
newCppObject();

} // namespace cxx_qt::my_object
