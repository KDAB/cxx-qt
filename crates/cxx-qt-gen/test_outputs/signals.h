#pragma once

#include <memory>
#include <mutex>

namespace rust::cxxqtlib1 {
template<typename T>
class CxxQtThread;
}

namespace cxx_qt::my_object {
class MyObject;

} // namespace cxx_qt::my_object

#include "cxx-qt-gen/ffi.cxx.h"

namespace cxx_qt::my_object {
class MyObject : public QObject
{
  Q_OBJECT

public:
  ~MyObject();
  MyObjectRust const& unsafeRust() const;
  MyObjectRust& unsafeRustMut();

public:
  Q_INVOKABLE void invokable();
  Q_SIGNAL void ready();
  ::QMetaObject::Connection readyConnect(::rust::Fn<void(MyObject&)> func,
                                         ::Qt::ConnectionType type);
  Q_SIGNAL void dataChanged(::std::int32_t first,
                            ::std::unique_ptr<Opaque> second,
                            QPoint third,
                            QPoint const& fourth);
  ::QMetaObject::Connection dataChangedConnect(
    ::rust::Fn<void(MyObject&,
                    ::std::int32_t first,
                    ::std::unique_ptr<Opaque> second,
                    QPoint third,
                    QPoint const& fourth)> func,
    ::Qt::ConnectionType type);
  ::QMetaObject::Connection newDataConnect(
    ::rust::Fn<void(MyObject&,
                    ::std::int32_t first,
                    ::std::unique_ptr<Opaque> second,
                    QPoint third,
                    QPoint const& fourth)> func,
    ::Qt::ConnectionType type);
  explicit MyObject(QObject* parent = nullptr);

private:
  ::rust::Box<MyObjectRust> m_rustObj;
  ::std::shared_ptr<::std::recursive_mutex> m_rustObjMutex;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
