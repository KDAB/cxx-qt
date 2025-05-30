// #include "cxx-qt-lib-extras/qapplication.h"

#include "cxx-qt-lib-extras/qjsengine.h"

namespace rust
{
    namespace cxxqtlib1
    {

        ::std::unique_ptr<QJSEngine> qjsengineNew()
        {
            auto ptr = ::std::make_unique<QJSEngine>();
            Q_ASSERT(ptr != nullptr);

            return ptr;
        }
    }
}
