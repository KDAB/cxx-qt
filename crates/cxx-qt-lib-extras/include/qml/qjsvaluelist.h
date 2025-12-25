#pragma once

#include <QtCore/QList>
#include <QtQml/QJSValue>

// Define a proper operator== for QJSValue
#ifndef QJSVALUE_OPERATOR_EQ_DEFINED
#define QJSVALUE_OPERATOR_EQ_DEFINED

inline bool operator==(const QJSValue& lhs, const QJSValue& rhs)
{
    return lhs.strictlyEquals(rhs); // Example using strictlyEquals method
}

#endif // QJSVALUE_OPERATOR_EQ_DEFINED

namespace rust
{
    namespace cxxqtlib2
    {

        class QJSValueList : public QList<QJSValue>
        {
        public:
            QJSValueList() : QList<QJSValue>() {}
            ~QJSValueList() {}
        };

        ::std::unique_ptr<QJSValueList> qjsvaluelistNew();
        ::std::unique_ptr<QJSValueList> qjsvaluelistClone(const QJSValueList &list);
    }
}