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

const QPoint&
MyObject::getPoint() const
{
  return m_point;
}

void
MyObject::setPoint(const QPoint& value)
{
  if (!m_initialised) {
    m_point = value;
    return;
  }

  if (value != m_point) {
    m_point = value;

    runOnGUIThread([&]() { Q_EMIT pointChanged(); });
  }
}

const QPointF&
MyObject::getPointf() const
{
  return m_pointf;
}

void
MyObject::setPointf(const QPointF& value)
{
  if (!m_initialised) {
    m_pointf = value;
    return;
  }

  if (value != m_pointf) {
    m_pointf = value;

    runOnGUIThread([&]() { Q_EMIT pointfChanged(); });
  }
}

const QRectF&
MyObject::getRectf() const
{
  return m_rectf;
}

void
MyObject::setRectf(const QRectF& value)
{
  if (!m_initialised) {
    m_rectf = value;
    return;
  }

  if (value != m_rectf) {
    m_rectf = value;

    runOnGUIThread([&]() { Q_EMIT rectfChanged(); });
  }
}

const QSize&
MyObject::getSize() const
{
  return m_size;
}

void
MyObject::setSize(const QSize& value)
{
  if (!m_initialised) {
    m_size = value;
    return;
  }

  if (value != m_size) {
    m_size = value;

    runOnGUIThread([&]() { Q_EMIT sizeChanged(); });
  }
}

const QSizeF&
MyObject::getSizef() const
{
  return m_sizef;
}

void
MyObject::setSizef(const QSizeF& value)
{
  if (!m_initialised) {
    m_sizef = value;
    return;
  }

  if (value != m_sizef) {
    m_sizef = value;

    runOnGUIThread([&]() { Q_EMIT sizefChanged(); });
  }
}

const QString&
MyObject::getString() const
{
  return m_string;
}

void
MyObject::setString(const QString& value)
{
  if (!m_initialised) {
    m_string = value;
    return;
  }

  if (value != m_string) {
    m_string = value;

    runOnGUIThread([&]() { Q_EMIT stringChanged(); });
  }
}

const QVariant&
MyObject::getVariant() const
{
  return m_variant;
}

void
MyObject::setVariant(const QVariant& value)
{
  if (!m_initialised) {
    m_variant = value;
    return;
  }

  if (value != m_variant) {
    m_variant = value;

    runOnGUIThread([&]() { Q_EMIT variantChanged(); });
  }
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
