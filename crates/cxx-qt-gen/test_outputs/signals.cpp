#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
{
}

MyObject::~MyObject() {}

MyObjectRust const&
MyObject::unsafeRust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafeRustMut()
{
  return *m_rustObj;
}

void
MyObject::invokable()
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableWrapper(*this);
}

void
MyObject::emitReady()
{
  Q_EMIT ready();
}

::QMetaObject::Connection
MyObject::readyConnect(::rust::Fn<void(MyObject&)> func,
                       ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::ready,
    this,
    [&, func = ::std::move(func)]() {
      const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
      func(*this);
    },
    type);
}

void
MyObject::emitDataChanged(::std::int32_t first,
                          ::std::unique_ptr<Opaque> second,
                          QPoint third,
                          QPoint const& fourth)
{
  Q_EMIT dataChanged(::std::move(first),
                     ::std::move(second),
                     ::std::move(third),
                     ::std::move(fourth));
}

::QMetaObject::Connection
MyObject::dataChangedConnect(::rust::Fn<void(MyObject&,
                                             ::std::int32_t first,
                                             ::std::unique_ptr<Opaque> second,
                                             QPoint third,
                                             QPoint const& fourth)> func,
                             ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::dataChanged,
    this,
    [&, func = ::std::move(func)](::std::int32_t first,
                                  ::std::unique_ptr<Opaque> second,
                                  QPoint third,
                                  QPoint const& fourth) {
      const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
      func(*this,
           ::std::move(first),
           ::std::move(second),
           ::std::move(third),
           ::std::move(fourth));
    },
    type);
}

void
MyObject::emitNewData(::std::int32_t first,
                      ::std::unique_ptr<Opaque> second,
                      QPoint third,
                      QPoint const& fourth)
{
  Q_EMIT newData(::std::move(first),
                 ::std::move(second),
                 ::std::move(third),
                 ::std::move(fourth));
}

::QMetaObject::Connection
MyObject::newDataConnect(::rust::Fn<void(MyObject&,
                                         ::std::int32_t first,
                                         ::std::unique_ptr<Opaque> second,
                                         QPoint third,
                                         QPoint const& fourth)> func,
                         ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::newData,
    this,
    [&, func = ::std::move(func)](::std::int32_t first,
                                  ::std::unique_ptr<Opaque> second,
                                  QPoint third,
                                  QPoint const& fourth) {
      const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
      func(*this,
           ::std::move(first),
           ::std::move(second),
           ::std::move(third),
           ::std::move(fourth));
    },
    type);
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
::std::unique_ptr<MyObject>
newCppObject()
{
  return ::std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object
