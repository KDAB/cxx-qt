#pragma once

#include <QtQml/QJSValue>
#include <QtQml/QJSValueIterator>

namespace rust
{
    namespace cxxqtlib2
    {

        ::std::unique_ptr<QJSValueIterator> qjsvalueiterator_new(const QJSValue &value);

        ::std::unique_ptr<QJSValue> qjsvalueiterator_value(const QJSValueIterator &value);
    }
}
