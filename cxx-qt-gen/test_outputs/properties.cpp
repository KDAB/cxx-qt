#include "cxx-qt-gen/include/my_object.cxx.h"
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

qint32
MyObject::getPrimitive() const
{
  return m_primitive;
}

void
MyObject::setPrimitive(qint32 value)
{
  if (!m_initialised) {
    m_primitive = value;
    return;
  }

  if (value != m_primitive) {
    m_primitive = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "primitiveChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

const QColor&
MyObject::getOpaque() const
{
  return m_opaque;
}

void
MyObject::setOpaque(const QColor& value)
{
  if (!m_initialised) {
    m_opaque = value;
    return;
  }

  if (value != m_opaque) {
    m_opaque = value;

    const auto signalSuccess =
      QMetaObject::invokeMethod(this, "opaqueChanged", Qt::QueuedConnection);
    Q_ASSERT(signalSuccess);
  }
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
