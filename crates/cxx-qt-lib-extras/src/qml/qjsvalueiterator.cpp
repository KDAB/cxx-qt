// #include "cxx-qt-lib-extras/qapplication.h"

#include "cxx-qt-lib-extras/qjsvalueiterator.h"

namespace rust
{
    namespace cxxqtlib1
    {

        ::std::unique_ptr<QJSValueIterator> qjsvalueiterator_new(const QJSValue &value)
        {
            auto ptr = std::make_unique<QJSValueIterator>(value);
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        ::std::unique_ptr<QJSValue> qjsvalueiterator_value(const QJSValueIterator &iterator)
        {
            auto ptr = std::make_unique<QJSValue>(iterator.value());
            Q_ASSERT(ptr != nullptr);
            return ptr;
        }
    }
}
