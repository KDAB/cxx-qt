// #include "cxx-qt-lib-extras/qapplication.h"

#include "cxx-qt-lib-extras/qjsvalue.h"

namespace rust
{
    namespace cxxqtlib1
    {
        ::std::unique_ptr<QJSValue> qjsvalue_new()
        {
            auto ptr = std::make_unique<QJSValue>(QJSValue::UndefinedValue);
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalue_new_null()
        {
            auto ptr = std::make_unique<QJSValue>(QJSValue::NullValue);
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalue_new_bool(bool value)
        {
            auto ptr = std::make_unique<QJSValue>(value);
            Q_ASSERT(ptr != nullptr);
            return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalue_new_int(int value)
        {
            auto ptr = std::make_unique<QJSValue>(value);
            Q_ASSERT(ptr != nullptr);
            return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalue_new_uint(uint value)
        {
            auto ptr = std::make_unique<QJSValue>(value);
            Q_ASSERT(ptr != nullptr);
            return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalue_new_double(double value)
        {
            auto ptr = std::make_unique<QJSValue>(value);
            Q_ASSERT(ptr != nullptr);
            return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalue_new_qstring(const QString &value)
        {
            auto ptr = std::make_unique<QJSValue>(value);
            Q_ASSERT(ptr != nullptr);
            return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalue_from_jsvalue(const QJSValue &value)
        {
            auto ptr = std::make_unique<QJSValue>(value);
            Q_ASSERT(ptr != nullptr);
            return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalue_get_property(const QString &value)
        {
            auto ptr = std::make_unique<QJSValue>(value);
            Q_ASSERT(ptr != nullptr);
            return ptr;
        }

        QString qjsvalue_to_string(const QJSValue &value)
        {
            return value.toString();
            // auto ptr = std::make_unique<QString>(value.toString());
            // return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalue_property(
            const QJSValue &value,
            const QString &name)
        {
            auto ptr = ::std::make_unique<QJSValue>(value.property(name));
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalue_element(
            const QJSValue &value,
            quint32 index)
        {
            auto ptr = ::std::make_unique<QJSValue>(value.property(index));
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        QVariant qjsvalue_to_qvariant(const QJSValue &value)
        {
            return value.toVariant(QJSValue::RetainJSObjects);
        }

        QObject *qjsvalue_to_qobject(QJSValue &value)
        {
            return value.toQObject();
        }

        bool qvariantCanConvertQJSValue(const QVariant &variant)
        {
            return variant.canConvert<QJSValue *>();
        }

        ::std::unique_ptr<QJSValue> qjsvalueFromQVariant(const QVariant &variant) noexcept
        {
            auto ptr = ::std::make_unique<QJSValue>(variant.value<QJSValue>());
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }
    }
}
