#include "cxx-qt-gen/include/my_object.h"
#include "cxx-qt-gen/src/my_object.rs.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(createRs())
{
  initialiseCpp(*this);
  m_initialised = true;
}

MyObject::~MyObject() = default;

void
MyObject::invokable()
{
  const std::lock_guard<std::mutex> guard(m_rustObjMutex);
  m_rustObj->invokableWrapper(*this);
}

void
MyObject::emitReady()
{
  const auto signalSuccess = QMetaObject::invokeMethod(
    this, [this]() { Q_EMIT ready(); }, Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

void
MyObject::emitDataChanged(qint32 first,
                          std::unique_ptr<QVariant> second,
                          QPoint third)
{
  const auto signalSuccess = QMetaObject::invokeMethod(
    this,
    [this,
     first = std::move(first),
     second = std::move(second),
     third = std::move(third)]() { Q_EMIT dataChanged(first, *second, third); },
    Qt::QueuedConnection);
  Q_ASSERT(signalSuccess);
}

std::unique_ptr<CppObj>
newCppObject()
{
  return std::make_unique<CppObj>();
}

} // namespace cxx_qt::my_object
