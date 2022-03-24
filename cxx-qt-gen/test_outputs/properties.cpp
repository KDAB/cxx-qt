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

    runOnGUIThread([&]() { Q_EMIT primitiveChanged(); });
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

    runOnGUIThread([&]() { Q_EMIT opaqueChanged(); });
  }
}

cxx_qt::nested_object::CppObj*
MyObject::getNested() const
{
  return m_nested;
}

void
MyObject::setNested(cxx_qt::nested_object::CppObj* value)
{
  if (value != m_nested) {
    if (m_ownedNested) {
      m_ownedNested.reset();
    }

    m_nested = value;

    runOnGUIThread([&]() { Q_EMIT nestedChanged(); });
  }
}

std::unique_ptr<cxx_qt::nested_object::CppObj>
MyObject::takeNested()
{
  auto value = std::move(m_ownedNested);
  setNested(nullptr);
  return value;
}

void
MyObject::giveNested(std::unique_ptr<cxx_qt::nested_object::CppObj> value)
{
  Q_ASSERT(value.get() != m_nested);

  m_ownedNested = std::move(value);
  m_nested = m_ownedNested.get();

  runOnGUIThread([&]() { Q_EMIT nestedChanged(); });
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
