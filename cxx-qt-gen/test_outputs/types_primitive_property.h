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
  Q_PROPERTY(
    bool boolean READ getBoolean WRITE setBoolean NOTIFY booleanChanged)
  Q_PROPERTY(
    float float32 READ getFloat32 WRITE setFloat32 NOTIFY float32Changed)
  Q_PROPERTY(
    double float64 READ getFloat64 WRITE setFloat64 NOTIFY float64Changed)
  Q_PROPERTY(qint8 int8 READ getInt8 WRITE setInt8 NOTIFY int8Changed)
  Q_PROPERTY(qint16 int16 READ getInt16 WRITE setInt16 NOTIFY int16Changed)
  Q_PROPERTY(qint32 int32 READ getInt32 WRITE setInt32 NOTIFY int32Changed)
  Q_PROPERTY(quint8 uint8 READ getUint8 WRITE setUint8 NOTIFY uint8Changed)
  Q_PROPERTY(quint16 uint16 READ getUint16 WRITE setUint16 NOTIFY uint16Changed)
  Q_PROPERTY(quint32 uint32 READ getUint32 WRITE setUint32 NOTIFY uint32Changed)

public:
  explicit MyObject(QObject* parent = nullptr);
  ~MyObject();
  const MyObjectRust& unsafeRust() const;
  MyObjectRust& unsafeRustMut();
  std::unique_ptr<MyObjectCxxQtThread> qtThread() const;

public:
  bool getBoolean() const;
  void emitBooleanChanged();
  float getFloat32() const;
  void emitFloat32Changed();
  double getFloat64() const;
  void emitFloat64Changed();
  qint8 getInt8() const;
  void emitInt8Changed();
  qint16 getInt16() const;
  void emitInt16Changed();
  qint32 getInt32() const;
  void emitInt32Changed();
  quint8 getUint8() const;
  void emitUint8Changed();
  quint16 getUint16() const;
  void emitUint16Changed();
  quint32 getUint32() const;
  void emitUint32Changed();

public Q_SLOTS:
  void setBoolean(bool value);
  void setFloat32(float value);
  void setFloat64(double value);
  void setInt8(qint8 value);
  void setInt16(qint16 value);
  void setInt32(qint32 value);
  void setUint8(quint8 value);
  void setUint16(quint16 value);
  void setUint32(quint32 value);

Q_SIGNALS:
  void booleanChanged();
  void float32Changed();
  void float64Changed();
  void int8Changed();
  void int16Changed();
  void int32Changed();
  void uint8Changed();
  void uint16Changed();
  void uint32Changed();

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
