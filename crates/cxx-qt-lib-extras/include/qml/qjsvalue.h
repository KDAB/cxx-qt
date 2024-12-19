#pragma once

#include <QtQml/QJSValue>
#include <QtCore/QVariant>

namespace rust
{
    namespace cxxqtlib2
    {

        ::std::unique_ptr<QJSValue> qjsvalue_new();
        ::std::unique_ptr<QJSValue> qjsvalue_new_null();
        ::std::unique_ptr<QJSValue> qjsvalue_new_bool(bool value);
        ::std::unique_ptr<QJSValue> qjsvalue_new_int(int value);
        ::std::unique_ptr<QJSValue> qjsvalue_new_uint(uint value);
        ::std::unique_ptr<QJSValue> qjsvalue_new_double(double value);
        ::std::unique_ptr<QJSValue> qjsvalue_new_qstring(const QString &value);

        ::std::unique_ptr<QJSValue> qjsvalue_from_jsvalue(const QJSValue &value);

        QString qjsvalue_to_string(const QJSValue &value);

        ::std::unique_ptr<QJSValue> qjsvalue_property(
            const QJSValue &value,
            const QString &name);
        ::std::unique_ptr<QJSValue> qjsvalue_element(
            const QJSValue &value,
            quint32 index);

        QVariant qjsvalue_to_qvariant(const QJSValue &value);
        QObject *qjsvalue_to_qobject(QJSValue &value);

        bool qvariantCanConvertQJSValue(const QVariant &variant);
        ::std::unique_ptr<QJSValue> qjsvalueFromQVariant(const QVariant &variant) noexcept;
    }
}
