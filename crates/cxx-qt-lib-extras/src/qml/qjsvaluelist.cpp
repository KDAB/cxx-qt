// #include "cxx-qt-lib-extras/qapplication.h"

#include "cxx-qt-lib-extras/qjsvaluelist.h"

namespace rust
{
    namespace cxxqtlib1
    {
        ::std::unique_ptr<QJSValueList> qjsvaluelistNew()
        {
            auto ptr = std::make_unique<QJSValueList>();
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

        ::std::unique_ptr<QJSValueList> qjsvaluelistClone(const QJSValueList &list)
        {
            auto ptr = std::make_unique<QJSValueList>(list);
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }

    }
}
